use dioxus::prelude::*;

use crate::components::ui::*;
use crate::components::widget::Widget;
use crate::state::*;

#[component]
pub fn Denominations() -> Element {
    let fedimint = use_context::<Signal<Fedimint>>();
    let federation = fedimint().get_active_federation().unwrap();

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
