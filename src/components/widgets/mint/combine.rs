use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Combine() -> Element {
    rsx! {
        Widget { title: "Combine Notes", "e" }
    }
}
