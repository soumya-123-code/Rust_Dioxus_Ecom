use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::Deserialize;
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, ConfirmDialog};
use crate::Route;

#[derive(Debug, Clone, Deserialize)]
pub struct Product {
    pub id: u64,
    pub uuid: String,
    pub title: String,
    pub slug: String,
    pub status: String,
    pub verification_status: Option<String>,
    pub featured: String,
    pub seller_id: u64,
    pub category_id: u64,
}

#[component]
pub fn Products() -> Element {
    let mut products = use_signal(|| Vec::<Product>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;

    let mut show_delete_confirm = use_signal(|| Option::<u64>::None);

    let fetch_data = move || {
        let current_page = *page.read();
        spawn(async move {
            loading.set(true);
            match api::get::<PaginatedResponse<Product>>(&format!("/products/datatable?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => {
                    products.set(data.data);
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
                match api::delete(&format!("/products/{}", id)).await {
                    Ok(_) => {
                        show_delete_confirm.set(None);
                        fetch_data();
                    }
                    Err(e) => println!("Error deleting product: {}", e),
                }
            });
        }
    };

    let columns = vec![
        Column { key: "id".to_string(), label: "ID".to_string(), sortable: true },
        Column { key: "title".to_string(), label: "Title".to_string(), sortable: true },
        Column { key: "status".to_string(), label: "Status".to_string(), sortable: true },
        Column { key: "verification".to_string(), label: "Verification".to_string(), sortable: true },
        Column { key: "featured".to_string(), label: "Featured".to_string(), sortable: false },
        Column { key: "actions".to_string(), label: "Actions".to_string(), sortable: false },
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;
    let products_list = products.read().clone();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Products" }
                }
                
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for product in products_list {
                        tr { key: "{product.id}",
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{product.id}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 max-w-xs truncate", "{product.title}" }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span {
                                    class: if product.status == "active" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800" },
                                    "{product.status}"
                                }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span {
                                    class: match product.verification_status.as_deref() {
                                        Some("approved") => "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800",
                                        Some("pending") => "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800",
                                        Some("rejected") => "px-2 py-1 text-xs rounded-full bg-red-100 text-red-800",
                                        _ => "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800",
                                    },
                                    "{product.verification_status.clone().unwrap_or_default()}"
                                }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm",
                                if product.featured == "1" {
                                    span { class: "text-yellow-500", "★" }
                                } else {
                                    span { class: "text-gray-300", "☆" }
                                }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                Link {
                                    to: Route::ProductDetail { id: product.id },
                                    class: "text-blue-600 hover:text-blue-900 mr-3",
                                    "View"
                                }
                                button {
                                    class: "text-red-600 hover:text-red-900",
                                    onclick: move |_| show_delete_confirm.set(Some(product.id)),
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

            ConfirmDialog {
                title: "Delete Product".to_string(),
                message: "Are you sure you want to delete this product? This action cannot be undone.".to_string(),
                is_open: show_delete_confirm.read().is_some(),
                on_confirm: handle_delete_confirm,
                on_cancel: move |_| show_delete_confirm.set(None),
            }
        }
    }
}

#[component]
pub fn ProductDetail(id: u64) -> Element {
    let mut product = use_signal(|| Option::<Product>::None);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            #[derive(Deserialize)]
            struct Response {
                data: Option<Product>,
            }
            
            match api::get::<Response>(&format!("/products/{}", id)).await {
                Ok(resp) => product.set(resp.data),
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
                        to: Route::Products {},
                        class: "text-blue-600 hover:text-blue-800",
                        "← Back to Products"
                    }
                }
                
                if *loading.read() {
                    div { class: "text-center py-8",
                        p { class: "text-gray-500", "Loading..." }
                    }
                } else if let Some(p) = product.read().as_ref() {
                    div { class: "bg-white rounded-lg shadow p-6",
                        h1 { class: "text-2xl font-bold text-gray-800 mb-4", "{p.title}" }
                        
                        div { class: "grid grid-cols-2 gap-4",
                            div {
                                p { class: "text-sm text-gray-500", "ID" }
                                p { class: "font-medium", "{p.id}" }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "UUID" }
                                p { class: "font-medium text-sm", "{p.uuid}" }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "Status" }
                                span {
                                    class: if p.status == "active" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800" },
                                    "{p.status}"
                                }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "Verification Status" }
                                span {
                                    class: match p.verification_status.as_deref() {
                                        Some("approved") => "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800",
                                        Some("pending") => "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800",
                                        _ => "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800",
                                    },
                                    "{p.verification_status.clone().unwrap_or_default()}"
                                }
                            }
                        }
                    }
                } else {
                    div { class: "text-center py-8",
                        p { class: "text-gray-500", "Product not found" }
                    }
                }
            }
        }
    }
}
