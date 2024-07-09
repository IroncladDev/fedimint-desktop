#![allow(dead_code)]
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub onclick: EventHandler<MouseEvent>,
    pub size: Option<ButtonSize>,
    pub variant: Option<ButtonVariant>,
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
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

pub fn Button(props: ButtonProps) -> Element {
    let class = tw_merge!(
        "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        match props.size {
            Some(ButtonSize::Sm) => "h-9 rounded-md px-3",
            Some(ButtonSize::Lg) => "h-11 rounded-md px-8",
            Some(ButtonSize::Icon) => "h-10 w-10",
            _ => "h-10 px-4 py-2"
        },
        match props.variant {
            Some(ButtonVariant::Destructive) => "bg-destructive text-destructive-foreground",
            Some(ButtonVariant::Outline) => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            Some(ButtonVariant::Secondary) => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            Some(ButtonVariant::Ghost) => "hover:bg-accent hover:text-accent-foreground",
            _ => "bg-primary text-primary-foreground hover:bg-primary/90"
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
