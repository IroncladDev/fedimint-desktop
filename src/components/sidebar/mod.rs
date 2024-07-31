use add_federation_dialog::AddFederationDialog;
use dioxus::prelude::*;
use federation_item::FederationItem;

use crate::{components::ui::*, util::state::AppState};
mod add_federation_dialog;
mod empty;
mod federation_item;

use empty::Empty;

#[component]
pub fn Sidebar() -> Element {
    let state = use_context::<Signal<AppState>>();
    let mut add_federation_dialog = use_signal::<bool>(|| false);

    let federations = state().federations.clone();

    let visibility = if state().sidebar_open {
        "visible"
    } else {
        "hidden"
    };

    rsx! {
        Flex {
            col: true,
            gap: 2,
            p: 2,
            class: "min-w-[300px] border-r {visibility}",
            if federations.is_empty() {
                Empty { add_federation_dialog }
            } else {
                Flex { col: true, grow: true,
                    Flex { col: true, gap: 2, class: "inset-0 overflow-y-auto",
                        for (_ , info) in federations {
                            FederationItem { info: info.clone() }
                        }
                    }
                }
                Flex { 
                    Button { class: "w-full", onclick: move |_| add_federation_dialog.set(true), "Add a Federation" }
                }
            }
        }
        AddFederationDialog { add_federation_dialog }
    }
}
