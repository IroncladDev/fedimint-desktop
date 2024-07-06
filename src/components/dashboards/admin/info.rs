use dioxus::prelude::*;

#[component]
pub fn Info() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 grow basis-0 overflow-y-auto bg-slate-900 p-2",
            h2 { class: "text-xl font-bold", "Info" }
        }
    }
}
