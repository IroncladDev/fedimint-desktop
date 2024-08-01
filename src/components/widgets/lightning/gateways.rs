use crate::components::ui::*;
use crate::components::widget::Widget;
use crate::state::*;
use dioxus::prelude::*;
use multimint::fedimint_ln_client::LightningClientModule;

#[component]
pub fn Gateways() -> Element {
    let fedimint = use_context::<Signal<Fedimint>>();

    let gateways = use_resource(move || async move {
        let client = fedimint().get_multimint_client().await.unwrap();
        let lightning_module = client.get_first_module::<LightningClientModule>();

        lightning_module.list_gateways().await
    });

    rsx! {
        match &*gateways.read_unchecked() {
            Some(gateways) => rsx! {
                Widget { title: "Lightning Gateways",
                    Flex { col: true, gap: 2,
                        if gateways.is_empty() {
                            Text { weight: TextWeight::Medium, "No gateways" }
                        } else {
                            for gateway in gateways.iter() {
                                Flex { col: true, gap: 2, p: 2, class: "border rounded-lg",
                                    Flex { gap: 2,
                                        Text { weight: TextWeight::Medium, size: TextSize::Lg, "{gateway.info.lightning_alias}" },
                                        CopyButton { value: gateway.info.lightning_alias.clone() }
                                    }
                                    Flex { gap: 1,
                                        col: true,
                                        Text { "Gateway ID" },
                                        CopyValue { value: "{gateway.info.gateway_id.to_string()}" }
                                    }
                                    Flex { gap: 1,
                                        col: true,
                                        Text { "Node Public Key" },
                                        CopyValue { value: "{gateway.info.node_pub_key.to_string()}" }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            None => rsx! {
                Widget { title: "Lightning Gateways",
                    "Loading"
                }
            },
        }
    }
}
