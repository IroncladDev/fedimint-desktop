use crate::components::ui::*;
use futures_util::StreamExt;
use std::str::FromStr;

use crate::components::widget::Widget;
use crate::state::*;
use dioxus::prelude::*;
use multimint::fedimint_mint_client::{MintClientModule, OOBNotes, ReissueExternalNotesState};

#[component]
pub fn Reissue() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut notes = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut fedimint = use_context::<Signal<Fedimint>>();

    let reissue = move |_| {
        if notes().is_empty() {
            state.write().toast("Notes must be provided".to_string());
            return;
        }

        spawn(async move {
            loading.set(true);
            let client = fedimint().get_multimint_client().await.unwrap();
            let mint = client.get_first_module::<MintClientModule>();

            let ecash_notes = OOBNotes::from_str(&notes()).unwrap();

            // TODO: error handling for double-spending notes

            let operation_id = mint
                .reissue_external_notes(ecash_notes.clone(), ())
                .await
                .unwrap();
            let mut updates = mint
                .subscribe_reissue_external_notes(operation_id)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to subscribe to reissue operation: {}", e))
                .unwrap()
                .into_stream();

            while let Some(update) = updates.next().await {
                if let ReissueExternalNotesState::Failed(e) = update {
                    state.write().toast(format!("Error reissuing notes: {e}"));
                }
            }

            fedimint.write().reload_active_federation().await;
            state.write().toast(format!(
                "Reissued {} of ecash notes",
                ecash_notes.total_amount()
            ));
            notes.set(String::new());
            loading.set(false);
        });
    };

    rsx! {
        Widget { title: "Reissue Notes",
            Flex { col: true, gap: 1,
                Text { "Notes" }
                Input { value: "{notes}", oninput: move |e: Event<FormData>| notes.set(e.value()) }
            }
            Button { onclick: reissue, disabled: loading(), "Reissue Notes" }
        }
    }
}
