use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, Modal, ConfirmDialog};
use crate::Route;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize)]
struct UpdateVerificationStatusRequest {
    verification_status: String,
    rejection_reason: Option<String>,
}

#[component]
pub fn PendingProducts() -> Element {
    let mut products = use_signal(|| Vec::<Product>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;

    let mut show_approve_confirm = use_signal(|| Option::<u64>::None);
    let mut show_reject_modal = use_signal(|| Option::<u64>::None);
    let mut rejection_reason = use_signal(|| String::new());

    let fetch_data = move || {
        let current_page = *page.read();
        spawn(async move {
            loading.set(true);
            match api::get::<PaginatedResponse<Product>>(&format!("/products/datatable?page={}&per_page={}&verification_status=pending", current_page, per_page)).await {
                Ok(data) => {
                    products.set(data.data);
                    total.set(data.total);
                }
                Err(e) => println!("Error fetching pending products: {}", e),
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let handle_approve_confirm = move |_| {
        if let Some(id) = *show_approve_confirm.read() {
            spawn(async move {
                let req = UpdateVerificationStatusRequest {
                    verification_status: "approved".to_string(),
                    rejection_reason: None,
                };
                match api::post::<serde_json::Value, _>(&format!("/products/{}/verification-status", id), &req).await {
                    Ok(_) => {
                        show_approve_confirm.set(None);
                        fetch_data();
                    }
                    Err(e) => println!("Error approving product: {}", e),
                }
            });
        }
    };

    let handle_reject_submit = move |_| {
        if let Some(id) = *show_reject_modal.read() {
            spawn(async move {
                let req = UpdateVerificationStatusRequest {
                    verification_status: "rejected".to_string(),
                    rejection_reason: Some(rejection_reason()),
                };
                match api::post::<serde_json::Value, _>(&format!("/products/{}/verification-status", id), &req).await {
                    Ok(_) => {
                        show_reject_modal.set(None);
                        fetch_data();
                    }
                    Err(e) => println!("Error rejecting product: {}", e),
                }
            });
        }
    };

    let columns = vec![
        Column::new("ID", "id"),
        Column::new("Title", "title"),
        Column::new("Status", "status"),
        Column::new("Verification", "verification"),
        Column::new("Featured", "featured"),
        Column::new("Actions", "actions"),
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;
    let products_list = products.read().clone();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Pending Products" }
                }
                
                DataTable {
                    columns: columns,
                    loading: *loading.read(),
                    for product in products_list {
                        {
                            rsx! {
                                tr { key: "{product.id}", class: "hover:bg-gray-50",
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
                                            class: "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800",
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
                                        button {
                                            class: "text-green-600 hover:text-green-900 mr-3",
                                            onclick: move |_| show_approve_confirm.set(Some(product.id)),
                                            "Approve"
                                        }
                                        button {
                                            class: "text-red-600 hover:text-red-900 mr-3",
                                            onclick: move |_| {
                                                rejection_reason.set(String::new());
                                                show_reject_modal.set(Some(product.id));
                                            },
                                            "Reject"
                                        }
                                        Link {
                                            to: Route::ProductDetail { id: product.id },
                                            class: "text-blue-600 hover:text-blue-900",
                                            "View"
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
                        total_pages: total_pages,
                        on_page_change: move |new_page| {
                            page.set(new_page);
                            fetch_data();
                        },
                    }
                }
            }

            ConfirmDialog {
                title: "Approve Product".to_string(),
                message: "Are you sure you want to approve this product? It will be visible to customers.".to_string(),
                is_open: show_approve_confirm.read().is_some(),
                on_confirm: handle_approve_confirm,
                on_cancel: move |_| show_approve_confirm.set(None),
            }

            Modal {
                title: "Reject Product".to_string(),
                is_open: show_reject_modal.read().is_some(),
                on_close: move |_| show_reject_modal.set(None),
                div { class: "mt-2",
                    label { class: "block text-sm font-medium text-gray-700", "Rejection Reason" }
                    textarea {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
                        rows: "3",
                        value: "{rejection_reason}",
                        oninput: move |e| rejection_reason.set(e.value()),
                        placeholder: "Enter reason for rejection..."
                    }
                    div { class: "mt-4 flex justify-end",
                        button {
                            class: "mr-3 inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none sm:text-sm",
                            onclick: move |_| show_reject_modal.set(None),
                            "Cancel"
                        }
                        button {
                            class: "inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-red-600 text-base font-medium text-white hover:bg-red-700 focus:outline-none sm:text-sm",
                            onclick: handle_reject_submit,
                            "Reject Product"
                        }
                    }
                }
            }
        }
    }
}
