#![allow(non_snake_case)]
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

mod components;
mod routes;
mod state;
mod util;

use components::ui::Toast;
use dioxus::prelude::*;
use routes::Route;
use state::*;
use tailwind_fuse::tw_merge;
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let fedimint_instance = use_resource(move || async move { Fedimint::new().await });

    // TODO: Store state in config file and read to here
    let themed_class = tw_merge!("dark");

    rsx! {
        match (fedimint_instance.value())() {
            Some(Ok(fedimint)) => rsx! {
                Content { fedimint }
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
fn Content(fedimint: Fedimint) -> Element {
    let state = use_signal::<AppState>(AppState::new);
    let fedimint = use_signal::<Fedimint>(|| fedimint);

    use_context_provider::<Signal<AppState>>(|| state);
    use_context_provider::<Signal<Fedimint>>(|| fedimint);

    let class = tw_merge!(
        "flex grow min-h-screen w-full",
        match state().theme {
            Theme::Light => "",
            Theme::Dark => "dark",
        }
    );

    rsx! {
        div { class, id: "app", Router::<Route> {} }
        Toast {}
    }
}
