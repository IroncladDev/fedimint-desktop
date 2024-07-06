use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn CreateAddress() -> Element {
    rsx! {
        Widget { "create address" }
    }
}
