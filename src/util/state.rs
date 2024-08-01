#![allow(dead_code)]
use std::{collections::BTreeMap, fs};

use super::types::*;
use anyhow::Result;
use multimint::{fedimint_core::config::FederationId, types::InfoResponse, MultiMint};

#[derive(Clone, Debug)]
pub struct AppState {
    pub fm_db_path: String,
    pub multimint: MultiMint,
    pub tab: Tab,
    pub toast: Toast,
    pub active_federation_id: Option<FederationId>,
    pub federations: BTreeMap<FederationId, InfoResponse>,
    pub sidebar_open: bool,
    pub theme: Theme,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        self.fm_db_path == other.fm_db_path
            && self.tab == other.tab
            && self.toast == other.toast
            && self.active_federation_id == other.active_federation_id
            && self.sidebar_open == other.sidebar_open
            && self.theme == other.theme
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
}
