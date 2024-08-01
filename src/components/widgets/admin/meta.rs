use dioxus::prelude::*;
use multimint::types::InfoResponse;

use crate::components::ui::*;
use crate::components::widget::Widget;

#[component]
pub fn Meta() -> Element {
    let federation = use_context::<Memo<Option<InfoResponse>>>().unwrap();

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
