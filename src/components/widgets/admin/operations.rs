use dioxus::prelude::*;

use crate::components::widget::Widget;

#[component]
pub fn Operations() -> Element {
    rsx! {
        Widget { title: "Operations", "e" }
    }
}
