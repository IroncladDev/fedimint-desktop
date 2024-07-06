use crate::components::widget::Widget;
use dioxus::prelude::*;

#[component]
pub fn Spend() -> Element {
    rsx! {
        Widget { 
            h2 { class: "text-xl font-bold", "Spend Notes" }
        }
    }
}
