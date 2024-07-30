use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Invoice() -> Element {
    rsx! {
        Widget { "create invoice" }
    }
}
