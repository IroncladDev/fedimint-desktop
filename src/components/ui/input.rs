#![allow(dead_code)]
use dioxus::prelude::*;
use tailwind_fuse::*;

use crate::util::types::Elevation;

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    pub oninput: EventHandler<Event<FormData>>,
    pub value: String,
    pub id: Option<String>,
    pub r#type: Option<String>,
    pub class: Option<String>,
    pub placeholder: Option<String>,
    pub size: Option<InputSize>,
    pub elevation: Option<Elevation>,
}

#[derive(Clone, PartialEq)]
pub enum InputSize {
    Default,
    Lg,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let class = tw_merge!(
        "text-sm rounded-lg transition-colors disabled:pointer-events-none disabled:opacity-50 focus:border-blue-700 outline-none w-full text-foreground placeholder:text-foreground-dimmest",
        match props.elevation {
            Some(Elevation::Root) => "bg-root border-outline-root",
            Some(Elevation::Higher) => "bg-higher border-outline-higher",
            Some(Elevation::Highest) => "bg-highest border-outline-highest",
            _ => "bg-default bg-outline-default"
        },
        match props.size {
            Some(InputSize::Lg) => "h-12 rounded-xl px-4 text-base".to_string(),
            _ => "h-10 px-4 py-2".to_string()
        },
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
