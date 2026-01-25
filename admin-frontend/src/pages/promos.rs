use dioxus::prelude::*;
use serde::Deserialize;
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination};

#[derive(Debug, Clone, Deserialize)]
pub struct Promo {
    pub id: u64,
    pub uuid: String,
    pub title: String,
    pub code: String,
    pub discount_type: String,
    pub discount_value: String,
    pub status: String,
}

#[component]
pub fn Promos() -> Element {
    let mut promos = use_signal(|| Vec::<Promo>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;

    let fetch_data = move || {
        let current_page = *page.read();
        spawn(async move {
            loading.set(true);
            match api::get::<PaginatedResponse<Promo>>(&format!("/promos/datatable?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => { promos.set(data.data); total.set(data.total); }
                Err(_) => {}
            }
            loading.set(false);
        });
    };

    use_effect(move || { fetch_data(); });

    let columns = vec![
        Column { key: "id".to_string(), label: "ID".to_string(), sortable: true },
        Column { key: "title".to_string(), label: "Title".to_string(), sortable: true },
        Column { key: "code".to_string(), label: "Code".to_string(), sortable: true },
        Column { key: "discount".to_string(), label: "Discount".to_string(), sortable: false },
        Column { key: "status".to_string(), label: "Status".to_string(), sortable: true },
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Promos" }
                    button { class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700", "+ Add Promo" }
                }
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for promo in promos.read().iter() {
                        tr { key: "{promo.id}",
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{promo.id}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{promo.title}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono bg-gray-100 px-2 py-1 rounded", "{promo.code}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", {if promo.discount_type == "percentage" { format!("{}%", promo.discount_value) } else { format!("${}", promo.discount_value) }} }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span { class: if promo.status == "1" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" }, if promo.status == "1" { "Active" } else { "Inactive" } }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                button { class: "text-blue-600 hover:text-blue-900 mr-3", "Edit" }
                                button { class: "text-red-600 hover:text-red-900", "Delete" }
                            }
                        }
                    }
                }
                if total_pages > 1 { Pagination { current_page: *page.read(), total_pages: total_pages, on_page_change: move |new_page| { page.set(new_page); fetch_data(); } } }
            }
        }
    }
}
