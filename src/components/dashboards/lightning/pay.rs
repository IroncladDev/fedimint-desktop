use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Pay() -> Element {
    rsx! {
        Widget { "pay" }
    }
}
