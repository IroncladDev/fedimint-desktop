#![allow(dead_code)]
use dioxus::prelude::*;
use tailwind_fuse::*;

use crate::util::types::Elevation;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub onclick: EventHandler<MouseEvent>,
    pub size: Option<ButtonSize>,
    pub variant: Option<ButtonVariant>,
    pub elevation: Option<Elevation>,
    pub class: Option<String>,
    pub disabled: Option<bool>,
    pub children: Element,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Filled,
    Outline,
}

pub fn Button(props: ButtonProps) -> Element {
    let class = tw_merge!(
        "inline-flex gap-2 items-center justify-center whitespace-nowrap rounded-lg transition-colors disabled:pointer-events-none disabled:opacity-50 outline-none text-foreground border-2",
        match props.size {
            Some(ButtonSize::Sm) => "h-8 rounded-md px-2 py-1 text-xs",
            Some(ButtonSize::Lg) => "h-12 rounded-xl px-8 text-base",
            Some(ButtonSize::Icon) => "h-8 w-8",
            _ => "h-10 px-4 py-2"
        },
        match props.variant {
            Some(ButtonVariant::Outline) => "!bg-transparent border-solid",
            Some(ButtonVariant::Filled) => "border-hidden",
            _ => "border-solid"
        },
        match props.elevation {
            Some(Elevation::Root) => "border-outline-root bg-root hover:bg-default hover:border-outline-default",
            Some(Elevation::Higher) => "border-outline-higher bg-higher hover:bg-highest: hover:border-outline-highest",
            Some(Elevation::Highest) => "border-outline-highest bg-highest hover:bg-higher hover:border-outline-higher",
            _ => "border-outline-default bg-default hover:bg-higher hover:border-outline-higher"
        },
        match props.disabled {
            Some(true) => "opacity-50",
            _ => ""
        },
        props.class
    );

    rsx! {
        button { class, onclick: move |e| props.onclick.call(e), {props.children} }
    }
}
