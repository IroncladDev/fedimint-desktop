use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Reissue() -> Element {
    rsx! {
        Widget { title: "Reissue Notes", "e" }
    }
}
