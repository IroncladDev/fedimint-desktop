use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdEllipsisVertical, LdLogOut, LdQrCode};
use dioxus_free_icons::Icon;
use multimint::types::InfoResponse;
use tailwind_fuse::tw_merge;

use crate::components::dialog::Dialog;
use crate::components::ui::*;
use crate::util::state::{AppState, Theme};

#[component]
pub fn FederationItem(info: InfoResponse) -> Element {
    let mut qr_open = use_signal(|| false);
    let mut state = use_context::<Signal<AppState>>();

    let class = tw_merge!(
        "p-1 flex gap-2 items-center rounded bg-background hover:bg-secondary cursor-pointer ring-offset-background transition-colors focus-fixible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
        if state.read().active_federation_id == Some(info.federation_id) {
            "bg-secondary"
        } else {
            ""
        }
    );

    let name: String = if let Some(n) = info.meta.get("federation_name") {
        n.to_string()
    } else {
        info.federation_id.to_string().chars().take(6).collect()
    };

    // TODO: fetch and populate meta from external URL
    let icon: String = if let Some(i) = info.meta.get("fedi:federation_icon_url") {
        i.to_string()
    } else if let Some(i) = info.meta.get("federation_icon_url") {
        i.to_string()
    } else if state.read().theme == Theme::Light {
        "federation-light.png".to_string()
    } else {
        "federation-dark.png".to_string()
    };

    let switch_active_federation = move |_| {
        state.write().active_federation_id = Some(info.federation_id);
    };

    rsx! {
        div { class, onclick: switch_active_federation,
            img {
                class: "border rounded-lg",
                src: "{icon}",
                alt: "Federation Icon",
                width: 36,
                height: 36
            }
            Text { class: "grow", "{name}" }
            Popover {
                PopoverTrigger {
                    Icon {
                        width: 16,
                        height: 16,
                        class: "text-muted-foreground",
                        icon: LdEllipsisVertical
                    }
                }
                PopoverContent {
                    Flex { col: true,
                        FederationOption { onclick: move |_| { qr_open.set(!qr_open()) },
                            Icon { width: 16, height: 16, class: "text-foreground", icon: LdQrCode }
                            Text { size: TextSize::Sm, "Invite" }
                        }
                        FederationOption { onclick: move |_| {},
                            Icon { width: 16, height: 16, class: "text-foreground", icon: LdLogOut }
                            Text { size: TextSize::Sm, "Leave Federation" }
                        }
                    }
                }
            }
        }
        Dialog { open: qr_open, title: "Invite",
            QRCode { value: info.federation_id }
        }
    }
}

#[component]
pub fn FederationOption(children: Element, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "flex gap-2 items-center rounded bg-popover text-popover-foreground hover:bg-muted transition-colors w-full px-2 py-1",
            onclick: move |e| onclick.call(e),
            {children}
        }
    }
}
