use dioxus::prelude::*;

#[component]
pub fn Code(children: Element) -> Element {
    rsx! {
        code { class: "bg-slate-700 rounded px-1 py-0.5 text-xs break-all", {children} }
    }
}
