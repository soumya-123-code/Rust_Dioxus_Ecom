use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, Modal, ConfirmDialog};
use crate::Route;

#[derive(Debug, Clone, Deserialize)]
pub struct Seller {
    pub id: u64,
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub verification_status: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize)]
struct UpdateVerificationStatusRequest {
    verification_status: String,
    rejection_reason: Option<String>,
}

#[component]
pub fn Sellers() -> Element {
    let mut sellers = use_signal(|| Vec::<Seller>::new());
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
            match api::get::<PaginatedResponse<Seller>>(&format!("/sellers/datatable?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => {
                    sellers.set(data.data);
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

    let handle_approve_confirm = move |_| {
        if let Some(id) = *show_approve_confirm.read() {
            spawn(async move {
                let req = UpdateVerificationStatusRequest {
                    verification_status: "approved".to_string(),
                    rejection_reason: None,
                };
                match api::post::<serde_json::Value, _>(&format!("/sellers/{}/verification-status", id), &req).await {
                    Ok(_) => {
                        show_approve_confirm.set(None);
                        fetch_data();
                    }
                    Err(e) => println!("Error approving seller: {}", e),
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
                match api::post::<serde_json::Value, _>(&format!("/sellers/{}/verification-status", id), &req).await {
                    Ok(_) => {
                        show_reject_modal.set(None);
                        fetch_data();
                    }
                    Err(e) => println!("Error rejecting seller: {}", e),
                }
            });
        }
    };

    let columns = vec![
        Column { key: "id".to_string(), label: "ID".to_string(), sortable: true },
        Column { key: "name".to_string(), label: "Name".to_string(), sortable: true },
        Column { key: "email".to_string(), label: "Email".to_string(), sortable: true },
        Column { key: "status".to_string(), label: "Status".to_string(), sortable: true },
        Column { key: "verification".to_string(), label: "Verification".to_string(), sortable: true },
        Column { key: "actions".to_string(), label: "Actions".to_string(), sortable: false },
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;
    let sellers_list = sellers.read().clone();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Sellers" }
                }
                
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for seller in sellers_list {
                        tr { key: "{seller.id}",
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{seller.id}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{seller.name}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{seller.email}" }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span {
                                    class: if seller.status == "1" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" },
                                    if seller.status == "1" { "Active" } else { "Inactive" }
                                }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span {
                                    class: match seller.verification_status.as_deref() {
                                        Some("approved") => "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800",
                                        Some("pending") => "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800",
                                        Some("rejected") => "px-2 py-1 text-xs rounded-full bg-red-100 text-red-800",
                                        _ => "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800",
                                    },
                                    "{seller.verification_status.clone().unwrap_or_default()}"
                                }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                if seller.verification_status.as_deref() != Some("approved") {
                                    button {
                                        class: "text-green-600 hover:text-green-900 mr-3",
                                        onclick: move |_| show_approve_confirm.set(Some(seller.id)),
                                        "Approve"
                                    }
                                }
                                if seller.verification_status.as_deref() != Some("rejected") {
                                    button {
                                        class: "text-red-600 hover:text-red-900 mr-3",
                                        onclick: move |_| {
                                            rejection_reason.set(String::new());
                                            show_reject_modal.set(Some(seller.id));
                                        },
                                        "Reject"
                                    }
                                }
                                Link {
                                    to: Route::SellerDetail { id: seller.id },
                                    class: "text-blue-600 hover:text-blue-900",
                                    "View"
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
                title: "Approve Seller".to_string(),
                message: "Are you sure you want to approve this seller? They will be able to sell products.".to_string(),
                is_open: show_approve_confirm.read().is_some(),
                on_confirm: handle_approve_confirm,
                on_cancel: move |_| show_approve_confirm.set(None),
            }

            Modal {
                title: "Reject Seller".to_string(),
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
                            "Reject Seller"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SellerDetail(id: u64) -> Element {
    let mut seller = use_signal(|| Option::<Seller>::None);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            #[derive(Deserialize)]
            struct Response {
                data: Option<Seller>,
            }
            match api::get::<Response>(&format!("/sellers/{}", id)).await {
                Ok(resp) => seller.set(resp.data),
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
                        to: Route::Sellers {},
                        class: "text-blue-600 hover:text-blue-800",
                        "← Back to Sellers"
                    }
                }
                
                if *loading.read() {
                    div { class: "text-center py-8", p { class: "text-gray-500", "Loading..." } }
                } else if let Some(s) = seller.read().as_ref() {
                    div { class: "bg-white rounded-lg shadow p-6",
                        h1 { class: "text-2xl font-bold text-gray-800 mb-4", "{s.name}" }
                        div { class: "grid grid-cols-2 gap-4",
                            div { p { class: "text-sm text-gray-500", "Email" } p { class: "font-medium", "{s.email}" } }
                            div { p { class: "text-sm text-gray-500", "Mobile" } p { class: "font-medium", "{s.mobile}" } }
                            div { p { class: "text-sm text-gray-500", "Status" } p { class: "font-medium", if s.status == "1" { "Active" } else { "Inactive" } } }
                            div { p { class: "text-sm text-gray-500", "Verification" } p { class: "font-medium", "{s.verification_status.clone().unwrap_or_default()}" } }
                        }
                    }
                }
            }
        }
    }
}
