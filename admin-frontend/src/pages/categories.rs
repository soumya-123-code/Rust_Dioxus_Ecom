use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, Modal, ConfirmDialog};
use crate::Route;

#[derive(Debug, Clone, Deserialize)]
pub struct Category {
    pub id: u64,
    pub uuid: String,
    pub title: String,
    pub slug: String,
    pub status: String,
    pub requires_approval: bool,
    pub commission: Option<String>,
}

#[component]
pub fn Categories() -> Element {
    let mut categories = use_signal(|| Vec::<Category>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;
    
    let mut show_create_modal = use_signal(|| false);
    let mut show_delete_confirm = use_signal(|| Option::<u64>::None);

    let fetch_data = move || {
        let current_page = *page.read();
        spawn(async move {
            loading.set(true);
            match api::get::<PaginatedResponse<Category>>(&format!("/categories/datatable?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => {
                    categories.set(data.data);
                    total.set(data.total);
                }
                Err(_) => {}
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let handle_delete_confirm = move |_| {
        if let Some(id) = *show_delete_confirm.read() {
            spawn(async move {
                match api::delete(&format!("/categories/{}", id)).await {
                    Ok(_) => {
                        show_delete_confirm.set(None);
                        fetch_data();
                    }
                    Err(e) => println!("Error deleting category: {}", e),
                }
            });
        }
    };

    let columns = vec![
        Column { key: "id".to_string(), label: "ID".to_string(), sortable: true },
        Column { key: "title".to_string(), label: "Title".to_string(), sortable: true },
        Column { key: "status".to_string(), label: "Status".to_string(), sortable: true },
        Column { key: "commission".to_string(), label: "Commission".to_string(), sortable: false },
        Column { key: "actions".to_string(), label: "Actions".to_string(), sortable: false },
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;
    let categories_list = categories.read().clone();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Categories" }
                    button {
                        class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700",
                        onclick: move |_| show_create_modal.set(true),
                        "+ Add Category"
                    }
                }
                
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for cat in categories_list {
                        tr { key: "{cat.id}",
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{cat.id}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{cat.title}" }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span {
                                    class: if cat.status == "active" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" },
                                    "{cat.status}"
                                }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                "{cat.commission.clone().unwrap_or_default()}%"
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                Link {
                                    to: Route::CategoryDetail { id: cat.id },
                                    class: "text-blue-600 hover:text-blue-900 mr-3",
                                    "View"
                                }
                                button {
                                    class: "text-red-600 hover:text-red-900",
                                    onclick: move |_| show_delete_confirm.set(Some(cat.id)),
                                    "Delete"
                                }
                            }
                        }
                    }
                }
                
                if total_pages > 1 {
                    Pagination {
                        current_page: *page.read(),
                        total_pages: total_pages,
                        on_page_change: move |new_page| {
                            page.set(new_page);
                            fetch_data();
                        },
                    }
                }
            }
            
            Modal {
                title: "Create Category".to_string(),
                is_open: *show_create_modal.read(),
                on_close: move |_| show_create_modal.set(false),
                CategoryForm {
                    on_submit: move |_| {
                        show_create_modal.set(false);
                        fetch_data();
                    },
                }
            }

            ConfirmDialog {
                title: "Delete Category".to_string(),
                message: "Are you sure you want to delete this category? This action cannot be undone.".to_string(),
                is_open: show_delete_confirm.read().is_some(),
                on_confirm: handle_delete_confirm,
                on_cancel: move |_| show_delete_confirm.set(None),
            }
        }
    }
}

#[component]
fn CategoryForm(on_submit: EventHandler<()>) -> Element {
    let mut title = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut status = use_signal(|| "active".to_string());
    let mut loading = use_signal(|| false);

    let handle_submit = move |_evt: Event<FormData>| {
        let title_val = title.read().clone();
        let description_val = description.read().clone();
        let status_val = status.read().clone();
        
        spawn(async move {
            loading.set(true);
            
            #[derive(Serialize)]
            struct CreateCategory {
                title: String,
                description: String,
                status: String,
                image: String,
            }
            
            let body = CreateCategory {
                title: title_val,
                description: description_val,
                status: status_val,
                image: "".to_string(),
            };
            
            match api::post::<serde_json::Value, _>("/categories", &body).await {
                Ok(_) => on_submit.call(()),
                Err(_) => {}
            }
            
            loading.set(false);
        });
    };

    rsx! {
        form { onsubmit: handle_submit,
            div { class: "mb-4",
                label { class: "block text-sm font-medium text-gray-700 mb-1", "Title" }
                input {
                    r#type: "text",
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    value: "{title}",
                    oninput: move |evt| title.set(evt.value()),
                    required: true,
                }
            }
            div { class: "mb-4",
                label { class: "block text-sm font-medium text-gray-700 mb-1", "Description" }
                textarea {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    rows: "3",
                    value: "{description}",
                    oninput: move |evt| description.set(evt.value()),
                }
            }
            div { class: "mb-4",
                label { class: "block text-sm font-medium text-gray-700 mb-1", "Status" }
                select {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    value: "{status}",
                    onchange: move |evt| status.set(evt.value()),
                    option { value: "active", "Active" }
                    option { value: "inactive", "Inactive" }
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

#[component]
pub fn CategoryDetail(id: u64) -> Element {
    let mut category = use_signal(|| Option::<Category>::None);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            #[derive(Deserialize)]
            struct Response {
                data: Option<Category>,
            }
            
            match api::get::<Response>(&format!("/categories/{}", id)).await {
                Ok(resp) => category.set(resp.data),
                Err(_) => {}
            }
            loading.set(false);
        });
    });

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "mb-6",
                    Link {
                        to: Route::Categories {},
                        class: "text-blue-600 hover:text-blue-800",
                        "← Back to Categories"
                    }
                }
                
                if *loading.read() {
                    div { class: "text-center py-8",
                        p { class: "text-gray-500", "Loading..." }
                    }
                } else if let Some(cat) = category.read().as_ref() {
                    div { class: "bg-white rounded-lg shadow p-6",
                        h1 { class: "text-2xl font-bold text-gray-800 mb-4", "{cat.title}" }
                        
                        div { class: "grid grid-cols-2 gap-4",
                            div {
                                p { class: "text-sm text-gray-500", "ID" }
                                p { class: "font-medium", "{cat.id}" }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "UUID" }
                                p { class: "font-medium text-sm", "{cat.uuid}" }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "Slug" }
                                p { class: "font-medium", "{cat.slug}" }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "Status" }
                                span {
                                    class: if cat.status == "active" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" },
                                    "{cat.status}"
                                }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "Commission" }
                                p { class: "font-medium", "{cat.commission.clone().unwrap_or_default()}%" }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "Requires Approval" }
                                p { class: "font-medium", if cat.requires_approval { "Yes" } else { "No" } }
                            }
                        }
                    }
                } else {
                    div { class: "text-center py-8",
                        p { class: "text-gray-500", "Category not found" }
                    }
                }
            }
        }
    }
}
