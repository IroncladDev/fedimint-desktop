use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdEllipsisVertical, LdLogOut, LdQrCode};
use dioxus_free_icons::Icon;
use multimint::types::InfoResponse;
use tailwind_fuse::tw_merge;

use crate::components::ui::*;
use crate::util::meta::get_federation_icon;
use crate::util::state::AppState;

#[component]
pub fn FederationItem(info: InfoResponse) -> Element {
    let mut qr_open = use_signal(|| false);
    let mut state = use_context::<Signal<AppState>>();

    let class = tw_merge!(
        "p-1 flex gap-2 items-center rounded-xl bg-background hover:bg-secondary cursor-pointer ring-offset-background transition-colors focus-fixible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
        if state().active_federation_id == Some(info.federation_id) {
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
    let icon: String = get_federation_icon(info.clone(), Some(state().theme.clone()));

    let switch_active_federation = move |_| {
        state.write().active_federation_id = Some(info.federation_id);
    };

    let remove_federation = move |_| {
        spawn(async move {
            // Does not actually write to the db yet. Waiting on next version of multimint
            state.write().multimint.remove(&info.federation_id).await;
            state.write().federations.remove(&info.federation_id);

            if info.federation_id == state().active_federation_id.unwrap() {
                state.write().active_federation_id = None;
            }
        });
    };

    rsx! {
        div { class, onclick: switch_active_federation,
            img {
                class: "border rounded-lg aspect-square bg-white",
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
                        if info.meta.contains_key("invite_code") {
                            FederationOption { onclick: move |_| { qr_open.set(!qr_open()) },
                                Icon { width: 16, height: 16, class: "text-foreground", icon: LdQrCode }
                                Text { size: TextSize::Sm, "Invite" }
                            }
                        }
                        FederationOption { onclick: remove_federation,
                            Icon { width: 16, height: 16, class: "text-foreground", icon: LdLogOut }
                            Text { size: TextSize::Sm, "Leave Federation" }
                        }
                    }
                }
            }
        }
        if let Some(invite_code) = info.meta.get("invite_code") {
            Dialog { open: qr_open, title: "Invite",
                QRCode { value: invite_code }
            }
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
