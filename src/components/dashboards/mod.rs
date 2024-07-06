pub mod admin;
pub mod lightning;
pub mod mint;
pub mod onchain;

use admin::AdminDashboard;
use dioxus::prelude::*;
use lightning::LightningDashboard;
use mint::MintDashboard;
use onchain::OnchainDashboard;

use crate::util::{state::AppState, types::Tab};

#[component]
pub fn Dashboards() -> Element {
    let state = use_context::<Signal<AppState>>();

    rsx! {
        match state.read().tab {
            Tab::Admin => rsx! {AdminDashboard {}},
            Tab::Lightning => rsx! {LightningDashboard {}},
            Tab::Mint => rsx! {MintDashboard {}},
            Tab::Onchain => rsx! {OnchainDashboard {}},
        }
    }
}
