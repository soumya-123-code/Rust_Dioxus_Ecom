use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api;
use crate::components::{Sidebar, Header};

#[derive(Debug, Clone, Deserialize)]
pub struct Setting {
    pub id: u64,
    pub variable: String,
    pub value: Option<String>,
}

#[component]
pub fn Settings() -> Element {
    let mut settings = use_signal(|| Vec::<Setting>::new());
    let mut loading = use_signal(|| true);
    let mut saving = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            #[derive(Deserialize)]
            struct Response { data: Vec<Setting> }
            match api::get::<Response>("/settings").await {
                Ok(resp) => settings.set(resp.data),
                Err(_) => {}
            }
            loading.set(false);
        });
    });

    let handle_save = move |_| {
        let current_settings = settings.read().clone();
        spawn(async move {
            saving.set(true);
            #[derive(Serialize)]
            struct SaveRequest { settings: Vec<SettingValue> }
            #[derive(Serialize)]
            struct SettingValue { variable: String, value: String }
            
            let values: Vec<SettingValue> = current_settings.iter().map(|s| SettingValue {
                variable: s.variable.clone(),
                value: s.value.clone().unwrap_or_default(),
            }).collect();
            
            let _ = api::post::<serde_json::Value, _>("/settings", &SaveRequest { settings: values }).await;
            saving.set(false);
        });
    };

    let current_settings = settings.read().clone();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Settings" }
                    button {
                        class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50",
                        disabled: *saving.read(),
                        onclick: handle_save,
                        if *saving.read() { "Saving..." } else { "Save Changes" }
                    }
                }
                
                if *loading.read() {
                    div { class: "text-center py-8", p { class: "text-gray-500", "Loading..." } }
                } else {
                    div { class: "bg-white rounded-lg shadow",
                        div { class: "p-6 space-y-4",
                            for (idx, setting) in current_settings.iter().enumerate() {
                                div { key: "{setting.id}", class: "grid grid-cols-3 gap-4 items-center py-2 border-b border-gray-100 last:border-0",
                                    label { class: "text-sm font-medium text-gray-700", "{setting.variable}" }
                                    input {
                                        r#type: "text",
                                        class: "col-span-2 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                        value: "{setting.value.clone().unwrap_or_default()}",
                                        oninput: move |evt| {
                                            let mut s = settings.write();
                                            if let Some(item) = s.get_mut(idx) {
                                                item.value = Some(evt.value());
                                            }
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
