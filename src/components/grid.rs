use dioxus::prelude::*;

#[component]
pub fn Grid(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-wrap justify-center w-full gap-2 p-2", {children} }
    }
}
