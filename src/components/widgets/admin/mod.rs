pub mod denominations;
pub mod info;
pub mod meta;
pub mod operations;

use denominations::Denominations;
use dioxus::prelude::*;
use info::Info;
use meta::Meta;
use operations::Operations;

use crate::components::grid::Grid;

#[component]
pub fn AdminDashboard() -> Element {
    rsx! {
        Grid { 
            Info {}
            Meta {}
            Operations {}
            Denominations {}
        }
    }
}
