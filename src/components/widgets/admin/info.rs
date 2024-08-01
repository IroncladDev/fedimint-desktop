use dioxus::prelude::*;
use multimint::types::InfoResponse;

use crate::components::ui::*;
use crate::components::widget::Widget;

#[component]
pub fn Info() -> Element {
    let federation = use_context::<Memo<Option<InfoResponse>>>().unwrap();

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
