use futures_util::StreamExt;
use std::str::FromStr;

use crate::components::ui::*;
use crate::{components::widget::Widget, util::state::AppState};
use dioxus::prelude::*;
use multimint::fedimint_core::core::OperationId;
use multimint::fedimint_wallet_client::{DepositState, WalletClientModule};

#[component]
pub fn Await() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut loading = use_signal(|| false);
    let mut operation_id = use_signal(String::new);

    let await_onchain = move |_| {
        spawn(async move {
            loading.set(true);
            let federation_id = state().active_federation_id.unwrap();
            let client = state().multimint.get(&federation_id).await.unwrap();
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
                            .toast
                            .show("Onchain transaction confirmed".to_string());
                        loading.set(false);
                    }
                    DepositState::Claimed(_) => {
                        state
                            .write()
                            .toast
                            .show("Onchain transaction claimed".to_string());
                        loading.set(false);
                    }
                    DepositState::Failed(reason) => {
                        state
                            .write()
                            .toast
                            .show(format!("Error waiting for onchain transaction: {reason}"));
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
