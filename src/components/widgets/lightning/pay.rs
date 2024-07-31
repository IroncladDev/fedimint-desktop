use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Pay() -> Element {
    rsx! {
        Widget { title: "Pay Invoice", "e" }
    }
}
