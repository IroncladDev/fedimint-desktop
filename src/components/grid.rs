use dioxus::prelude::*;

#[component]
pub fn Grid(children: Element) -> Element {
    rsx! {
        div { class: "masonry", {children} }
    }
}
