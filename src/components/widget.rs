use dioxus::prelude::*;

#[component]
pub fn Widget(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col grow min-w-[360px] gap-2 rounded p-4 bg-slate-900 border-2 border-slate-800 basis-0",
            {children}
        }
    }
}
