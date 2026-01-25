use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api;
use crate::components::{Sidebar, Header, DataTable, Column, Modal};
use std::collections::HashSet;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Role {
    pub id: u64,
    pub name: String,
    pub guard_name: String,
    pub label: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Permission {
    pub id: u64,
    pub name: String,
    pub guard_name: String,
}

#[component]
pub fn Roles() -> Element {
    let mut roles = use_signal(|| Vec::<Role>::new());
    let mut permissions = use_signal(|| Vec::<Permission>::new());
    let mut loading = use_signal(|| true);
    let mut show_create_modal = use_signal(|| false);
    
    // Permissions Modal State
    let mut show_permissions_modal = use_signal(|| false);
    let mut selected_role = use_signal(|| Option::<Role>::None);
    let mut selected_permissions = use_signal(|| HashSet::<String>::new());
    let mut permissions_loading = use_signal(|| false);

    let fetch_data = move || {
        spawn(async move {
            loading.set(true);
            // Fetch roles
            if let Ok(data) = api::get::<Vec<Role>>("/roles").await {
                roles.set(data);
            }
            // Fetch all permissions
            if let Ok(data) = api::get::<Vec<Permission>>("/permissions").await {
                permissions.set(data);
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let open_permissions_modal = move |role: Role| {
        spawn(async move {
            selected_role.set(Some(role.clone()));
            permissions_loading.set(true);
            show_permissions_modal.set(true);
            
            // Fetch permissions for this role
            let url = format!("/roles/{}/permissions", role.id);
            if let Ok(perms) = api::get::<Vec<String>>(&url).await {
                selected_permissions.set(perms.into_iter().collect());
            } else {
                selected_permissions.set(HashSet::new());
            }
            permissions_loading.set(false);
        });
    };

    let mut toggle_permission = move |perm_name: String| {
        let mut current = selected_permissions.read().clone();
        if current.contains(&perm_name) {
            current.remove(&perm_name);
        } else {
            current.insert(perm_name);
        }
        selected_permissions.set(current);
    };

    let save_permissions = move |_| {
        spawn(async move {
            if let Some(role) = selected_role.read().as_ref() {
                permissions_loading.set(true);
                
                #[derive(Serialize)]
                struct StorePermissionRequest {
                    role: String,
                    permissions: Vec<String>,
                }
                
                let body = StorePermissionRequest {
                    role: role.name.clone(),
                    permissions: selected_permissions.read().iter().cloned().collect(),
                };
                
                match api::post::<serde_json::Value, _>("/permissions/assign", &body).await {
                    Ok(_) => {
                        show_permissions_modal.set(false);
                    }
                    Err(e) => {
                        // Handle error (maybe show toast/alert)
                        println!("Error saving permissions: {}", e);
                    }
                }
                permissions_loading.set(false);
            }
        });
    };

    let columns = vec![
        Column { key: "id".to_string(), label: "ID".to_string(), sortable: true },
        Column { key: "name".to_string(), label: "Name".to_string(), sortable: true },
        Column { key: "guard_name".to_string(), label: "Guard".to_string(), sortable: true },
        Column { key: "label".to_string(), label: "Label".to_string(), sortable: false },
    ];

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Roles" }
                    button {
                        class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700",
                        onclick: move |_| show_create_modal.set(true),
                        "+ Add Role"
                    }
                }
                
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for role in roles.read().iter() {
                        tr { key: "{role.id}",
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{role.id}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{role.name}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{role.guard_name}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{role.label.clone().unwrap_or_default()}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                button {
                                    class: "text-indigo-600 hover:text-indigo-900 mr-3",
                                    onclick: {
                                        let role = role.clone();
                                        move |_| open_permissions_modal(role.clone())
                                    },
                                    "Permissions"
                                }
                                button {
                                    class: "text-blue-600 hover:text-blue-900 mr-3",
                                    "Edit"
                                }
                                button {
                                    class: "text-red-600 hover:text-red-900",
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            }
            
            Modal {
                title: "Create Role".to_string(),
                is_open: *show_create_modal.read(),
                on_close: move |_| show_create_modal.set(false),
                RoleForm {
                    on_submit: move |_| {
                        show_create_modal.set(false);
                        fetch_data();
                    },
                }
            }

            Modal {
                title: format!("Manage Permissions: {}", selected_role.read().as_ref().map(|r| r.name.clone()).unwrap_or_default()),
                is_open: *show_permissions_modal.read(),
                on_close: move |_| show_permissions_modal.set(false),
                div { class: "p-4",
                    if *permissions_loading.read() {
                        div { class: "text-center py-4", "Loading permissions..." }
                    } else {
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 max-h-96 overflow-y-auto",
                            for perm in permissions.read().iter() {
                                div { class: "flex items-center",
                                    input {
                                        r#type: "checkbox",
                                        class: "h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500",
                                        checked: selected_permissions.read().contains(&perm.name),
                                        onchange: {
                                            let perm_name = perm.name.clone();
                                            move |_| toggle_permission(perm_name.clone())
                                        }
                                    }
                                    label { class: "ml-2 text-sm text-gray-700", "{perm.name}" }
                                }
                            }
                        }
                        div { class: "mt-6 flex justify-end space-x-3",
                            button {
                                class: "px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300",
                                onclick: move |_| show_permissions_modal.set(false),
                                "Cancel"
                            }
                            button {
                                class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700",
                                onclick: save_permissions,
                                "Save Changes"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RoleForm(on_submit: EventHandler<()>) -> Element {
    let mut name = use_signal(|| String::new());
    let mut label = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut error_message = use_signal(|| Option::<String>::None);

    let handle_submit = move |_evt: Event<FormData>| {
        let name_val = name.read().clone();
        let label_val = label.read().clone();
        error_message.set(None);
        
        spawn(async move {
            loading.set(true);
            
            #[derive(Serialize)]
            struct CreateRole {
                name: String,
                guard_name: String,
                label: String,
            }
            
            let body = CreateRole {
                name: name_val,
                guard_name: "web".to_string(), // Default to web
                label: label_val,
            };
            
            match api::post::<serde_json::Value, _>("/roles", &body).await {
                Ok(_) => on_submit.call(()),
                Err(e) => error_message.set(Some(e)),
            }
            
            loading.set(false);
        });
    };

    rsx! {
        form { onsubmit: handle_submit,
            if let Some(msg) = error_message.read().as_ref() {
                div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4",
                    "{msg}"
                }
            }
            div { class: "mb-4",
                label { class: "block text-sm font-medium text-gray-700 mb-1", "Name (e.g., 'super-admin')" }
                input {
                    r#type: "text",
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    value: "{name}",
                    oninput: move |evt| name.set(evt.value()),
                    required: true,
                }
            }
            div { class: "mb-4",
                label { class: "block text-sm font-medium text-gray-700 mb-1", "Label (Readable Name)" }
                input {
                    r#type: "text",
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    value: "{label}",
                    oninput: move |evt| label.set(evt.value()),
                }
            }
            div { class: "flex justify-end space-x-3",
                button {
                    r#type: "submit",
                    class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50",
                    disabled: *loading.read(),
                    if *loading.read() { "Creating..." } else { "Create" }
                }
            }
        }
    }
}
