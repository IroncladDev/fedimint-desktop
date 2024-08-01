#![allow(dead_code)]
use crate::util::types::{Tab, Toast};

#[derive(Clone, Debug)]
pub struct AppState {
    pub tab: Tab,
    pub toast: Toast,
    pub sidebar_open: bool,
    pub theme: Theme,
}

impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        self.tab == other.tab
            && self.toast == other.toast
            && self.sidebar_open == other.sidebar_open
            && self.theme == other.theme
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            tab: Tab::Admin,
            toast: Toast::new(),
            theme: Theme::Dark,
            sidebar_open: true,
        }
    }

    pub fn switch_tab(&mut self, tab: Tab) {
        self.tab = tab;
    }
}
