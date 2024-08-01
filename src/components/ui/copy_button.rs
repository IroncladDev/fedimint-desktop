use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdCopy, Icon};

use crate::state::*;

#[component]
pub fn CopyButton(value: String) -> Element {
    let mut state = use_context::<Signal<AppState>>();

    rsx! {
        button {
            class: "rounded border flex items-center justify-center w-6 h-6 cursor-pointer hover:bg-muted",
            onclick: move |_| {
                eval(format!("window.navigator.clipboard.writeText(\"{}\")", value).as_str())
                    .send("".into())
                    .unwrap();
                state.write().toast("Copied to clipboard".to_string());
            },
            Icon { width: 16, height: 16, class: "text-muted-foreground", icon: LdCopy }
        }
    }
}
