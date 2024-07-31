use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Code(children: Element, copy_value: Option<String>) -> Element {
    let class = tw_merge!(
        "text-sm font-mono bg-muted rounded px-2 py-1",
        match copy_value {
            Some(_) => "cursor-pointer",
            None => "",
        }
    );

    let copy = move |_| {
        if let Some(value) = copy_value.clone() {
            eval(format!("navigator.clipboard.writeText(\"{:?}\")", value).as_str());
        }
    };

    rsx! {
        code { class, onclick: copy, {children} }
    }
}
