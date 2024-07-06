pub mod info;
pub mod operations;

use dioxus::prelude::*;
use info::Info;
use operations::Operations;

#[component]
pub fn AdminDashboard() -> Element {
    rsx! {
        div { class: "flex grow divide-x divide-slate-800",
            Info {}
            Operations {}
        }
    }
}
