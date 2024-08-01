use dioxus::prelude::*;
use multimint::types::InfoResponse;

use crate::components::ui::*;
use crate::components::widget::Widget;

#[component]
pub fn Denominations() -> Element {
    let federation = use_context::<Memo<Option<InfoResponse>>>().unwrap();

    let denominations = federation.denominations_msat.iter();

    rsx! {
        Widget { title: "Denominations",
            Flex { col: true, gap: 1,
                for (denomination , amount) in denominations {
                    Text { "{denomination}: {amount}" }
                }
            }
        }
    }
}
