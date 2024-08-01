use std::str::FromStr;

use crate::components::ui::*;
use crate::{components::widget::Widget, util::state::AppState};
use dioxus::prelude::*;
use futures_util::StreamExt;
use multimint::fedimint_core::core::OperationId;
use multimint::fedimint_ln_client::{LightningClientModule, LnReceiveState};

#[component]
pub fn Await() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut loading = use_signal(|| false);
    let mut operation_id = use_signal(String::new);

    let await_invoice = move |_| {
        spawn(async move {
            loading.set(true);
            let federation_id = state().active_federation_id.unwrap();
            let client = state().multimint.get(&federation_id).await.unwrap();
            let lightning_module = client.get_first_module::<LightningClientModule>();

            let mut updates = lightning_module
                .subscribe_ln_receive(OperationId::from_str(&operation_id()).unwrap())
                .await
                .unwrap()
                .into_stream();

            while let Some(update) = updates.next().await {
                match &update {
                    LnReceiveState::Claimed => {
                        state.write().toast.show("Invoice claimed".to_string());

                        // Update federation balance
                        let federation_info = state().multimint.info().await;
                        if let Ok(info) = federation_info {
                            for (federation_id, info) in info.iter() {
                                state
                                    .write()
                                    .federations
                                    .get_mut(federation_id)
                                    .unwrap()
                                    .total_amount_msat = info.total_amount_msat;
                            }
                        }

                        loading.set(false);
                        operation_id.set(String::new())
                    }
                    LnReceiveState::Canceled { reason } => {
                        state
                            .write()
                            .toast
                            .show(format!("Invoice canceled: {}", reason));
                        loading.set(false);
                        operation_id.set(String::new())
                    }
                    _ => {}
                }
            }
        });
    };

    rsx! {
        Widget { title: "Await Invoice",
            Flex { col: true, gap: 1,
                Text { "Operation ID" }
                Input {
                    value: "{operation_id}",
                    oninput: move |e: Event<FormData>| operation_id.set(e.value())
                }
                Button { onclick: await_invoice, disabled: loading(), "Wait for Invoice" }
            }
        }
    }
}
