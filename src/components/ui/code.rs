use crate::state::*;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdCopy, Icon};

#[component]
pub fn Code(children: Element) -> Element {
    rsx! {
        code { class: "text-sm font-mono bg-muted rounded px-2 py-1", {children} }
    }
}

#[component]
pub fn CopyValue(value: String) -> Element {
    let mut state = use_context::<Signal<AppState>>();

    rsx! {
        code {
            class: "text-sm font-mono bg-muted rounded py-1 px-1.5 break-all relative cursor-pointer group",
            onclick: move |_| {
                eval(format!("window.navigator.clipboard.writeText(\"{}\")", value).as_str())
                    .send("".into())
                    .unwrap();
                state.write().toast.show("Copied to clipboard".to_string());
            },
            Icon {
                width: 12,
                height: 12,
                class: "text-muted-foreground absolute right-2 top-2 invisible group-hover:visible drop-shadow-sm bg-muted",
                icon: LdCopy
            }
            "{value}"
        }
    }
}
