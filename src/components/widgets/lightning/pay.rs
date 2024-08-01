use crate::components::ui::*;
use crate::components::widget::Widget;
use crate::state::*;
use crate::util::get_invoice::{get_invoice, LnPayRequest};
use dioxus::prelude::*;
use multimint::fedimint_core::Amount;
use multimint::fedimint_ln_client::LightningClientModule;

#[component]
pub fn Pay() -> Element {
    let fedimint = use_context::<Signal<Fedimint>>();
    let mut state = use_context::<Signal<AppState>>();
    let mut loading = use_signal(|| false);
    let mut payment_info = use_signal(String::new);
    let mut payment_amount = use_signal(|| 0);
    let mut lnurl_comment = use_signal(String::new);

    let pay_invoice = move |_| {
        spawn(async move {
            loading.set(true);
            let bolt11 = get_invoice(&LnPayRequest {
                payment_info: payment_info(),
                amount_msat: if payment_amount() == 0 {
                    None
                } else {
                    Some(Amount::from_sats(payment_amount()))
                },
                lnurl_comment: if lnurl_comment().is_empty() {
                    None
                } else {
                    Some(lnurl_comment())
                },
            })
            .await;

            if bolt11.is_err() {
                loading.set(false);
                state
                    .write()
                    .toast
                    .show("Failed to generate invoice".to_string());
                return;
            }

            let client = fedimint().get_multimint_client().await.unwrap();
            let lightning_module = client.get_first_module::<LightningClientModule>();
            let gateways = lightning_module.list_gateways().await;

            if gateways.is_empty() {
                state
                    .write()
                    .toast
                    .show("No Lightning gateways available".to_string());
                return;
            }

            let gateway = &gateways[0];

            let _ = lightning_module
                .pay_bolt11_invoice(Some(gateway.info.to_owned()), bolt11.unwrap(), ())
                .await;

            state
                .write()
                .toast
                .show("Invoice paid successfully".to_string());
            loading.set(false);
        });
    };

    rsx! {
        Widget { title: "Pay Invoice",
            Flex { col: true, gap: 2,
                Flex { col: true, gap: 1,
                    Text { "Payment Info" }
                    Text { size: TextSize::Xs, "Can be a lightning address, LNURL, or Bolt11 invoice" }
                    Input {
                        placeholder: "lnbc...",
                        value: payment_info,
                        oninput: move |e: Event<FormData>| payment_info.set(e.value())
                    }
                }
                Flex { col: true, gap: 1,
                    Text { "Amount (in sats)" }
                    Text { size: TextSize::Xs, "Required if using a lightning address" }
                    Input {
                        placeholder: "1000",
                        value: payment_amount,
                        oninput: move |e: Event<FormData>| payment_amount.set(e.value().parse::<u64>().unwrap_or(0))
                    }
                }
                Flex { col: true, gap: 1,
                    Text { "LNURL Comment (optional)" }
                    Text { size: TextSize::Xs, "Only used if using a LNURL" }
                    Input {
                        value: lnurl_comment,
                        oninput: move |e: Event<FormData>| lnurl_comment.set(e.value())
                    }
                }
                Button { onclick: pay_invoice, disabled: loading(), "Pay" }
            }
        }
    }
}
