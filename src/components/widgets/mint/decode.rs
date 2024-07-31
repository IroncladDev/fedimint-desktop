use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Decode() -> Element {
    rsx! {
        Widget { title: "Decode Notes", "e" }
    }
}
