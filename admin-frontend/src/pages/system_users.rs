use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, Modal, ConfirmDialog};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SystemUser {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub status: Option<String>,
    pub password: Option<String>,
}

#[component]
pub fn SystemUsers() -> Element {
    let mut users = use_signal(|| Vec::<SystemUser>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;
    
    let mut show_create_modal = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| false);
    let mut show_delete_dialog = use_signal(|| false);
    let mut selected_user = use_signal(|| None::<SystemUser>);
    
    // Form signals
    let mut name = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut mobile = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut status = use_signal(|| "active".to_string());
    let mut saving = use_signal(|| false);

    let fetch_data = move || {
        let current_page = *page.read();
        spawn(async move {
            loading.set(true);
            match api::get::<PaginatedResponse<SystemUser>>(&format!("/system-users?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => {
                    users.set(data.data);
                    total.set(data.total);
                }
                Err(e) => println!("Failed to fetch users: {}", e),
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let mut reset_form = move || {
        name.set("".to_string());
        email.set("".to_string());
        mobile.set("".to_string());
        password.set("".to_string());
        status.set("active".to_string());
        selected_user.set(None);
    };

    let open_create_modal = move |_| {
        reset_form();
        show_create_modal.set(true);
    };

    let mut open_edit_modal = move |user: SystemUser| {
        selected_user.set(Some(user.clone()));
        name.set(user.name);
        email.set(user.email);
        mobile.set(user.mobile);
        status.set(user.status);
        password.set("".to_string()); // Don't show password
        show_edit_modal.set(true);
    };

    let mut open_delete_dialog = move |user: SystemUser| {
        selected_user.set(Some(user));
        show_delete_dialog.set(true);
    };

    let handle_create = move |_| {
        spawn(async move {
            saving.set(true);
            let req = CreateUserRequest {
                name: name.read().clone(),
                email: email.read().clone(),
                mobile: mobile.read().clone(),
                password: password.read().clone(),
            };

            match api::post::<serde_json::Value, _>("/system-users", &req).await {
                Ok(_) => {
                    show_create_modal.set(false);
                    fetch_data();
                }
                Err(e) => println!("Failed to create user: {}", e),
            }
            saving.set(false);
        });
    };

    let handle_update = move |_| {
        if let Some(user) = selected_user.read().clone() {
            spawn(async move {
                saving.set(true);
                let pwd = password.read().clone();
                let req = UpdateUserRequest {
                    name: Some(name.read().clone()),
                    email: Some(email.read().clone()),
                    mobile: Some(mobile.read().clone()),
                    status: Some(status.read().clone()),
                    password: if pwd.is_empty() { None } else { Some(pwd) },
                };

                match api::put::<serde_json::Value, _>(&format!("/system-users/{}", user.id), &req).await {
                    Ok(_) => {
                        show_edit_modal.set(false);
                        fetch_data();
                    }
                    Err(e) => println!("Failed to update user: {}", e),
                }
                saving.set(false);
            });
        }
    };

    let handle_delete = move |_| {
        if let Some(user) = selected_user.read().clone() {
            spawn(async move {
                match api::delete(&format!("/system-users/{}", user.id)).await {
                    Ok(_) => {
                        show_delete_dialog.set(false);
                        fetch_data();
                    }
                    Err(e) => println!("Failed to delete user: {}", e),
                }
            });
        }
    };

    let columns = vec![
        Column::new("ID", "id"),
        Column::new("Name", "name"),
        Column::new("Email", "email"),
        Column::new("Mobile", "mobile"),
        Column::new("Status", "status"),
        Column { key: "actions".to_string(), label: "Actions".to_string(), sortable: false },
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;
    let current_users = users.read().clone();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "System Users" }
                    button {
                        class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700",
                        onclick: open_create_modal,
                        "+ Add User"
                    }
                }
                
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for user in current_users {
                        {
                            let user_edit = user.clone();
                            let user_delete = user.clone();
                            rsx! {
                                tr { key: "{user.id}", class: "bg-white border-b hover:bg-gray-50",
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{user.id}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{user.name}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{user.email}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{user.mobile}" }
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        span {
                                            class: if user.status == "active" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" },
                                            "{user.status}"
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                        button {
                                            class: "text-blue-600 hover:text-blue-900 mr-3",
                                            onclick: move |_| open_edit_modal(user_edit.clone()),
                                            "Edit"
                                        }
                                        button {
                                            class: "text-red-600 hover:text-red-900",
                                            onclick: move |_| open_delete_dialog(user_delete.clone()),
                                            "Delete"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                if total_pages > 1 {
                    Pagination {
                        current_page: *page.read(),
                        total_pages,
                        on_page_change: move |p| page.set(p),
                    }
                }
            }

            Modal {
                title: "Create System User".to_string(),
                is_open: *show_create_modal.read(),
                on_close: move |_| show_create_modal.set(false),
                div { class: "space-y-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Name" }
                        input {
                            r#type: "text",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border",
                            value: "{name}",
                            oninput: move |e| name.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Email" }
                        input {
                            r#type: "email",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border",
                            value: "{email}",
                            oninput: move |e| email.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Mobile" }
                        input {
                            r#type: "text",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border",
                            value: "{mobile}",
                            oninput: move |e| mobile.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Password" }
                        input {
                            r#type: "password",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border",
                            value: "{password}",
                            oninput: move |e| password.set(e.value())
                        }
                    }
                    div { class: "flex justify-end pt-4",
                        button {
                            class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50",
                            disabled: *saving.read(),
                            onclick: handle_create,
                            if *saving.read() { "Creating..." } else { "Create User" }
                        }
                    }
                }
            }

            Modal {
                title: "Edit System User".to_string(),
                is_open: *show_edit_modal.read(),
                on_close: move |_| show_edit_modal.set(false),
                div { class: "space-y-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Name" }
                        input {
                            r#type: "text",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border",
                            value: "{name}",
                            oninput: move |e| name.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Email" }
                        input {
                            r#type: "email",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border",
                            value: "{email}",
                            oninput: move |e| email.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Mobile" }
                        input {
                            r#type: "text",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border",
                            value: "{mobile}",
                            oninput: move |e| mobile.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Status" }
                        select {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border",
                            value: "{status}",
                            onchange: move |e| status.set(e.value()),
                            option { value: "active", "Active" }
                            option { value: "inactive", "Inactive" }
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Password (leave blank to keep current)" }
                        input {
                            r#type: "password",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border",
                            value: "{password}",
                            oninput: move |e| password.set(e.value())
                        }
                    }
                    div { class: "flex justify-end pt-4",
                        button {
                            class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50",
                            disabled: *saving.read(),
                            onclick: handle_update,
                            if *saving.read() { "Saving..." } else { "Save Changes" }
                        }
                    }
                }
            }

            ConfirmDialog {
                title: "Delete User".to_string(),
                message: "Are you sure you want to delete this user? This action cannot be undone.".to_string(),
                is_open: *show_delete_dialog.read(),
                on_confirm: handle_delete,
                on_cancel: move |_| show_delete_dialog.set(false),
            }
        }
    }
}
