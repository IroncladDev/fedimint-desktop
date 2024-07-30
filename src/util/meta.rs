use multimint::types::InfoResponse;

use super::state::Theme;

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
