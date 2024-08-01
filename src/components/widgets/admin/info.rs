use dioxus::prelude::*;

use crate::components::ui::*;
use crate::components::widget::Widget;
use crate::state::*;

#[component]
pub fn Info() -> Element {
    let fedimint = use_context::<Signal<Fedimint>>();
    let federation = fedimint().get_active_federation().unwrap();

    let federation_id = federation.federation_id.to_string();
    let network = federation.network;
    let total_amount_msat = federation.total_amount_msat.to_string();
    let total_num_notes = federation.total_num_notes;

    rsx! {
        Widget { title: "Info",
            Flex { col: true, gap: 2,
                Flex { gap: 1, col: true,
                    Text { weight: TextWeight::Medium, "Balance" }
                    Code { "{total_amount_msat}" }
                }
                Flex { gap: 1, col: true,
                    Text { weight: TextWeight::Medium, "Federation ID" }
                    CopyValue { value: federation_id }
                }
                Flex { gap: 1, col: true,
                    Text { weight: TextWeight::Medium, "Network" }
                    CopyValue { value: network }
                }
                Flex { gap: 1, col: true,
                    Text { weight: TextWeight::Medium, "Total Ecash Notes" }
                    CopyValue { value: total_num_notes }
                }
            }
        }
    }
}
