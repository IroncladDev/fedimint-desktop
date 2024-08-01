use crate::util::state::AppState;
use dioxus::prelude::*;
use std::time::Duration;

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
