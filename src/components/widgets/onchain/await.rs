use futures_util::StreamExt;
use std::str::FromStr;

use crate::components::ui::*;
use crate::components::widget::Widget;
use crate::state::*;
use dioxus::prelude::*;
use multimint::fedimint_core::core::OperationId;
use multimint::fedimint_wallet_client::{DepositState, WalletClientModule};

#[component]
pub fn Await() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut loading = use_signal(|| false);
    let mut operation_id = use_signal(String::new);
    let mut fedimint = use_context::<Signal<Fedimint>>();

    let await_onchain = move |_| {
        spawn(async move {
            loading.set(true);
            let client = fedimint().get_multimint_client().await.unwrap();
            let mut updates = client
                .get_first_module::<WalletClientModule>()
                .subscribe_deposit_updates(OperationId::from_str(&operation_id()).unwrap())
                .await
                .unwrap()
                .into_stream();

            // TODO: better loading state
            while let Some(update) = updates.next().await {
                match update {
                    DepositState::Confirmed(_) => {
                        state
                            .write()
                            .toast("Onchain transaction confirmed".to_string());
                        fedimint.write().reload_active_federation().await;
                        loading.set(false);
                    }
                    DepositState::Claimed(_) => {
                        state
                            .write()
                            .toast("Onchain transaction claimed".to_string());
                        fedimint.write().reload_active_federation().await;
                        loading.set(false);
                    }
                    DepositState::Failed(reason) => {
                        state
                            .write()
                            .toast(format!("Error waiting for onchain transaction: {reason}"));
                        loading.set(false);
                    }
                    _ => {}
                }
            }
        });
    };

    rsx! {
        Widget { title: "Await Onchain Transaction",
            Flex { col: true, gap: 1,
                Text { "Operation ID" }
                Input {
                    value: "{operation_id}",
                    oninput: move |e: Event<FormData>| operation_id.set(e.value())
                }
                Button { onclick: await_onchain, disabled: loading(), "Wait for Payment" }
            }
        }
    }
}
