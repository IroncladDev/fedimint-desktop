use std::collections::{BTreeMap, HashMap};

use add_federation_dialog::AddFederationDialog;
use dioxus::prelude::*;
use federation_item::FederationItem;
use serde::Deserialize;

use crate::{components::ui::*, util::state::AppState};
mod add_federation_dialog;
mod empty;
mod federation_item;

use empty::Empty;

#[component]
pub fn Sidebar() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut add_federation_dialog = use_signal::<bool>(|| false);

    let federations = state.read().federations.clone();

    use_effect(move || {
        spawn(async move {
            let federation_info = state.read().multimint.info().await;
            if let Ok(info) = federation_info {
                state.write().federations = info.clone();
                if let Some((federation_id, _)) = info.first_key_value() {
                    state.write().active_federation_id = Some(*federation_id);
                }
            }
        });
    });

    use_effect(move || {
        spawn(async move {
            let mut meta_map: HashMap<String, MetaResponse> = HashMap::new();

            let mut updates = Vec::new();

            // First, collect all the data we need
            {
                for (federation_id, info) in state.read().federations.iter() {
                    updates.push((federation_id.clone().to_owned(), info.clone()));
                }
            } // federations_read is dropped here

            for (federation_id, info) in updates {
                let meta_external_url = info.meta.get("meta_external_url");

                if let Some(meta_url) = meta_external_url {
                    if !meta_url.is_empty() && !meta_map.contains_key(meta_url) {
                        let meta = fetch_meta(meta_url).await;

                        if let Ok(meta) = meta {
                            meta_map.insert(meta_url.to_string(), meta);
                        }
                    }

                    if let Some(meta) = meta_map.get(meta_url) {
                        match meta {
                            MetaResponse::Json(meta_json) => {
                                let meta_json = meta_json.clone();

                                state
                                    .write()
                                    .federations
                                    .get_mut(&federation_id)
                                    .unwrap()
                                    .meta = meta_json;
                            }
                            MetaResponse::Tree(meta_tree) => {
                                let federation_id_string = federation_id.to_string();
                                let meta_item = meta_tree.get(&federation_id_string);

                                if let Some(meta) = meta_item {
                                    let meta_json = meta.clone();

                                    state
                                        .write()
                                        .federations
                                        .get_mut(&federation_id)
                                        .unwrap()
                                        .meta = meta_json;
                                }
                            }
                        }
                    }
                }
            }
        });
    });

    let visibility = if state.read().sidebar_open {
        "visible"
    } else {
        "hidden"
    };

    rsx! {
        Flex {
            col: true,
            gap: 2,
            p: 2,
            class: "min-w-[300px] border-r {visibility}",
            if federations.is_empty() {
                Empty { add_federation_dialog }
            } else {
                Flex { col: true, grow: true,
                    Flex { col: true, gap: 2, class: "inset-0 overflow-y-auto",
                        for (_ , info) in federations {
                            FederationItem { info: info.clone() }
                        }
                    }
                }
                Flex { 
                    Button { class: "w-full", onclick: move |_| add_federation_dialog.set(true), "Add a Federation" }
                }
            }
        }
        AddFederationDialog { add_federation_dialog }
    }
}

type MetaJson = BTreeMap<String, String>;
type MetaTree = BTreeMap<String, MetaJson>;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum MetaResponse {
    Json(MetaJson),
    Tree(MetaTree),
}

async fn fetch_meta(url: &str) -> Result<MetaResponse, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let meta: MetaResponse = response.json().await?;
    Ok(meta)
}
