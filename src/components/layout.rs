#![allow(dead_code)]

use crate::components::no_federation::NoFederation;
use crate::components::sidebar::Sidebar;
use crate::components::tabs::Tabs;
use crate::util::state::{AppState, Theme};
use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Layout(children: Element) -> Element {
    let state = use_context::<Signal<AppState>>();
    let has_federation = true;

    let class = tw_merge!(
        "flex grow min-h-screen w-full",
        match state.read().theme {
            Theme::Light => "",
            Theme::Dark => "dark",
        }
    );

    rsx! {
        div { class, id: "app",
            Sidebar {}
            div { class: "flex flex-col grow",
                if has_federation {
                    Tabs {}
                    {children}
                } else {
                    NoFederation {}
                }
            }
        }
    }
}
