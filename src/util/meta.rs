use multimint::types::InfoResponse;

use crate::Theme;

pub fn get_federation_icon(info: InfoResponse, theme: Option<Theme>) -> String {
    if let Some(i) = info.meta.get("fedi:federation_icon_url") {
        i.to_string()
    } else if let Some(i) = info.meta.get("federation_icon_url") {
        i.to_string()
    } else if theme == Some(Theme::Light) {
        "federation-light.png".to_string()
    } else {
        "federation-dark.png".to_string()
    }
}

pub fn get_federation_name(info: InfoResponse) -> String {
    if let Some(n) = info.meta.get("federation_name") {
        n.to_string()
    } else {
        info.federation_id.to_string().chars().take(6).collect()
    }
}
