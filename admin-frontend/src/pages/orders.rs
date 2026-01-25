use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::Deserialize;
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination};
use crate::Route;

#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    pub id: u64,
    pub uuid: String,
    pub order_number: String,
    pub status: String,
    pub payment_status: String,
    pub total: String,
    pub user_id: u64,
}

#[component]
pub fn Orders() -> Element {
    let mut orders = use_signal(|| Vec::<Order>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;

    let fetch_data = move || {
        let current_page = *page.read();
        spawn(async move {
            loading.set(true);
            match api::get::<PaginatedResponse<Order>>(&format!("/orders/datatable?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => {
                    orders.set(data.data);
                    total.set(data.total);
                }
                Err(_) => {}
            }
            loading.set(false);
        });
    };

    use_effect(move || { fetch_data(); });

    let columns = vec![
        Column { key: "id".to_string(), label: "ID".to_string(), sortable: true },
        Column { key: "order_number".to_string(), label: "Order #".to_string(), sortable: true },
        Column { key: "status".to_string(), label: "Status".to_string(), sortable: true },
        Column { key: "payment".to_string(), label: "Payment".to_string(), sortable: true },
        Column { key: "total".to_string(), label: "Total".to_string(), sortable: true },
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Orders" }
                }
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for order in orders.read().iter() {
                        tr { key: "{order.id}",
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{order.id}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900", "{order.order_number}" }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span { class: match order.status.as_str() { "delivered" => "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800", "pending" => "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800", "cancelled" => "px-2 py-1 text-xs rounded-full bg-red-100 text-red-800", _ => "px-2 py-1 text-xs rounded-full bg-blue-100 text-blue-800" }, "{order.status}" }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span { class: if order.payment_status == "paid" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800" }, "{order.payment_status}" }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "${order.total}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                Link { to: Route::OrderDetail { id: order.id }, class: "text-blue-600 hover:text-blue-900", "View" }
                            }
                        }
                    }
                }
                if total_pages > 1 {
                    Pagination { current_page: *page.read(), total_pages: total_pages, on_page_change: move |new_page| { page.set(new_page); fetch_data(); } }
                }
            }
        }
    }
}

#[component]
pub fn OrderDetail(id: u64) -> Element {
    let mut order = use_signal(|| Option::<Order>::None);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            #[derive(Deserialize)]
            struct Response { data: Option<Order> }
            match api::get::<Response>(&format!("/orders/{}", id)).await {
                Ok(resp) => order.set(resp.data),
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
                div { class: "mb-6", Link { to: Route::Orders {}, class: "text-blue-600 hover:text-blue-800", "← Back to Orders" } }
                if *loading.read() {
                    div { class: "text-center py-8", p { class: "text-gray-500", "Loading..." } }
                } else if let Some(o) = order.read().as_ref() {
                    div { class: "bg-white rounded-lg shadow p-6",
                        h1 { class: "text-2xl font-bold text-gray-800 mb-4", "Order #{o.order_number}" }
                        div { class: "grid grid-cols-2 gap-4",
                            div { p { class: "text-sm text-gray-500", "Status" } p { class: "font-medium", "{o.status}" } }
                            div { p { class: "text-sm text-gray-500", "Payment Status" } p { class: "font-medium", "{o.payment_status}" } }
                            div { p { class: "text-sm text-gray-500", "Total" } p { class: "font-medium", "${o.total}" } }
                        }
                    }
                }
            }
        }
    }
}
