use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Gateways() -> Element {
    rsx! {
        Widget { 
            h2 { class: "text-xl font-bold", "Gateways" }
        }
    }
}
