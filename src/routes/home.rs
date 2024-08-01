use std::collections::{BTreeMap, HashMap};

use crate::components::sidebar::Sidebar;
use crate::components::tabs::Tabs;
use crate::components::widgets::Widgets;
use crate::util::state::AppState;
use dioxus::prelude::*;
use multimint::fedimint_mint_client::MintClientModule;
use multimint::fedimint_wallet_client::WalletClientModule;
use multimint::types::InfoResponse;
use serde::Deserialize;

type MetaJson = BTreeMap<String, String>;
type MetaTree = BTreeMap<String, MetaJson>;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum MetaResponse {
    Json(MetaJson),
    Tree(MetaTree),
}

async fn fetch_meta(url: &str) -> Result<MetaResponse, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let meta: MetaResponse = response.json().await?;
    Ok(meta)
}

#[component]
pub fn Home() -> Element {
    let mut state = use_context::<Signal<AppState>>();

    let active_federation: Memo<Option<InfoResponse>> = use_memo(move || {
        state()
            .active_federation_id
            .and_then(|id| state().federations.get(&id).cloned())
    });

    use_effect(move || {
        spawn(async move {
            let mut info = BTreeMap::new();

            for (id, client) in state().multimint.clients.lock().await.iter() {
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

            state.write().federations = info.clone();
            if let Some((federation_id, _)) = info.first_key_value() {
                state.write().active_federation_id = Some(*federation_id);
            }
        });
    });

    use_effect(move || {
        spawn(async move {
            let mut meta_map: HashMap<String, MetaResponse> = HashMap::new();

            let mut updates = Vec::new();

            for (federation_id, info) in state().federations.iter() {
                updates.push((federation_id.clone().to_owned(), info.clone()));
            }

            for (federation_id, info) in updates {
                let meta_external_url = info.meta.get("meta_external_url");

                if let Some(meta_url) = meta_external_url {
                    if !meta_url.is_empty() && !meta_map.contains_key(meta_url) {
                        let meta = fetch_meta(meta_url).await;

                        if let Ok(meta) = meta {
                            meta_map.insert(meta_url.to_string(), meta);
                        }
                    }

                    if let Some(meta) = meta_map.get(meta_url) {
                        match meta {
                            MetaResponse::Json(meta_json) => {
                                let meta_json = meta_json.clone();

                                state
                                    .write()
                                    .federations
                                    .get_mut(&federation_id)
                                    .unwrap()
                                    .meta = meta_json;
                            }
                            MetaResponse::Tree(meta_tree) => {
                                let federation_id_string = federation_id.to_string();
                                let meta_item = meta_tree.get(&federation_id_string);

                                if let Some(meta) = meta_item {
                                    let meta_json = meta.clone();

                                    state
                                        .write()
                                        .federations
                                        .get_mut(&federation_id)
                                        .unwrap()
                                        .meta = meta_json;
                                }
                            }
                        }
                    }
                }
            }
        });
    });

    use_context_provider(|| active_federation);

    rsx! {
        Sidebar {}
        div { class: "flex flex-col grow",
            Tabs {}
            div { class: "grow relative",
                div { class: "absolute inset-0 overflow-y-auto flex flex-col",
                    if let Some(_) = active_federation() {
                        Widgets {}
                    } else {
                        "Loading"
                    }
                }
            }
        }
    }
}
