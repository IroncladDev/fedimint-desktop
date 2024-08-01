use crate::components::sidebar::Sidebar;
use crate::components::tabs::Tabs;
use crate::components::widgets::Widgets;
use crate::state::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let fedimint = use_context::<Signal<Fedimint>>();

    rsx! {
        Sidebar {}
        div { class: "flex flex-col grow",
            Tabs {}
            div { class: "grow relative",
                div { class: "absolute inset-0 overflow-y-auto flex flex-col",
                    if let Some(_) = fedimint().active_federation_id {
                        Widgets {}
                    } else {
                        "Loading"
                    }
                }
            }
        }
    }
}
