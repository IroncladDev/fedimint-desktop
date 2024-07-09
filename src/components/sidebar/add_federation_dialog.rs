use std::collections::BTreeMap;
use std::str::FromStr;

use dioxus::prelude::*;
use multimint::{
    fedimint_core::{api::InviteCode, config::FederationId},
    types::InfoResponse,
};

use crate::{
    components::{dialog::Dialog, ui::*},
    util::state::AppState,
};

#[component]
pub fn AddFederationDialog(
    federations: Signal<BTreeMap<FederationId, InfoResponse>>,
    add_federation_dialog: Signal<bool>,
) -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut is_joining = use_signal::<bool>(|| false);
    let mut invite_code = use_signal::<String>(|| "".to_string());

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
                    add_federation_dialog.set(false);
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

    rsx! {
        Dialog { open: add_federation_dialog, title: "Add a Federation",
            Flex { col: true, gap: 2,
                Flex { col: true, gap: 1,
                    label { r#for: "federation_invite", "Federation Invite Code" }
                    Input {
                        id: "federation_invite",
                        value: invite_code.read(),
                        oninput: move |e: Event<FormData>| invite_code.set(e.value()),
                        placeholder: "fed1..."
                    }
                }
                Button { onclick: join_federation, disabled: is_joining() || invite_code().is_empty(), "Submit" }
            }
        }
    }
}
