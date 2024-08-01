use dioxus::prelude::*;

use crate::components::ui::*;
use crate::components::widget::Widget;
use crate::state::*;

#[component]
pub fn Meta() -> Element {
    let fedimint = use_context::<Signal<Fedimint>>();

    let federation = fedimint().get_active_federation().unwrap();

    rsx! {
        Widget { title: "Metadata",
            Flex { col: true, gap: 2,
                for (key , value) in federation.meta.iter() {
                    Flex { col: true, gap: 1,
                        Flex { gap: 2,
                            Text { weight: TextWeight::Medium, "{key}" }
                            CopyButton { value: value.to_string() }
                        }
                        CopyValue { value: value.to_string() }
                    }
                }
            }
        }
    }
}
