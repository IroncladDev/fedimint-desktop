use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Popover(children: Element) -> Element {
    rsx! {
        div { class: "popover", {children} }
    }
}

#[component]
pub fn PopoverTrigger(children: Element, as_child: Option<bool>) -> Element {
    let is_as_child = as_child.unwrap_or(false);

    rsx! {
        if is_as_child {
            {children}
        } else {
            div { tabindex: "1", onclick: move |e| e.stop_propagation(), {children} }
        }
    }
}

#[component]
pub fn PopoverContent(children: Element, class: Option<String>) -> Element {
    rsx! {
        div { tabindex: "0", class: tw_merge!("popover-content", class), {children} }
    }
}
