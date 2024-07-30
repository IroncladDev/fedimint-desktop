use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Gateways() -> Element {
    rsx! {
        Widget { "gateways" }
    }
}
