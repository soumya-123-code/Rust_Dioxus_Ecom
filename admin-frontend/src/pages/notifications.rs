use dioxus::prelude::*;
use crate::components::{Sidebar, Header, DataTable, Column, Modal};
use crate::api;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Notification {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub target_type: Option<String>,
    pub target_id: Option<u64>,
    pub created_at: Option<String>, // Using String for simplicity in frontend
}

#[component]
pub fn Notifications() -> Element {
    let mut notifications = use_signal(|| Vec::<Notification>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);
    
    // Modal states
    let mut show_create_modal = use_signal(|| false);
    let mut saving = use_signal(|| false);

    // Form signals
    let mut title = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    let mut notification_type = use_signal(|| "system".to_string());
    let mut target_type = use_signal(|| "all".to_string());
    let mut target_id = use_signal(|| "".to_string());

    let fetch_data = move || {
        spawn(async move {
            loading.set(true);
            match api::get::<Vec<Notification>>("/notifications").await {
                Ok(data) => notifications.set(data),
                Err(e) => error.set(Some(e.to_string())),
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let mut reset_form = move || {
        title.set(String::new());
        message.set(String::new());
        notification_type.set("system".to_string());
        target_type.set("all".to_string());
        target_id.set("".to_string());
    };

    let open_create_modal = move |_| {
        reset_form();
        show_create_modal.set(true);
    };

    let handle_send = move |_| {
        spawn(async move {
            saving.set(true);
            
            let tid = if target_id().is_empty() {
                None
            } else {
                target_id().parse::<u64>().ok()
            };

            let req = serde_json::json!({
                "title": title(),
                "message": message(),
                "notification_type": notification_type(),
                "target_type": if target_type() == "all" { None } else { Some(target_type()) },
                "target_id": tid
            });

            match api::post::<serde_json::Value, _>("/notifications/send", &req).await {
                Ok(_) => {
                    show_create_modal.set(false);
                    fetch_data();
                }
                Err(e) => println!("Error sending notification: {}", e),
            }
            saving.set(false);
        });
    };

    let columns = vec![
        Column::new("ID", "id"),
        Column::new("Title", "title"),
        Column::new("Message", "message"),
        Column::new("Type", "notification_type"),
        Column::new("Target", "target_type"),
        Column::new("Date", "created_at"),
    ];

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Notifications" }
                    button {
                        class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg flex items-center",
                        onclick: open_create_modal,
                        "Send Notification"
                    }
                }

                DataTable {
                    columns: columns,
                    loading: *loading.read(),
                    for n in notifications.read().iter() {
                        {
                            rsx! {
                                tr { key: "{n.id}", class: "hover:bg-gray-50",
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{n.id}" }
                                    td { class: "px-6 py-4 text-sm text-gray-900 font-medium", "{n.title}" }
                                    td { class: "px-6 py-4 text-sm text-gray-500 max-w-xs truncate", "{n.message}" }
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        span {
                                            class: "px-2 py-1 text-xs rounded-full bg-blue-100 text-blue-800",
                                            "{n.notification_type}"
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        if let Some(t) = &n.target_type {
                                            "{t}"
                                        } else {
                                            "All Users"
                                        }
                                        if let Some(id) = n.target_id {
                                            " (ID: {id})"
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "{n.created_at.clone().unwrap_or_default()}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Create Modal
        Modal {
            title: "Send Notification".to_string(),
            is_open: *show_create_modal.read(),
            on_close: move |_| show_create_modal.set(false),
            div { class: "space-y-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Title" }
                    input {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        r#type: "text",
                        value: "{title}",
                        oninput: move |e| title.set(e.value())
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Message" }
                    textarea {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        rows: "4",
                        value: "{message}",
                        oninput: move |e| message.set(e.value())
                    }
                }
                div { class: "grid grid-cols-2 gap-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Type" }
                        select {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{notification_type}",
                            onchange: move |e| notification_type.set(e.value()),
                            option { value: "system", "System" }
                            option { value: "promotional", "Promotional" }
                            option { value: "alert", "Alert" }
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Target Audience" }
                        select {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{target_type}",
                            onchange: move |e| target_type.set(e.value()),
                            option { value: "all", "All Users" }
                            option { value: "customer", "Customers" }
                            option { value: "seller", "Sellers" }
                            option { value: "specific_user", "Specific User" }
                        }
                    }
                }
                
                if target_type() == "specific_user" {
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Target User ID" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            r#type: "number",
                            value: "{target_id}",
                            oninput: move |e| target_id.set(e.value())
                        }
                    }
                }

                div { class: "flex justify-end pt-4",
                    button {
                        class: "mr-3 px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50",
                        onclick: move |_| show_create_modal.set(false),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 disabled:opacity-50",
                        onclick: handle_send,
                        disabled: *saving.read(),
                        if *saving.read() { "Sending..." } else { "Send Notification" }
                    }
                }
            }
        }
    }
}
