use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Validate() -> Element {
    rsx! {
        Widget { title: "Validate Notes", "e" }
    }
}
