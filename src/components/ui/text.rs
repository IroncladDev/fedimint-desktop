#![allow(dead_code)]
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(Props, Clone, PartialEq)]
pub struct TextProps {
    pub children: Element,
    pub class: Option<String>,
    pub size: Option<TextSize>,
    pub color: Option<TextColor>,
    pub weight: Option<TextWeight>,
    pub multiline: Option<TextMultiline>,
    pub center: Option<bool>,
}

#[derive(Clone, PartialEq)]
pub enum TextSize {
    Xs,
    Xxs,
    Sm,
    Base,
    Lg,
    Xl,
    H1,
    H2,
}

#[derive(Clone, PartialEq)]
pub enum TextColor {
    Default,
    Dimmer,
    Dimmest,
    Inherit,
}

#[derive(Clone, PartialEq)]
pub enum TextWeight {
    Default,
    Medium,
    Semibold,
    Bold,
}

#[derive(Clone, PartialEq)]
pub enum TextMultiline {
    Multiline,
    Truncate,
    Clamp2,
    Clamp3,
    Clamp4,
}

#[component]
pub fn Text(props: TextProps) -> Element {
    let class = tw_merge!(
        "flex flex-row",
        match props.size {
            Some(TextSize::Xs) => "text-xs",
            Some(TextSize::Xxs) => "text-[10px] leading-[14px]",
            Some(TextSize::Sm) => "text-sm",
            Some(TextSize::Lg) => "text-lg",
            Some(TextSize::Xl) => "text-xl",
            Some(TextSize::H1) => "text-6xl max-md:text-[40px]",
            Some(TextSize::H2) => "text-3xl",
            _ => "text-base",
        },
        match props.color {
            Some(TextColor::Dimmer) => "text-foreground-dimmer",
            Some(TextColor::Dimmest) => "text-foreground-dimmest",
            Some(TextColor::Inherit) => "text-inherit",
            _ => "text-foreground",
        },
        match props.weight {
            Some(TextWeight::Medium) => "font-medium",
            Some(TextWeight::Semibold) => "font-semibold",
            Some(TextWeight::Bold) => "font-bold",
            _ => "font-normal",
        },
        match props.multiline {
            Some(TextMultiline::Multiline) => "",
            Some(TextMultiline::Clamp2) => "line-clamp-2",
            Some(TextMultiline::Clamp3) => "line-clamp-3",
            Some(TextMultiline::Clamp4) => "line-clamp-4",
            _ => "truncate",
        },
        match props.center {
            Some(true) => "text-center",
            _ => "",
        },
        props.class
    );

    rsx! {
        div { class, {props.children} }
    }
}
