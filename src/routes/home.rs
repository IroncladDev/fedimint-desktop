use crate::components::dashboards::Dashboards;
use crate::components::sidebar::Sidebar;
use crate::components::tabs::Tabs;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Sidebar {}
        div { class: "flex flex-col grow",
            Tabs {}
            div { class: "grow relative",
                div { class: "absolute inset-0 overflow-y-auto flex flex-col", Dashboards {} }
            }
        }
    }
}
