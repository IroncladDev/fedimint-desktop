use dioxus::prelude::*;

#[component]
pub fn NoFederation() -> Element {
    rsx! {
        div { "Please choose a federation" }
    }
}
