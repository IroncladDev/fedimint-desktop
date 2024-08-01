use std::time::Duration;

use crate::components::ui::*;
use crate::components::widget::Widget;
use crate::state::*;
use dioxus::prelude::*;
use multimint::fedimint_core::core::OperationId;
use multimint::fedimint_ln_common::bitcoin::Address;
use multimint::{fedimint_core::time::now, fedimint_wallet_client::WalletClientModule};

#[component]
pub fn CreateAddress() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut timeout = use_signal::<u64>(|| 0);
    let mut loading = use_signal(|| false);
    let mut address = use_signal::<Option<(OperationId, Address)>>(|| None);
    let mut dialog = use_signal(|| false);
    let fedimint = use_context::<Signal<Fedimint>>();

    let create_address = move |_| {
        spawn(async move {
            loading.set(true);
            let client = fedimint().get_multimint_client().await.unwrap();
            let wallet_module = client.get_first_module::<WalletClientModule>();
            let res = wallet_module
                .get_deposit_address(now() + Duration::from_secs(timeout()), ())
                .await;

            match res {
                Ok(addr) => {
                    address.set(Some(addr));
                    dialog.set(true);
                }
                Err(e) => {
                    state
                        .write()
                        .toast(format!("Error creating onchain address: {e}"));
                }
            }
            loading.set(false);
        });
    };

    rsx! {
        Widget { title: "Create Onchain Address",
            Flex { col: true, gap: 1,
                Text { "Timeout (Seconds)" }
                Input {
                    value: "{timeout}",
                    r#type: "number",
                    oninput: move |e: Event<FormData>| timeout.set(e.value().parse::<u64>().unwrap_or(0))
                }
            }
            Button { onclick: create_address, disabled: loading(), "Create Address" }
            CreateAddressDialog { address, open: dialog }
        }
    }
}

#[component]
fn CreateAddressDialog(
    address: Signal<Option<(OperationId, Address)>>,
    open: Signal<bool>,
) -> Element {
    let mut state = use_context::<Signal<AppState>>();

    rsx! {
        if let Some(address) = address() {
            Dialog { title: "Onchain Address", open,
                Flex { col: true, gap: 2,
                    QRCode { value: address.1.clone() }
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            eval(format!("window.navigator.clipboard.writeText(\"{}\")", address.1).as_str())
                                .send("".into())
                                .unwrap();
                            state.write().toast("Copied to clipboard".to_string());
                        },
                        class: "w-full",
                        "Copy Address"
                    }
                }
                Flex { col: true, gap: 2,
                    Text { "Operation ID" }
                    CopyValue { value: address.0.to_string() }
                }
            }
        }
    }
}
