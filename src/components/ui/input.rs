#![allow(dead_code)]
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    pub oninput: EventHandler<Event<FormData>>,
    pub value: String,
    pub id: Option<String>,
    pub r#type: Option<String>,
    pub class: Option<String>,
    pub placeholder: Option<String>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let class = tw_merge!(
        "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
        props.class
    );

    rsx! {
        input {
            class,
            id: props.id,
            placeholder: props.placeholder,
            value: props.value,
            r#type: props.r#type,
            oninput: move |e| props.oninput.call(e)
        }
    }
}
