#![allow(dead_code)]
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(Props, Clone, PartialEq)]
pub struct FlexProps {
    pub children: Element,
    pub class: Option<String>,
    pub row: Option<bool>,
    pub col: Option<bool>,
    pub center: Option<bool>,
    pub grow: Option<bool>,
    pub wrap: Option<bool>,
    pub no_basis: Option<bool>,
    pub align: Option<FlexPosition>,
    pub justify: Option<FlexPosition>,
    pub gap: Option<i32>,
    pub p: Option<i32>,
    pub shrink: Option<bool>,
    pub width: Option<FlexDimension>,
    pub height: Option<FlexDimension>,
}

#[derive(Clone, PartialEq)]
pub enum FlexPosition {
    Unset,
    Start,
    End,
    Center,
    Between,
}

#[derive(Clone, PartialEq)]
pub enum FlexDimension {
    Unset,
    Full,
    Auto,
}

#[component]
pub fn Flex(props: FlexProps) -> Element {
    let class = tw_merge!(
        "flex flex-row",
        match props.align {
            Some(FlexPosition::Start) => "items-start",
            Some(FlexPosition::End) => "items-end",
            Some(FlexPosition::Center) => "items-center",
            _ => "",
        },
        match props.justify {
            Some(FlexPosition::Start) => "justify-start",
            Some(FlexPosition::End) => "justify-end",
            Some(FlexPosition::Center) => "justify-center",
            Some(FlexPosition::Between) => "justify-between",
            _ => "",
        },
        match props.gap {
            Some(g) => match g {
                0 => "gap-0".to_string(),
                1 => "gap-1".to_string(),
                2 => "gap-2".to_string(),
                4 => "gap-4".to_string(),
                6 => "gap-6".to_string(),
                8 => "gap-8".to_string(),
                12 => "gap-12".to_string(),
                16 => "gap-16".to_string(),
                _ => format!("gap-{}", g),
            },
            _ => "".to_string(),
        },
        match props.p {
            Some(p) => match p {
                0 => "p-0".to_string(),
                1 => "p-1".to_string(),
                2 => "p-2".to_string(),
                4 => "p-4".to_string(),
                6 => "p-6".to_string(),
                8 => "p-8".to_string(),
                12 => "p-12".to_string(),
                16 => "p-16".to_string(),
                _ => format!("p-{}", p),
            },
            _ => "".to_string(),
        },
        match props.row {
            Some(true) => "flex-row",
            _ => "",
        },
        match props.col {
            Some(true) => "flex-col",
            _ => "",
        },
        match props.grow {
            Some(true) => "grow",
            _ => "",
        },
        match props.wrap {
            Some(true) => "flex-wrap",
            _ => "",
        },
        match props.no_basis {
            Some(true) => "basis-0",
            _ => "",
        },
        match props.center {
            Some(true) => "justify-center items-center",
            _ => "",
        },
        match props.shrink {
            Some(true) => "flex-shrink",
            Some(false) => "flex-shrink-0",
            _ => "",
        },
        match props.width {
            Some(FlexDimension::Full) => "w-full",
            Some(FlexDimension::Auto) => "w-auto",
            _ => "",
        },
        match props.height {
            Some(FlexDimension::Full) => "h-full",
            Some(FlexDimension::Auto) => "h-auto",
            _ => "",
        },
        props.class
    );

    rsx! {
        div { class, {props.children} }
    }
}
