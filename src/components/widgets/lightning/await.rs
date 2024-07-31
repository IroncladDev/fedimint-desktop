use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Await() -> Element {
    rsx! {
        Widget { title: "Await Invoice", "e" }
    }
}
