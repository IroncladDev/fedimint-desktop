use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Reissue() -> Element {
    rsx! {
        Widget { 
            h2 { class: "text-xl font-bold", "Spend Notes" }
        }
    }
}
