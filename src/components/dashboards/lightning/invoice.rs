use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Invoice() -> Element {
    rsx! {
        Widget { 
            h2 { class: "text-xl font-bold", "Create Invoice" }
        }
    }
}
