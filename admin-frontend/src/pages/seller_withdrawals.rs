use dioxus::prelude::*;
use crate::components::{Sidebar, Header, DataTable, Column, Modal, Pagination};
use crate::api::{self, PaginatedResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WithdrawalItem {
    pub id: u64,
    pub amount: String,
    pub status: String,
    pub created_at: String,
}

#[component]
pub fn SellerWithdrawals() -> Element {
    let mut withdrawals = use_signal(|| Vec::<WithdrawalItem>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;
    
    // Modal states
    let mut show_update_modal = use_signal(|| false);
    let mut selected_withdrawal = use_signal(|| Option::<WithdrawalItem>::None);
    let mut saving = use_signal(|| false);

    // Form signals
    let mut status = use_signal(|| "pending".to_string());
    let mut notes = use_signal(|| String::new());

    let fetch_data = move || {
        let current_page = *page.read();
        spawn(async move {
            loading.set(true);
            match api::get::<PaginatedResponse<WithdrawalItem>>(&format!("/seller-withdrawals?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => {
                    withdrawals.set(data.data);
                    total.set(data.total);
                }
                Err(e) => println!("Error fetching withdrawals: {}", e),
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let mut open_update_modal = move |w: WithdrawalItem| {
        selected_withdrawal.set(Some(w.clone()));
        status.set(w.status);
        notes.set(String::new());
        show_update_modal.set(true);
    };

    let columns = vec![
        Column::new("ID", "id"),
        Column::new("Amount", "amount"),
        Column::new("Status", "status"),
        Column::new("Date", "created_at"),
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Seller Withdrawals" }
                }

                DataTable {
                    columns: columns,
                    loading: *loading.read(),
                    for w in withdrawals.read().iter() {
                        {
                            let w1 = w.clone();
                            rsx! {
                                tr { key: "{w.id}", class: "hover:bg-gray-50",
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{w.id}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 font-medium", "{w.amount}" }
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        span {
                                            class: match w.status.as_str() {
                                                "approved" => "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800",
                                                "rejected" => "px-2 py-1 text-xs rounded-full bg-red-100 text-red-800",
                                                _ => "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800",
                                            },
                                            "{w.status}"
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{w.created_at}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                        button {
                                            class: "text-blue-600 hover:text-blue-900",
                                            onclick: move |_| open_update_modal(w1.clone()),
                                            "Update Status"
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
        }

        Modal {
            title: "Update Withdrawal Status".to_string(),
            is_open: *show_update_modal.read(),
            on_close: move |_| show_update_modal.set(false),
            div { class: "space-y-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Status" }
                    select {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        value: "{status}",
                        onchange: move |e| status.set(e.value()),
                        option { value: "pending", "Pending" }
                        option { value: "approved", "Approved" }
                        option { value: "rejected", "Rejected" }
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Notes" }
                    textarea {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        rows: "3",
                        value: "{notes}",
                        oninput: move |e| notes.set(e.value())
                    }
                }

                div { class: "flex justify-end pt-4",
                    button {
                        class: "mr-3 px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50",
                        onclick: move |_| show_update_modal.set(false),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 disabled:opacity-50",
                        onclick: move |_| {
                            spawn(async move {
                                if let Some(w) = selected_withdrawal() {
                                    saving.set(true);
                                    let req = serde_json::json!({
                                        "status": status(),
                                        "notes": if notes().is_empty() { None } else { Some(notes()) }
                                    });

                                    match api::patch::<serde_json::Value, _>(&format!("/seller-withdrawals/{}", w.id), &req).await {
                                        Ok(_) => {
                                            show_update_modal.set(false);
                                            fetch_data();
                                        }
                                        Err(e) => println!("Error updating withdrawal: {}", e),
                                    }
                                    saving.set(false);
                                }
                            });
                        },
                        disabled: *saving.read(),
                        if *saving.read() { "Saving..." } else { "Save" }
                    }
                }
            }
        }
    }
}
