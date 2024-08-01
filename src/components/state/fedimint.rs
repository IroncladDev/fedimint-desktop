#![allow(dead_code)]
use std::{collections::BTreeMap, fs};

use multimint::{fedimint_core::config::FederationId, types::InfoResponse, MultiMint};

pub struct Fedimint {
    pub multimint: MultiMint,
    pub active_federation_id: Option<FederationId>,
    pub federations: BTreeMap<FederationId, InfoResponse>,
}

impl Fedimint {
    pub async fn new() -> Result<Self, String> {
        let mut fm_db_path = dirs::data_local_dir().unwrap();
        fm_db_path.push("fedimint-desktop");
        fm_db_path.push("fm_db");

        if !fm_db_path.exists() {
            if let Err(e) = fs::create_dir_all(&fm_db_path) {
                return Err(format!("Error creating directory: {}", e));
            }
        }

        let clients = MultiMint::new(fm_db_path.clone())
            .await
            .map_err(|e| format!("Error getting clients {}", e))?;
        clients
            .update_gateway_caches()
            .await
            .map_err(|e| format!("Error getting clients {}", e))?;

        let federation_ids = clients.ids().await.into_iter().collect::<Vec<_>>();

        Ok(Fedimint {
            multimint: clients,
            active_federation_id: if federation_ids.is_empty() {
                None
            } else {
                Some(federation_ids[0])
            },
            federations: BTreeMap::new(),
        })
    }
}
