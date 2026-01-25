use dioxus::prelude::*;
use crate::components::{Sidebar, Header, DataTable, Column};
use crate::api;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SystemUpdate {
    pub id: u64,
    pub version: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[component]
pub fn SystemUpdates() -> Element {
    let mut updates = use_signal(|| Vec::<SystemUpdate>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);
    let mut message = use_signal(|| Option::<String>::None);

    let fetch_data = move || {
        spawn(async move {
            loading.set(true);
            match api::get::<ApiResponse<Vec<SystemUpdate>>>("/system-updates").await {
                Ok(resp) => {
                    if let Some(data) = resp.data {
                        updates.set(data);
                    }
                }
                Err(e) => error.set(Some(e.to_string())),
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let handle_check_update = move |_| {
        spawn(async move {
            // This is a placeholder action
            loading.set(true);
            // Simulate check
            gloo_timers::future::TimeoutFuture::new(1000).await;
            message.set(Some("System is up to date.".to_string()));
            loading.set(false);
            
            // Clear message after 3 seconds
            gloo_timers::future::TimeoutFuture::new(3000).await;
            message.set(None);
        });
    };

    let columns = vec![
        Column::new("ID", "id"),
        Column::new("Version", "version"),
        Column::new("Status", "status"),
        Column::new("Date", "created_at"),
    ];

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "System Updates" }
                    div { class: "space-x-3",
                        button {
                            class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg flex items-center",
                            onclick: handle_check_update,
                            "Check for Updates"
                        }
                    }
                }

                if let Some(msg) = message.read().as_ref() {
                    div { class: "mb-4 p-4 bg-green-100 text-green-700 rounded-md",
                        "{msg}"
                    }
                }

                if let Some(err) = error.read().as_ref() {
                    div { class: "mb-4 p-4 bg-red-100 text-red-700 rounded-md",
                        "{err}"
                    }
                }

                DataTable {
                    columns: columns,
                    loading: *loading.read(),
                    for u in updates.read().iter() {
                        {
                            let u = u.clone();
                            rsx! {
                                tr { key: "{u.id}", class: "hover:bg-gray-50",
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{u.id}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 font-medium", "{u.version}" }
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        span {
                                            class: if u.status == "applied" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800" },
                                            "{u.status}"
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{u.created_at}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
