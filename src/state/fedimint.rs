#![allow(dead_code)]
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::PathBuf,
    sync::Arc,
};

use multimint::{
    fedimint_client::ClientHandle,
    fedimint_core::{api::InviteCode, config::FederationId},
    fedimint_mint_client::MintClientModule,
    fedimint_wallet_client::WalletClientModule,
    types::InfoResponse,
    MultiMint,
};
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Fedimint {
    pub multimint: MultiMint,
    pub active_federation_id: Option<FederationId>,
    pub federations: BTreeMap<FederationId, InfoResponse>,
}

impl PartialEq for Fedimint {
    fn eq(&self, other: &Self) -> bool {
        self.active_federation_id == other.active_federation_id
            && self.federations == other.federations
    }
}

type MetaJson = BTreeMap<String, String>;
type MetaTree = BTreeMap<String, MetaJson>;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum MetaResponse {
    Json(MetaJson),
    Tree(MetaTree),
}

impl Fedimint {
    pub async fn new() -> Result<Self, String> {
        let multimint = Self::init_multimint().await?;
        let federations = Self::load_federation_info(&multimint).await?;
        let first_federation_id = federations.first_key_value().map(|(k, _)| *k);

        let mut multimint_client = Fedimint {
            multimint,
            federations,
            active_federation_id: first_federation_id,
        };

        multimint_client.reload_meta().await
    }

    fn get_fm_db_path() -> Result<PathBuf, String> {
        let mut fm_db_path = dirs::data_local_dir().unwrap();
        fm_db_path.push("fedimint-desktop");
        fm_db_path.push("fm_db");

        if !fm_db_path.exists() {
            if let Err(e) = fs::create_dir_all(&fm_db_path) {
                return Err(format!("Error creating directory: {}", e));
            }
        }

        Ok(fm_db_path)
    }

    async fn init_multimint() -> Result<MultiMint, String> {
        let fm_db_path = Self::get_fm_db_path()?;
        let clients = MultiMint::new(fm_db_path.clone())
            .await
            .map_err(|e| format!("Error getting clients {}", e))?;
        clients
            .update_gateway_caches()
            .await
            .map_err(|e| format!("Error getting clients {}", e))?;

        Ok(clients)
    }

    async fn load_federation_info(
        multimint: &MultiMint,
    ) -> Result<BTreeMap<FederationId, InfoResponse>, String> {
        let mut info = BTreeMap::new();

        for (id, client) in multimint.clients.lock().await.iter() {
            let mint_client = client.get_first_module::<MintClientModule>();
            let wallet_client = client.get_first_module::<WalletClientModule>();
            let summary = mint_client
                .get_wallet_summary(
                    &mut client
                        .db()
                        .begin_transaction_nc()
                        .await
                        .to_ref_with_prefix_module_id(1),
                )
                .await;

            info.insert(
                *id,
                InfoResponse {
                    network: wallet_client.get_network().to_string(),
                    meta: client.get_config().global.meta.clone(),
                    total_amount_msat: summary.total_amount(),
                    total_num_notes: summary.count_items(),
                    denominations_msat: summary,
                    federation_id: *id,
                },
            );
        }

        Ok(info)
    }

    async fn reload_meta(&mut self) -> Result<Self, String> {
        let mut meta_map: HashMap<String, MetaResponse> = HashMap::new();

        let mut updates = Vec::new();

        for (federation_id, info) in self.federations.iter() {
            updates.push((federation_id.clone().to_owned(), info.clone()));
        }

        for (federation_id, info) in updates {
            let meta_external_url = info.meta.get("meta_external_url");

            if let Some(meta_url) = meta_external_url {
                if !meta_url.is_empty() && !meta_map.contains_key(meta_url) {
                    let meta = Self::fetch_meta(meta_url).await;

                    if let Ok(meta) = meta {
                        meta_map.insert(meta_url.to_string(), meta);
                    }
                }

                if let Some(meta) = meta_map.get(meta_url) {
                    match meta {
                        MetaResponse::Json(meta_json) => {
                            let meta_json = meta_json.clone();

                            self.federations.get_mut(&federation_id).unwrap().meta = meta_json;
                        }
                        MetaResponse::Tree(meta_tree) => {
                            let federation_id_string = federation_id.to_string();
                            let meta_item = meta_tree.get(&federation_id_string);

                            if let Some(meta) = meta_item {
                                let meta_json = meta.clone();

                                self.federations.get_mut(&federation_id).unwrap().meta = meta_json;
                            }
                        }
                    }
                }
            }
        }

        Ok(self.clone())
    }

    async fn fetch_meta(url: &str) -> Result<MetaResponse, reqwest::Error> {
        let response = reqwest::get(url).await?;
        let meta: MetaResponse = response.json().await?;
        Ok(meta)
    }

    pub fn get_active_federation(&self) -> Option<InfoResponse> {
        self.federations
            .get(&self.active_federation_id.unwrap())
            .cloned()
    }

    pub async fn get_multimint_client(&self) -> Option<Arc<ClientHandle>> {
        let federation_id = self.active_federation_id.unwrap();

        self.multimint.get(&federation_id).await
    }

    pub async fn remove_federation(&mut self, federation_id: FederationId) {
        self.multimint.remove(&federation_id).await;
        self.federations.remove(&federation_id);

        if federation_id == self.active_federation_id.unwrap() {
            self.active_federation_id = None;
        }
    }

    pub async fn add_federation(&mut self, invite_code: InviteCode) -> Result<(), String> {
        self.multimint
            .register_new(invite_code, None)
            .await
            .map_err(|e| e.to_string())?;
        let federations = Self::load_federation_info(&self.multimint).await?;

        self.federations = federations;
        self.reload_meta().await.unwrap();

        Ok(())
    }

    pub async fn reload_active_federation(&mut self) {
        let client = self.get_multimint_client().await.unwrap();

        let mint_client = client.get_first_module::<MintClientModule>();
        let summary = mint_client
            .get_wallet_summary(
                &mut client
                    .db()
                    .begin_transaction_nc()
                    .await
                    .to_ref_with_prefix_module_id(1),
            )
            .await;

        let active_federation = self
            .federations
            .get_mut(&self.active_federation_id.unwrap())
            .unwrap();

        active_federation.total_amount_msat = summary.total_amount();
        active_federation.total_num_notes = summary.count_items();
        active_federation.denominations_msat = summary;
    }
}
