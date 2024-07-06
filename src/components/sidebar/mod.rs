use std::{collections::BTreeMap, str::FromStr};

use dioxus::prelude::*;
use multimint::{
    fedimint_core::{api::InviteCode, config::FederationId},
    types::InfoResponse,
};

use crate::{
    components::{modal::Modal, ui::*},
    util::{state::AppState, types::Elevation},
};
mod empty;
mod federation_item;

use empty::Empty;

#[component]
pub fn Sidebar() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut invite_code = use_signal::<String>(|| "".to_string());
    let mut is_joining = use_signal::<bool>(|| false);
    let mut federations = use_signal::<BTreeMap<FederationId, InfoResponse>>(BTreeMap::new);
    let mut add_federation_modal = use_signal::<bool>(|| false);
    let federation_info = use_resource(move || async move { state.read().multimint.info().await });

    let join_federation = move |_| {
        is_joining.set(true);
        spawn(async move {
            let invite = InviteCode::from_str(invite_code().as_str()).unwrap();
            let res = state.write().multimint.register_new(invite, None).await;
            match res {
                Ok(_) => {
                    let info = state.read().multimint.info().await.unwrap();
                    federations.set(info);
                    state
                        .write()
                        .toast
                        .show("Federation Added Successfully".to_string());
                    add_federation_modal.set(false);
                }
                Err(e) => {
                    state
                        .write()
                        .toast
                        .show(format!("Failed to join Federation: {}", e));
                }
            };
            is_joining.set(false);
        });
    };

    use_context_provider(|| add_federation_modal);
    use_context_provider(|| federations);

    use_effect(move || {
        if let Some(Ok(info)) = &*federation_info.read() {
            federations.set(info.clone());
        }
    });

    rsx! {
        Flex { col: true, gap: 2, p: 2, class: "min-w-[300px]", SidebarContent {} }
        Modal { open: add_federation_modal, title: "Add a Federation",
            Flex { col: true, gap: 2,
                Flex { col: true, gap: 1,
                    label { r#for: "federation_invite", "Federation Invite Code" }
                    Input {
                        elevation: Elevation::Higher,
                        id: "federation_invite",
                        value: invite_code.read(),
                        oninput: move |e: Event<FormData>| invite_code.set(e.value()),
                        placeholder: "fed1..."
                    }
                }
                Button { elevation: Elevation::Higher, onclick: join_federation, disabled: is_joining(), "Submit" }
            }
        }
    }
}

#[component]
fn SidebarContent() -> Element {
    let federations = use_context::<Signal<BTreeMap<FederationId, InfoResponse>>>();
    let mut add_federation_modal = use_context::<Signal<bool>>();

    let open_federation_modal = move |_| add_federation_modal.set(true);

    rsx! {
        if federations.read().is_empty() {
            Empty {}
        } else {
            Flex { col: true, gap: 2, grow: true,
                for (id , value) in &*federations.read() {
                    div { "{id}" }
                }
            }
            Flex { 
                Button { onclick: open_federation_modal, "Add a Federation" }
            }
        }
    }
}
