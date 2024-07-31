pub mod r#await;
pub mod create_address;

use crate::components::grid::Grid;
use create_address::CreateAddress;
use dioxus::prelude::*;
use r#await::Await;

#[component]
pub fn OnchainDashboard() -> Element {
    rsx! {
        Grid { 
            CreateAddress {}
            Await {}
        }
    }
}
