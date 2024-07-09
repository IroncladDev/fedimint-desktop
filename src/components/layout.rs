#![allow(dead_code)]

use crate::components::sidebar::Sidebar;
use crate::components::tabs::Tabs;
use dioxus::prelude::*;

#[component]
pub fn Layout(children: Element) -> Element {
    rsx! {
        Sidebar {}
        div { class: "flex flex-col grow", Tabs {} }
    }
}
