#![allow(non_snake_case)]
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

mod components;
mod routes;
mod util;

use dioxus::prelude::*;
use routes::Route;
use tracing::Level;
use util::state::AppState;

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

    rsx! {
        match (initial_state.value())() {
            Some(Ok(state)) => rsx! {
                Content { state }
            },
            Some(Err(e)) => rsx! {
                div {
                    "An error occurred: {e}"
                }
            },
            None => rsx! {
                div {
                    "Loading"
                }
            }
        }
    }
}

#[component]
fn Content(state: AppState) -> Element {
    use_context_provider::<Signal<AppState>>(|| Signal::new(state));

    rsx! {
        Router::<Route> {}
    }
}
