#![allow(dead_code)]
use std::{collections::BTreeMap, fs};

use super::types::*;
use anyhow::Result;
use multimint::{
    fedimint_client::ClientHandleArc, fedimint_core::config::FederationId, types::InfoResponse,
    MultiMint,
};

#[derive(Clone, Debug)]
pub struct AppState {
    // Multimint stuff
    pub fm_db_path: String,
    pub multimint: MultiMint,

    // Sidebar
    pub active_federation_id: Option<FederationId>,
    pub federations: BTreeMap<FederationId, InfoResponse>,
    pub sidebar_open: bool,

    // UI
    pub tab: Tab,
    pub toast: Toast,
    pub theme: Theme,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        self.fm_db_path == other.fm_db_path && self.tab == other.tab && self.toast == other.toast
    }
}

impl AppState {
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

        Ok(AppState {
            fm_db_path: fm_db_path.to_str().unwrap().to_string(),
            multimint: clients,
            tab: Tab::Admin,
            toast: Toast::new(),
            active_federation_id: if federation_ids.is_empty() {
                None
            } else {
                Some(federation_ids[0])
            },
            federations: BTreeMap::new(),
            theme: Theme::Dark,
            sidebar_open: true,
        })
    }

    pub fn switch_tab(&mut self, tab: Tab) {
        self.tab = tab;
    }

    pub async fn get_federation_ids(&self) -> Vec<FederationId> {
        self.multimint.ids().await.into_iter().collect::<Vec<_>>()
    }

    pub async fn get_federation_info(&self) -> Result<BTreeMap<FederationId, InfoResponse>> {
        self.multimint.info().await
    }

    pub async fn get_client_by_federation_id(
        &self,
        federation_id: FederationId,
    ) -> Result<ClientHandleArc, String> {
        match self.multimint.get(&federation_id).await {
            Some(client) => Ok(client),
            None => Err("No client found for federation id".to_string()),
        }
    }

    pub async fn get_client(&self) -> Result<ClientHandleArc, String> {
        match self.active_federation_id {
            Some(id) => self.get_client_by_federation_id(id).await,
            None => Err("Active Federation ID Not Set".to_string()),
        }
    }
}
