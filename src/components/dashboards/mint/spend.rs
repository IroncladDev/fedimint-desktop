use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Spend() -> Element {
    rsx! {
        Widget { "spend notes" }
    }
}
