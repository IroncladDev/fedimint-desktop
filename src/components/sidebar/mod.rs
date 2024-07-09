use std::collections::BTreeMap;

use add_federation_dialog::AddFederationDialog;
use dioxus::prelude::*;
use federation_item::FederationItem;
use multimint::{fedimint_core::config::FederationId, types::InfoResponse};

use crate::{components::ui::*, util::state::AppState};
mod add_federation_dialog;
mod empty;
mod federation_item;

use empty::Empty;

#[component]
pub fn Sidebar() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut federations = use_signal::<BTreeMap<FederationId, InfoResponse>>(BTreeMap::new);
    let mut add_federation_dialog = use_signal::<bool>(|| false);

    use_effect(move || {
        spawn(async move {
            let federation_info = state.read().multimint.info().await;

            if let Ok(info) = federation_info {
                federations.set(info.clone());

                if let Some((federation_id, _)) = info.first_key_value() {
                    state.write().active_federation_id = Some(*federation_id);
                }
            }
        });
    });

    rsx! {
        Flex { col: true, gap: 2, p: 2, class: "min-w-[300px]",
            if federations.read().is_empty() {
                Empty { add_federation_dialog }
            } else {
                Flex { col: true, gap: 2, grow: true,
                    for (_ , info) in &*federations.read() {
                        FederationItem { info: info.clone() }
                    }
                }
                Flex { 
                    Button { onclick: move |_| add_federation_dialog.set(true), "Add a Federation" }
                }
            }
        }
        AddFederationDialog { add_federation_dialog, federations }
    }
}
