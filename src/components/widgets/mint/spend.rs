use std::time::Duration;

use crate::components::ui::*;
use crate::{
    components::{dialog::Dialog, widget::Widget},
    util::state::AppState,
};
use dioxus::prelude::*;
use multimint::fedimint_core::Amount;
use multimint::fedimint_mint_client::{
    MintClientModule, SelectNotesWithAtleastAmount, SelectNotesWithExactAmount,
};
use multimint::{fedimint_core::core::OperationId, fedimint_mint_client::OOBNotes};
use tracing::warn;

#[component]
pub fn Spend() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut amount = use_signal(|| 0);
    let mut allow_overpay = use_signal(|| false);
    let mut timeout = use_signal::<u64>(|| 0);
    let mut include_invite = use_signal(|| false);
    let mut dialog = use_signal(|| false);
    let mut notes = use_signal::<Option<(OperationId, OOBNotes)>>(|| None);
    let mut loading = use_signal(|| false);

    let spend = move |_| {
        if amount() == 0 {
            state
                .write()
                .toast
                .show("Amount must be greater than 0".to_string());
            return;
        }

        spawn(async move {
            loading.set(true);
            let federation_id = state().active_federation_id.unwrap();
            let client = state().multimint.get(&federation_id).await.unwrap();
            let mint_module = client.get_first_module::<MintClientModule>();

            let res = if allow_overpay() {
                let notes = mint_module
                    .spend_notes_with_selector(
                        &SelectNotesWithAtleastAmount,
                        Amount::from_sats(amount()),
                        Duration::from_secs(timeout()),
                        include_invite(),
                        (),
                    )
                    .await;

                if let Ok(notes_res) = &notes {
                    let overspend_amount = notes_res.1.total_amount() - Amount::from_sats(amount());
                    if overspend_amount != Amount::ZERO {
                        warn!(
                            "Selected notes {} worth more than requested",
                            overspend_amount
                        );
                    }
                }

                notes
            } else {
                mint_module
                    .spend_notes_with_selector(
                        &SelectNotesWithExactAmount,
                        Amount::from_sats(amount()),
                        Duration::from_secs(timeout()),
                        include_invite(),
                        (),
                    )
                    .await
            };

            match res {
                Ok(res) => {
                    notes.set(Some(res));
                    dialog.set(true);
                }
                Err(e) => {
                    state
                        .write()
                        .toast
                        .show(format!("Error spending notes: {e}"));
                }
            }

            loading.set(false);
        });
    };

    rsx! {
        Widget { title: "Spend Notes",
            Flex { col: true, gap: 1,
                Text { "Amount (Sats)" }
                Input {
                    value: "{amount}",
                    r#type: "number",
                    oninput: move |e: Event<FormData>| amount.set(e.value().parse::<u64>().unwrap_or(0))
                }
            }
            Flex { col: true, gap: 1,
                Text { "Timeout (Seconds)" }
                Input {
                    value: "{timeout}",
                    r#type: "number",
                    oninput: move |e: Event<FormData>| timeout.set(e.value().parse::<u64>().unwrap_or(0))
                }
            }
            Flex { row: true, gap: 2, justify: FlexPosition::Between,
                Text { "Include Invite" }
                input {
                    onchange: move |_| include_invite.set(!include_invite()),
                    aria_checked: include_invite(),
                    r#type: "checkbox",
                    checked: include_invite()
                }
            }
            Flex { row: true, gap: 2, justify: FlexPosition::Between,
                Text { "Allow Overpay" }
                input {
                    onchange: move |_| allow_overpay.set(!allow_overpay()),
                    aria_checked: allow_overpay(),
                    r#type: "checkbox",
                    checked: allow_overpay()
                }
            }
            Button { onclick: spend, disabled: loading(), "Submit" }
            SpendDialog { notes, open: dialog }
        }
    }
}

#[component]
fn SpendDialog(notes: Signal<Option<(OperationId, OOBNotes)>>, open: Signal<bool>) -> Element {
    let mut state = use_context::<Signal<AppState>>();

    use_effect(move || {
        if notes().is_some() {
            open.set(true);
        } else {
            open.set(false);
        }
    });

    rsx! {
        if let Some(notes) = notes() {
            Dialog { title: "Ecash Notes", open,
                Flex { col: true, gap: 2,
                    QRCode { value: notes.1.to_string() }
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            eval(format!("window.navigator.clipboard.writeText(\"{}\")", notes.1).as_str())
                                .send("".into())
                                .unwrap();
                            state.write().toast.show("Copied to clipboard".to_string());
                        },
                        class: "w-full",
                        "Copy Notes"
                    }
                }
            }
        }
    }
}
