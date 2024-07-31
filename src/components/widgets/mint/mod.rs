pub mod combine;
pub mod decode;
pub mod encode;
pub mod reissue;
pub mod spend;
pub mod split;
pub mod validate;

use combine::Combine;
use decode::Decode;
use dioxus::prelude::*;
use encode::Encode;
use reissue::Reissue;
use spend::Spend;
use split::Split;
use validate::Validate;

use crate::components::grid::Grid;

#[component]
pub fn MintDashboard() -> Element {
    rsx! {
        Grid { 
            Spend {}
            Reissue {}
            Validate {}
            Encode {}
            Decode {}
            Split {}
            Combine {}
        }
    }
}
