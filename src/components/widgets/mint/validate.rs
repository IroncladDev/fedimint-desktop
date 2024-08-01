use crate::components::ui::*;
use crate::components::widget::Widget;
use crate::state::*;
use dioxus::prelude::*;
use multimint::fedimint_mint_client::{MintClientModule, OOBNotes};
use std::str::FromStr;

#[component]
pub fn Validate() -> Element {
    let fedimint = use_context::<Signal<Fedimint>>();
    let mut state = use_context::<Signal<AppState>>();
    let mut notes = use_signal(String::new);
    let mut loading = use_signal(|| false);

    let validate = move |_| {
        if notes().is_empty() {
            state
                .write()
                .toast
                .show("Notes must be provided".to_string());
            return;
        }

        spawn(async move {
            loading.set(true);
            let client = fedimint().get_multimint_client().await.unwrap();

            let ecash_notes = OOBNotes::from_str(&notes()).unwrap();
            let amount_msat = client
                .get_first_module::<MintClientModule>()
                .validate_notes(ecash_notes)
                .await;

            match amount_msat {
                Ok(amount_msat) => {
                    state
                        .write()
                        .toast
                        .show(format!("Validated {} of ecash notes", amount_msat));
                }
                Err(e) => {
                    state
                        .write()
                        .toast
                        .show(format!("Error validating notes: {e}"));
                }
            }

            notes.set(String::new());
            loading.set(false);
        });
    };

    rsx! {
        Widget { title: "Validate Notes",
            Flex { col: true, gap: 1,
                Text { "Notes" }
                Input { value: "{notes}", oninput: move |e: Event<FormData>| notes.set(e.value()) }
            }
            Button { onclick: validate, disabled: loading(), "Validate Notes" }
        }
    }
}
