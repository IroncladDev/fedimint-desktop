use dioxus::prelude::*;

use crate::components::ui::{Button, ButtonSize};

#[component]
pub fn Modal(open: Signal<bool>, children: Element, title: String) -> Element {
    let display = if *open.read() { "" } else { "hidden" };

    rsx! {
        div { class: "fixed inset-0 z-50 flex items-center justify-center {display}",
            div {
                class: "fixed inset-0 bg-slate-950 opacity-50",
                onclick: move |_| open.set(false)
            }
            div { class: "relative bg-default border border-outline-default rounded-lg p-6 z-10 flex flex-col gap-2 min-w-[360px] max-w-[480px]",
                div { class: "flex justify-between gap-2 items-center",
                    h2 { class: "font-bold text-lg", "{title}" }
                }
                Button {
                    size: ButtonSize::Icon,
                    class: "text-foreground-higher absolute top-4 right-4 text-lg w-6 h-6 flex items-center justify-center",
                    onclick: move |_| open.set(false),
                    "✕"
                }
                {children}
            }
        }
    }
}
