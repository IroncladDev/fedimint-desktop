use dioxus::prelude::*;

use crate::components::ui::{Text, TextSize, TextWeight};

#[component]
pub fn Widget(children: Element, title: String) -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 rounded-lg p-4 bg-slate-900 border-2 border-slate-800 h-auto max-h-screen overflow-y-auto",
            Text { class: "shrink-0", size: TextSize::Lg, weight: TextWeight::Bold, "{title}" }
            {children}
        }
    }
}
