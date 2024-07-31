pub mod r#await;
pub mod claim_tweaks;
pub mod gateways;
pub mod invoice;
pub mod pay;
pub mod tweak_invoice;

use crate::components::grid::Grid;
use claim_tweaks::ClaimTweaks;
use dioxus::prelude::*;
use gateways::Gateways;
use invoice::Invoice;
use pay::Pay;
use r#await::Await;
use tweak_invoice::TweakInvoice;

#[component]
pub fn LightningDashboard() -> Element {
    rsx! {
        Grid { 
            Gateways {}
            Invoice {}
            Await {}
            Pay {}
            TweakInvoice {}
            ClaimTweaks {}
        }
    }
}
