#![allow(dead_code)]

use crate::components::ui::ToastController;

#[derive(Clone, Debug)]
pub struct AppState {
    pub tab: Tab,
    pub toast: ToastController,
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

#[derive(PartialEq, Clone, Debug)]
pub enum Tab {
    Admin,
    Lightning,
    Mint,
    Onchain,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            tab: Tab::Admin,
            toast: ToastController::new(),
            theme: Theme::Dark,
            sidebar_open: true,
        }
    }

    pub fn switch_tab(&mut self, tab: Tab) {
        self.tab = tab;
    }

    pub fn toast(&mut self, message: String) {
        self.toast.show(message);
    }
}
