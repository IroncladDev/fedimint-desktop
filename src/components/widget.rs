use dioxus::prelude::*;

use crate::components::ui::{Text, TextSize, TextWeight};

#[component]
pub fn Widget(children: Element, title: String) -> Element {
    rsx! {
        div { class: "flex flex-col grow min-w-[360px] gap-4 rounded-lg p-4 bg-slate-900 border-2 border-slate-800 basis-0",
            Text { size: TextSize::Lg, weight: TextWeight::Bold, "{title}" }
            {children}
        }
    }
}
