#![allow(non_snake_case)]
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

mod components;
mod routes;
mod util;

use dioxus::prelude::*;
use routes::Route;
use tailwind_fuse::tw_merge;
use tracing::Level;
use util::state::{AppState, Theme};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let initial_state = use_resource(move || async move { AppState::new().await });

    // TODO: Store state in config file and read to here
    let themed_class = tw_merge!("dark");

    rsx! {
        match (initial_state.value())() {
            Some(Ok(state)) => rsx! {
                Content { state }
            },
            Some(Err(e)) => rsx! {
                div {
                    class:themed_class,
                    "An error occurred: {e}"
                }
            },
            None => rsx! {
                div {
                    class: themed_class,
                    "Loading"
                }
            }
        }
    }
}

#[component]
fn Content(state: AppState) -> Element {
    let app_state = use_signal::<AppState>(|| state);

    use_context_provider::<Signal<AppState>>(|| app_state);

    let class = tw_merge!(
        "flex grow min-h-screen w-full",
        match app_state.read().theme {
            Theme::Light => "",
            Theme::Dark => "dark",
        }
    );

    rsx! {
        div { class, id: "app", Router::<Route> {} }
    }
}
