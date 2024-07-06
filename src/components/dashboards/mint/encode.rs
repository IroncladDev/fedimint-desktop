use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Encode() -> Element {
    rsx! {
        Widget { "encode" }
    }
}
