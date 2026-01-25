use dioxus::prelude::*;
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, ConfirmDialog};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Store {
    pub id: u64,
    pub uuid: String,
    pub title: String,
    pub slug: String,
    pub verification_status: Option<String>,
    pub status: String,
    pub seller_id: u64,
}

#[derive(Debug, Serialize)]
pub struct UpdateVerificationStatusRequest {
    pub verification_status: String,
}

#[component]
pub fn Stores() -> Element {
    let mut stores = use_signal(|| Vec::<Store>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;

    let mut show_approve_confirm = use_signal(|| Option::<u64>::None);
    let mut show_reject_confirm = use_signal(|| Option::<u64>::None);

    let fetch_data = move || {
        let current_page = *page.read();
        spawn(async move {
            loading.set(true);
            match api::get::<PaginatedResponse<Store>>(&format!("/stores/datatable?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => {
                    stores.set(data.data);
                    total.set(data.total);
                }
                Err(_) => {}
            }
            loading.set(false);
        });
    };

    use_effect(move || { fetch_data(); });

    let handle_approve_confirm = move |_| {
        if let Some(id) = *show_approve_confirm.read() {
            spawn(async move {
                let req = UpdateVerificationStatusRequest {
                    verification_status: "approved".to_string(),
                };
                match api::post::<serde_json::Value, _>(&format!("/stores/{}/verification-status", id), &req).await {
                    Ok(_) => {
                        show_approve_confirm.set(None);
                        fetch_data();
                    }
                    Err(e) => println!("Error approving store: {}", e),
                }
            });
        }
    };

    let handle_reject_confirm = move |_| {
        if let Some(id) = *show_reject_confirm.read() {
            spawn(async move {
                let req = UpdateVerificationStatusRequest {
                    verification_status: "rejected".to_string(),
                };
                match api::post::<serde_json::Value, _>(&format!("/stores/{}/verification-status", id), &req).await {
                    Ok(_) => {
                        show_reject_confirm.set(None);
                        fetch_data();
                    }
                    Err(e) => println!("Error rejecting store: {}", e),
                }
            });
        }
    };

    let columns = vec![
        Column { key: "id".to_string(), label: "ID".to_string(), sortable: true },
        Column { key: "title".to_string(), label: "Title".to_string(), sortable: true },
        Column { key: "status".to_string(), label: "Status".to_string(), sortable: true },
        Column { key: "verification".to_string(), label: "Verification".to_string(), sortable: true },
        Column { key: "actions".to_string(), label: "Actions".to_string(), sortable: false },
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;
    let stores_list = stores.read().clone();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Stores" }
                }
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for store in stores_list {
                        tr { key: "{store.id}",
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{store.id}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{store.title}" }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span { class: if store.status == "active" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" }, "{store.status}" }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span { class: match store.verification_status.as_deref() { Some("approved") => "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800", Some("pending") => "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800", _ => "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" }, "{store.verification_status.clone().unwrap_or_default()}" }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                if store.verification_status.as_deref() != Some("approved") {
                                    button { class: "text-green-600 hover:text-green-900 mr-3", onclick: move |_| show_approve_confirm.set(Some(store.id)), "Approve" }
                                }
                                if store.verification_status.as_deref() != Some("rejected") {
                                    button { class: "text-red-600 hover:text-red-900 mr-3", onclick: move |_| show_reject_confirm.set(Some(store.id)), "Reject" }
                                }
                                button { class: "text-blue-600 hover:text-blue-900", "View" }
                            }
                        }
                    }
                }
                if total_pages > 1 {
                    Pagination { current_page: *page.read(), total_pages: total_pages, on_page_change: move |new_page| { page.set(new_page); fetch_data(); } }
                }
            }
            
            ConfirmDialog {
                title: "Approve Store".to_string(),
                message: "Are you sure you want to approve this store? It will be visible to customers.".to_string(),
                is_open: show_approve_confirm.read().is_some(),
                on_confirm: handle_approve_confirm,
                on_cancel: move |_| show_approve_confirm.set(None),
            }

            ConfirmDialog {
                title: "Reject Store".to_string(),
                message: "Are you sure you want to reject this store?".to_string(),
                is_open: show_reject_confirm.read().is_some(),
                on_confirm: handle_reject_confirm,
                on_cancel: move |_| show_reject_confirm.set(None),
            }
        }
    }
}
