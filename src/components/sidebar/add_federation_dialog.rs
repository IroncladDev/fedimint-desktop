use std::str::FromStr;

use dioxus::prelude::*;
use multimint::fedimint_core::api::InviteCode;

use crate::components::ui::*;
use crate::state::*;

#[component]
pub fn AddFederationDialog(add_federation_dialog: Signal<bool>) -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut is_joining = use_signal::<bool>(|| false);
    let mut invite_code = use_signal::<String>(|| "".to_string());
    let fedimint = use_context::<Signal<Fedimint>>();

    let join_federation = move |_| {
        is_joining.set(true);
        spawn(async move {
            let invite = InviteCode::from_str(invite_code().as_str()).unwrap();
            let res = fedimint().multimint.register_new(invite, None).await;
            match res {
                Ok(_) => {
                    let info = fedimint().multimint.info().await.unwrap();
                    // TODO: update meta as well in helper function
                    fedimint().federations = info;
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
                    label { "Federation Invite Code" }
                    Input {
                        value: invite_code(),
                        oninput: move |e: Event<FormData>| invite_code.set(e.value()),
                        placeholder: "fed1..."
                    }
                }
                Button { onclick: join_federation, disabled: is_joining() || invite_code().is_empty(), "Submit" }
            }
        }
    }
}
