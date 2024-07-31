use dioxus::prelude::*;
use multimint::types::InfoResponse;

use crate::components::ui::*;
use crate::components::widget::Widget;

#[component]
pub fn Info() -> Element {
    let active_federation = use_context::<Memo<Option<InfoResponse>>>();
    let federation = active_federation.unwrap();

    rsx! {
        Widget { title: "Info",
            Flex { gap: 2, col: true,
                Text { weight: TextWeight::Medium, size: TextSize::Lg, "Federation ID" }
                Code { copy_value: "{federation.federation_id}", "{federation.federation_id}" }
            }
        }
    }
}
