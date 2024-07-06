use dioxus::prelude::*;

use crate::components::ui::Button;

#[component]
pub fn Empty() -> Element {
    let mut add_federation_modal = use_context::<Signal<bool>>();

    let toggle_modal = move |_| {
        *add_federation_modal.write() = true;
    };

    rsx! {
        div {
            Button { onclick: toggle_modal, "click {add_federation_modal.read()}" }
        }
    }
}
