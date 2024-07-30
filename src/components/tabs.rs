use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdArrowRightFromLine, LdMenu},
    Icon,
};

use crate::{
    components::ui::*,
    util::{meta::get_federation_icon, state::AppState, types::Tab},
};

#[component]
pub fn Tabs() -> Element {
    rsx! {
        div { class: "flex gap-2 items-center w-full p-2 border-b",
            Flex { grow: true, FederationIndicator {} }
            Flex { gap: 2, align: FlexPosition::Center,
                TabItem { tab: Tab::Admin }
                TabItem { tab: Tab::Lightning }
                TabItem { tab: Tab::Mint }
                TabItem { tab: Tab::Onchain }
            }
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

#[component]
fn FederationIndicator() -> Element {
    let state = use_context::<Signal<AppState>>();

    let state_read = state.read();
    let active_federation = state_read.active_federation_id;

    if active_federation.is_none() {
        return rsx! {
            ToggleButton {
                Icon { width: 16, height: 16, class: "text-muted-foreground", icon: LdMenu }
            }
        };
    }

    let federation = state_read
        .federations
        .get(&active_federation.unwrap())
        .unwrap();

    let src = get_federation_icon(federation.clone(), Some(state_read.theme.clone()));
    let name: String = if let Some(n) = federation.meta.get("federation_name") {
        n.to_string()
    } else {
        "unknown".to_string()
    };

    rsx! {
        ToggleButton {
            if !state_read.sidebar_open {
                Icon { width: 16, height: 16, class: "text-muted-foreground", icon: LdArrowRightFromLine }
            }
            img { src, class: "w-6 h-6 rounded-full border" }
            Text { "{name}" }
        }
    }
}

#[component]
fn ToggleButton(children: Element) -> Element {
    let mut state = use_context::<Signal<AppState>>();

    rsx! {
        button {
            class: "flex gap-2 items-center border rounded-full px-2 py-1.5 hover:bg-muted transition-colors",
            onclick: move |_| state.write().sidebar_open = !state().sidebar_open,
            {children}
        }
    }
}
