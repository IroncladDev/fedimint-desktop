use dioxus::prelude::*;

use crate::components::ui::*;
use crate::components::widget::Widget;
use crate::state::*;

#[component]
pub fn Operations() -> Element {
    let fedimint = use_context::<Signal<Fedimint>>();

    let operations = use_resource(move || async move {
        let client = fedimint().get_multimint_client().await.unwrap();

        client.operation_log().list_operations(24, None).await
    });

    rsx! {
        match &*operations.read_unchecked() {
            Some(ops) => rsx! {
                Widget { title: "Operations",
                    Flex { col: true, gap: 2,
                        if ops.is_empty() {
                            Text { weight: TextWeight::Medium, "No operations" }
                        } else {
                            for (id , op) in ops.iter() {
                                Flex { gap: 2,
                                    col: true,
                                    p: 2,
                                    class: "border rounded-md",
                                    Flex { col: true, gap: 1,
                                        Text { weight: TextWeight::Medium, "ID" }
                                        CopyValue { value: "{id.operation_id.to_string()}" },
                                    }
                                    Flex { col: true, gap: 1,
                                        Text { weight: TextWeight::Medium, "Type" }
                                        CopyValue { value: "{op.operation_module_kind()}" },
                                    }
                                }
                            }
                        }
                    }
                }
            },
            None => rsx! {
                Widget { title: "Operations",
                    "Loading"
                }
            },
        }
    }
}
