use crate::state::AppState;
use dioxus::prelude::*;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

#[component]
pub fn Toast() -> Element {
    let mut state = use_context::<Signal<AppState>>();

    let mut refresh = use_resource(move || async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        state.write().toast.hide();
    });

    use_effect(move || {
        if state().toast.visible {
            refresh.restart();
        }
    });

    let visibility = if state().toast.visible {
        "opacity-100 scale-y-1 translate-y-0"
    } else {
        "opacity-0 scale-y-0 translate-y-full"
    };

    let toast_key = state().toast.timestamp;

    rsx! {
        div { key: "{toast_key}", class: "fixed bg-card border-2 px-4 py-2 rounded-lg transition-all {visibility} right-[20px] bottom-[20px] text-foreground-highest z-50 shadow-lg",
            if let Some(ms) = &state().toast.content {
                "{ms}"
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ToastController {
    pub content: Option<String>,
    pub timestamp: u128,
    pub visible: bool,
}

impl ToastController {
    pub fn new() -> Self {
        let timestamp = Self::get_timestamp();

        ToastController {
            content: None,
            visible: false,
            timestamp,
        }
    }

    pub fn show(&mut self, message: String) {
        self.content = Some(message);
        self.visible = true;
        self.timestamp = Self::get_timestamp();
    }

    pub fn hide(&mut self) {
        self.visible = false;
        self.timestamp = Self::get_timestamp();
    }

    fn get_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}
