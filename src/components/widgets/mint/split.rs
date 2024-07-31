use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Split() -> Element {
    rsx! {
        Widget { title: "Split Notes", "e" }
    }
}
