pub mod reissue;
pub mod spend;
pub mod validate;

use dioxus::prelude::*;
use reissue::Reissue;
use spend::Spend;
use validate::Validate;

use crate::components::grid::Grid;

#[component]
pub fn MintDashboard() -> Element {
    rsx! {
        Grid { 
            Spend {}
            Reissue {}
            Validate {}
        }
    }
}
