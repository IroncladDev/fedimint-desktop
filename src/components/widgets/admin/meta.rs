use dioxus::prelude::*;

use crate::components::widget::Widget;

#[component]
pub fn Meta() -> Element {
    rsx! {
        Widget { title: "Meta", "e" }
    }
}
