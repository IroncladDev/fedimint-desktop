use crate::components::dashboards::Dashboards;
use crate::components::layout::Layout;
use crate::components::toast::Toast;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Layout { 
            div { class: "grow relative",
                div { class: "absolute inset-0 overflow-y-auto flex flex-col", Dashboards {} }
            }
            Toast {}
        }
    }
}
