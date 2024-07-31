use dioxus::prelude::*;

use crate::components::widget::Widget;

#[component]
pub fn Denominations() -> Element {
    rsx! {
        Widget { title: "Denominations", "e" }
    }
}
