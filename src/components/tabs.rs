use dioxus::prelude::*;

use crate::util::{state::AppState, types::Tab};

#[component]
pub fn Tabs() -> Element {
    rsx! {
        div { class: "flex gap-2 items-center w-full p-2",
            TabItem { tab: Tab::Admin }
            TabItem { tab: Tab::Lightning }
            TabItem { tab: Tab::Mint }
            TabItem { tab: Tab::Onchain }
        }
    }
}

#[component]
fn TabItem(tab: Tab) -> Element {
    let mut state = use_context::<Signal<AppState>>();

    let current_tab = state.read().tab.clone();
    let tab_cloned = tab.clone();

    let switch_tab = move |_| {
        let _ = &state.write().switch_tab(tab_cloned.clone());
    };

    let class_name = if current_tab == tab {
        "text-blue-500"
    } else {
        "text-white"
    };

    rsx! {
        button { class: "grow {class_name}", onclick: switch_tab,
            TabContent { tab }
        }
    }
}

#[component]
fn TabContent(tab: Tab) -> Element {
    rsx! {
        match tab {
            Tab::Admin => "admin",
            Tab::Lightning => "lightning",
            Tab::Mint => "Mint",
            Tab::Onchain => "Onchain"
        }
    }
}
