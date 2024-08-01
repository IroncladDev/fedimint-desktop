use crate::components::ui::*;
use crate::components::widget::Widget;
use crate::state::*;
use dioxus::prelude::*;
use multimint::fedimint_core::core::OperationId;
use multimint::fedimint_core::Amount;
use multimint::fedimint_ln_client::LightningClientModule;
use multimint::fedimint_ln_common::lightning_invoice::{
    Bolt11Invoice, Bolt11InvoiceDescription, Description,
};

#[component]
pub fn Invoice() -> Element {
    let fedimint = use_context::<Signal<Fedimint>>();
    let mut state = use_context::<Signal<AppState>>();
    let mut amount = use_signal(|| 0);
    let mut description = use_signal(String::new);
    let mut expiry = use_signal(|| 0);
    let mut invoice = use_signal::<Option<(OperationId, Bolt11Invoice, [u8; 32])>>(|| None);
    let mut dialog = use_signal(|| false);
    let mut loading = use_signal(|| false);

    let create_invoice = move |_| {
        if amount() == 0 {
            state
                .write()
                .toast
                .show("Amount must be greater than 0".to_string());
            return;
        }

        spawn(async move {
            loading.set(true);
            let client = fedimint().get_multimint_client().await.unwrap();
            let lightning_module = client.get_first_module::<LightningClientModule>();
            let expiry_time = if expiry() == 0 { None } else { Some(expiry()) };
            let gateway = &lightning_module.list_gateways().await[0];

            let created_invoice = lightning_module
                .create_bolt11_invoice(
                    Amount::from_sats(amount()),
                    Bolt11InvoiceDescription::Direct(&Description::new(description()).unwrap()),
                    expiry_time,
                    (),
                    Some(gateway.info.to_owned()),
                )
                .await;

            match created_invoice {
                Ok(res) => {
                    invoice.set(Some(res));
                    dialog.set(true);
                }
                Err(e) => {
                    state
                        .write()
                        .toast
                        .show(format!("Error creating invoice: {e}"));
                }
            }

            loading.set(false);
        });
    };

    rsx! {
        Widget { title: "Create Invoice",
            Flex { col: true, gap: 1,
                Text { "Amount (Sats)" }
                Input {
                    value: "{amount}",
                    r#type: "number",
                    oninput: move |e: Event<FormData>| amount.set(e.value().parse::<u64>().unwrap_or(0))
                }
            }
            Flex { col: true, gap: 1,
                Text { "Description" }
                Input {
                    value: "{description}",
                    oninput: move |e: Event<FormData>| description.set(e.value())
                }
            }
            Flex { col: true, gap: 1,
                Text { "Expiry (Seconds) (Optional)" }
                Input {
                    value: "{expiry}",
                    r#type: "number",
                    oninput: move |e: Event<FormData>| expiry.set(e.value().parse::<u64>().unwrap_or(0))
                }
            }
            Button { onclick: create_invoice, disabled: loading(), "Submit" }
            InvoiceDialog { invoice, open: dialog }
        }
    }
}

#[component]
fn InvoiceDialog(
    invoice: Signal<Option<(OperationId, Bolt11Invoice, [u8; 32])>>,
    open: Signal<bool>,
) -> Element {
    let mut state = use_context::<Signal<AppState>>();

    use_effect(move || {
        if invoice().is_some() {
            open.set(true);
        } else {
            open.set(false);
        }
    });

    rsx! {
        if let Some(inv) = invoice() {
            Dialog { title: "Lightning Invoice", open,
                Flex { col: true, gap: 2,
                    QRCode { value: inv.1.to_string() }
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            eval(format!("window.navigator.clipboard.writeText(\"{}\")", inv.1).as_str())
                                .send("".into())
                                .unwrap();
                            state.write().toast.show("Copied to clipboard".to_string());
                        },
                        class: "w-full",
                        "Copy Invoice"
                    }
                    Flex { col: true, gap: 1,
                        Text { "Operation ID" }
                        CopyValue { value: inv.0.to_string() }
                    }
                }
            }
        }
    }
}
