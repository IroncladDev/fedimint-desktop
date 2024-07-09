use dioxus::prelude::*;

use crate::components::ui::*;

#[component]
pub fn Empty(add_federation_dialog: Signal<bool>) -> Element {
    rsx! {
        Flex { 
            Button { onclick: move |_| add_federation_dialog.set(true), "Add a Federation" }
        }
    }
}
