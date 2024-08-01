use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdArrowRightFromLine, LdInfo, LdLandmark, LdLink, LdMenu, LdZap},
    Icon,
};

use crate::{components::ui::*, util::meta::get_federation_icon};
use crate::{state::*, util::meta::get_federation_name};

#[component]
pub fn Tabs() -> Element {
    rsx! {
        div { class: "flex gap-2 items-center w-full border-b",
            Flex { p: 2, grow: true, FederationIndicator {} }
            Flex { align: FlexPosition::End, class: "px-2 self-end",
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

    let current_tab = state().tab.clone();
    let tab_cloned = tab.clone();

    let switch_tab = move |_| {
        let _ = &state.write().switch_tab(tab_cloned.clone());
    };

    let class_name = if current_tab == tab { "bg-muted" } else { "" };

    rsx! {
        button {
            class: "grow border border-b-0 {class_name} rounded-t-lg px-2 py-1 text-left inline-flex gap-2 items-center text-sm",
            onclick: switch_tab,
            TabContent { tab }
        }
    }
}

#[component]
fn TabContent(tab: Tab) -> Element {
    match tab {
        Tab::Admin => rsx! {
            Icon { width: 16, height: 16, class: "text-muted-foreground", icon: LdInfo }
            "Info"
        },
        Tab::Lightning => rsx! {
            Icon { width: 16, height: 16, class: "text-muted-foreground", icon: LdZap }
            "Lightning"
        },
        Tab::Mint => rsx! {
            Icon { width: 16, height: 16, class: "text-muted-foreground", icon: LdLandmark }
            "Mint"
        },
        Tab::Onchain => rsx! {
            Icon { width: 16, height: 16, class: "text-muted-foreground", icon: LdLink }
            "Onchain"
        },
    }
}

#[component]
fn FederationIndicator() -> Element {
    let state = use_context::<Signal<AppState>>();
    let fedimint = use_context::<Signal<Fedimint>>();

    let state_read = state.read();
    let active_federation = fedimint().active_federation_id;

    if active_federation.is_none() {
        return rsx! {
            ToggleButton { 
                Icon { width: 16, height: 16, class: "text-muted-foreground", icon: LdMenu }
            }
        };
    }

    let federation = fedimint().get_active_federation().unwrap();

    let src = get_federation_icon(federation.clone(), Some(state_read.theme.clone()));
    let name = get_federation_name(federation.clone());

    rsx! {
        ToggleButton { 
            if !state().sidebar_open {
                Icon {
                    width: 16,
                    height: 16,
                    class: "text-muted-foreground",
                    icon: LdArrowRightFromLine
                }
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
