use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Validate() -> Element {
    rsx! {
        Widget { "validate" }
    }
}
