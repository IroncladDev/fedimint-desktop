pub mod r#await;
pub mod gateways;
pub mod invoice;
pub mod pay;

use crate::components::grid::Grid;
use dioxus::prelude::*;
use gateways::Gateways;
use invoice::Invoice;
use pay::Pay;
use r#await::Await;

#[component]
pub fn LightningDashboard() -> Element {
    rsx! {
        Grid { 
            Gateways {}
            Invoice {}
            Await {}
            Pay {}
        }
    }
}
