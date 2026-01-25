use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, ConfirmDialog};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RefundRequest {
    pub id: u64,
    pub order_id: u64,
    pub order_number: String,
    pub customer_name: String,
    pub product_name: String,
    pub amount: f64,
    pub status: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

#[component]
pub fn Refunds() -> Element {
    let mut refunds = use_signal(|| Vec::<RefundRequest>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;
    
    let mut show_confirm_dialog = use_signal(|| false);
    let mut selected_refund = use_signal(|| None::<RefundRequest>);
    let mut pending_status = use_signal(|| "".to_string());

    let fetch_data = move || {
        let current_page = *page.read();
        spawn(async move {
            loading.set(true);
            match api::get::<PaginatedResponse<RefundRequest>>(&format!("/refunds?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => {
                    refunds.set(data.data);
                    total.set(data.total);
                }
                Err(e) => println!("Failed to fetch refunds: {}", e),
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let handle_status_update = move |_| {
        if let Some(refund) = selected_refund.read().clone() {
            let status = pending_status.read().clone();
            spawn(async move {
                match api::post::<serde_json::Value, _>(&format!("/refunds/{}/status", refund.id), &UpdateStatusRequest { status: status.clone() }).await {
                    Ok(_) => {
                        show_confirm_dialog.set(false);
                        fetch_data();
                    }
                    Err(e) => println!("Failed to update refund status: {}", e),
                }
            });
        }
    };

    let columns = vec![
        Column::new("Order ID", "order_number"),
        Column::new("Customer", "customer_name"),
        Column::new("Product", "product_name"),
        Column::new("Amount", "amount"),
        Column::new("Date", "date"),
        Column::new("Status", "status"),
        Column { key: "actions".to_string(), label: "Actions".to_string(), sortable: false },
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;
    let current_refunds = refunds.read().clone();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Refund Requests" }
                }
                
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for refund in current_refunds {
                        tr { key: "{refund.id}", class: "bg-white border-b hover:bg-gray-50",
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "#{refund.order_number}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{refund.customer_name}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{refund.product_name}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 font-medium", "${refund.amount}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{refund.date}" }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span {
                                    class: match refund.status.as_str() {
                                        "approved" => "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800",
                                        "rejected" => "px-2 py-1 text-xs rounded-full bg-red-100 text-red-800",
                                        _ => "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800",
                                    },
                                    "{refund.status}"
                                }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                if refund.status == "pending" {
                                    {
                                        let refund_approve = refund.clone();
                                        let refund_reject = refund.clone();
                                        rsx! {
                                            button {
                                                class: "text-green-600 hover:text-green-900 mr-3",
                                                onclick: move |_| {
                                                    selected_refund.set(Some(refund_approve.clone()));
                                                    pending_status.set("approved".to_string());
                                                    show_confirm_dialog.set(true);
                                                },
                                                "Approve"
                                            }
                                            button {
                                                class: "text-red-600 hover:text-red-900",
                                                onclick: move |_| {
                                                    selected_refund.set(Some(refund_reject.clone()));
                                                    pending_status.set("rejected".to_string());
                                                    show_confirm_dialog.set(true);
                                                },
                                                "Reject"
                                            }
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

            ConfirmDialog {
                title: format!("{} Refund Request", if pending_status.read().as_str() == "approved" { "Approve" } else { "Reject" }),
                message: format!("Are you sure you want to {} this refund request?", if pending_status.read().as_str() == "approved" { "approve" } else { "reject" }),
                is_open: *show_confirm_dialog.read(),
                on_confirm: handle_status_update,
                on_cancel: move |_| show_confirm_dialog.set(false),
            }
        }
    }
}
