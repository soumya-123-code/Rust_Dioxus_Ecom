use dioxus::prelude::*;
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, ConfirmDialog};
use crate::api::{self, PaginatedResponse};

#[derive(PartialEq, Clone, Debug, serde::Deserialize)]
pub struct Review {
    pub id: u64,
    pub product_name: String,
    pub user_name: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
}

#[component]
pub fn Reviews() -> Element {
    let mut page = use_signal(|| 1);
    let per_page = use_signal(|| 10);
    let mut reviews = use_signal(|| PaginatedResponse::<Review>::default());
    let mut error = use_signal(|| Option::<String>::None);
    let mut refresh_trigger = use_signal(|| 0);
    
    let mut show_delete_dialog = use_signal(|| false);
    let mut selected_review_id = use_signal(|| None::<u64>);

    use_effect(move || {
        spawn(async move {
            match api::get::<PaginatedResponse<Review>>(&format!("/reviews?page={}&per_page={}", page(), per_page())).await {
                Ok(res) => reviews.set(res),
                Err(e) => error.set(Some(e.to_string())),
            }
        });
    });

    use_effect(move || {
        if refresh_trigger() > 0 {
             spawn(async move {
                match api::get::<PaginatedResponse<Review>>(&format!("/reviews?page={}&per_page={}", page(), per_page())).await {
                    Ok(res) => reviews.set(res),
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        }
    });

    let toggle_status = move |id: u64, current_status: String| {
        spawn(async move {
            let new_status = if current_status == "approved" { "pending" } else { "approved" };
            
            match api::post::<serde_json::Value, _>(&format!("/reviews/{}/status", id), &serde_json::json!({ "status": new_status })).await {
                Ok(_) => {
                    refresh_trigger.set(refresh_trigger() + 1);
                },
                Err(e) => error.set(Some(e.to_string())),
            }
        });
    };

    let mut delete_review = move |id: u64| {
        selected_review_id.set(Some(id));
        show_delete_dialog.set(true);
    };

    let confirm_delete = move |_| {
        if let Some(id) = *selected_review_id.read() {
            spawn(async move {
                match api::delete(&format!("/reviews/{}", id)).await {
                    Ok(_) => {
                        show_delete_dialog.set(false);
                        refresh_trigger.set(refresh_trigger() + 1);
                    },
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        }
    };

    let columns = vec![
        Column::new("ID", "id"),
        Column::new("Product", "product_name"),
        Column::new("Customer", "user_name"),
        Column::new("Rating", "rating"),
        Column::new("Comment", "comment"),
        Column::new("Status", "status"),
        Column::new("Date", "created_at"),
        Column::new("Actions", "actions"),
    ];

    let current_reviews = reviews.read().data.clone();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Product Reviews" }
                }

                if let Some(err) = error() {
                    div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4", "{err}" }
                }
                
                DataTable {
                    columns: columns.clone(),
                    loading: false, // TODO: Add loading state
                    for review in current_reviews {
                        {
                            let id = review.id;
                            let status = review.status.clone();
                            rsx! {
                                tr { key: "{review.id}", class: "bg-white border-b hover:bg-gray-50",
                                    td { class: "px-6 py-4", "#{review.id}" }
                                    td { class: "px-6 py-4", "{review.product_name}" }
                                    td { class: "px-6 py-4", "{review.user_name}" }
                                    td { class: "px-6 py-4",
                                        div { class: "flex text-yellow-400",
                                            for _ in 0..review.rating {
                                                span { "★" }
                                            }
                                            for _ in review.rating..5 {
                                                span { class: "text-gray-300", "★" }
                                            }
                                        }
                                    }
                                    td { class: "px-6 py-4", "{review.comment.clone().unwrap_or_default()}" }
                                    td { class: "px-6 py-4",
                                        span { 
                                            class: if review.status == "approved" { "px-2 py-1 bg-green-100 text-green-800 rounded text-xs" } else { "px-2 py-1 bg-yellow-100 text-yellow-800 rounded text-xs" },
                                            "{review.status}"
                                        }
                                    }
                                    td { class: "px-6 py-4", "{review.created_at.clone().unwrap_or_default()}" }
                                    td { class: "px-6 py-4",
                                        button {
                                            class: "text-blue-600 hover:text-blue-800 mr-2",
                                            onclick: move |_| toggle_status(id, status.clone()),
                                            if status == "approved" { "Reject" } else { "Approve" }
                                        }
                                        button {
                                            class: "text-red-600 hover:text-red-800",
                                            onclick: move |_| delete_review(id),
                                            "Delete"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                Pagination {
                    current_page: page(),
                    total_pages: (reviews().total as f64 / per_page() as f64).ceil() as i64,
                    on_page_change: move |p| page.set(p),
                }

                ConfirmDialog {
                    title: "Delete Review".to_string(),
                    message: "Are you sure you want to delete this review? This action cannot be undone.".to_string(),
                    is_open: *show_delete_dialog.read(),
                    on_confirm: confirm_delete,
                    on_cancel: move |_| show_delete_dialog.set(false),
                }
            }
        }
    }
}

