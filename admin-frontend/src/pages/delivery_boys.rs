use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, Modal, ConfirmDialog};

#[derive(Debug, Clone, Deserialize)]
pub struct DeliveryBoy {
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
pub fn DeliveryBoys() -> Element {
    let mut boys = use_signal(|| Vec::<DeliveryBoy>::new());
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
            match api::get::<PaginatedResponse<DeliveryBoy>>(&format!("/delivery-boys/datatable?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => { boys.set(data.data); total.set(data.total); }
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
                    rejection_reason: None,
                };
                match api::post::<serde_json::Value, _>(&format!("/delivery-boys/{}/verification-status", id), &req).await {
                    Ok(_) => {
                        show_approve_confirm.set(None);
                        fetch_data();
                    }
                    Err(e) => println!("Error approving delivery boy: {}", e),
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
                match api::post::<serde_json::Value, _>(&format!("/delivery-boys/{}/verification-status", id), &req).await {
                    Ok(_) => {
                        show_reject_modal.set(None);
                        fetch_data();
                    }
                    Err(e) => println!("Error rejecting delivery boy: {}", e),
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
    let boys_list = boys.read().clone();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Delivery Boys" }
                }
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for boy in boys_list {
                        tr { key: "{boy.id}",
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{boy.id}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{boy.name}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{boy.email}" }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span { class: if boy.status == "1" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" }, if boy.status == "1" { "Active" } else { "Inactive" } }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span { class: match boy.verification_status.as_deref() { Some("approved") => "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800", Some("pending") => "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800", Some("rejected") => "px-2 py-1 text-xs rounded-full bg-red-100 text-red-800", _ => "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" }, "{boy.verification_status.clone().unwrap_or_default()}" }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                if boy.verification_status.as_deref() != Some("approved") {
                                    button { class: "text-green-600 hover:text-green-900 mr-3", onclick: move |_| show_approve_confirm.set(Some(boy.id)), "Approve" }
                                }
                                if boy.verification_status.as_deref() != Some("rejected") {
                                    button { class: "text-red-600 hover:text-red-900 mr-3", onclick: move |_| {
                                        rejection_reason.set(String::new());
                                        show_reject_modal.set(Some(boy.id));
                                    }, "Reject" }
                                }
                                button { class: "text-blue-600 hover:text-blue-900", "View" }
                            }
                        }
                    }
                }
                if total_pages > 1 { Pagination { current_page: *page.read(), total_pages: total_pages, on_page_change: move |new_page| { page.set(new_page); fetch_data(); } } }
                
                ConfirmDialog {
                    title: "Approve Delivery Boy".to_string(),
                    message: "Are you sure you want to approve this delivery boy?".to_string(),
                    is_open: show_approve_confirm.read().is_some(),
                    on_confirm: handle_approve_confirm,
                    on_cancel: move |_| show_approve_confirm.set(None),
                }

                Modal {
                    title: "Reject Delivery Boy".to_string(),
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
                                "Reject"
                            }
                        }
                    }
                }
            }
        }
    }
}
