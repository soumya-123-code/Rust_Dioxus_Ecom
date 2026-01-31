use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, Modal, ConfirmDialog};
use crate::Route;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub status: String,
    pub role: String,
    pub created_at: String,
    pub last_login: Option<String>,
}

#[derive(Debug, Serialize)]
struct UpdateStatusRequest {
    status: String,
}

#[component]
pub fn Users() -> Element {
    let mut users = use_signal(|| Vec::<User>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;

    let mut show_delete_confirm = use_signal(|| Option::<u64>::None);
    let mut status_filter = use_signal(|| "".to_string());

    let fetch_data = move || {
        let current_page = *page.read();
        let status = status_filter.read().clone();
        spawn(async move {
            loading.set(true);
            let mut url = format!("/users/datatable?page={}&per_page={}", current_page, per_page);
            if !status.is_empty() {
                url.push_str(&format!("&status={}", status));
            }
            match api::get::<PaginatedResponse<User>>(&url).await {
                Ok(data) => {
                    users.set(data.data);
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
                match api::delete(&format!("/users/{}", id)).await {
                    Ok(_) => {
                        show_delete_confirm.set(None);
                        fetch_data();
                    }
                    Err(_e) => println!("Error deleting user"),
                }
            });
        }
    };

    // Toggle user status (active/suspended)
    let toggle_status = move |id: u64, current_status: String| {
        spawn(async move {
            let new_status = if current_status == "active" { "suspended" } else { "active" };
            let req = UpdateStatusRequest { status: new_status.to_string() };
            match api::put::<serde_json::Value, _>(&format!("/users/{}/status", id), &req).await {
                Ok(_) => fetch_data(),
                Err(_e) => println!("Error updating status"),
            }
        });
    };

    let columns = vec![
        Column { key: "email".to_string(), label: "Email".to_string(), sortable: true },
        Column { key: "name".to_string(), label: "Name".to_string(), sortable: true },
        Column { key: "phone".to_string(), label: "Phone".to_string(), sortable: false },
        Column { key: "role".to_string(), label: "Role".to_string(), sortable: true },
        Column { key: "status".to_string(), label: "Status".to_string(), sortable: true },
        Column { key: "created_at".to_string(), label: "Created".to_string(), sortable: true },
        Column { key: "actions".to_string(), label: "Actions".to_string(), sortable: false },
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;
    let users_list = users.read().clone();

    rsx! {
        div { class: "admin-layout",
            Sidebar {}
            div { class: "admin-main",
                Header {}
                main { class: "admin-content",
                    div { class: "page-header",
                        h1 { class: "page-header__title", "Users" }
                    }

                    // Filter Bar
                    div { class: "filter-bar",
                        div { class: "filter-bar__item",
                            label { class: "filter-label", "Status" }
                            select {
                                class: "filter-select",
                                value: "{status_filter}",
                                onchange: move |e| {
                                    status_filter.set(e.value());
                                    page.set(1);
                                    fetch_data();
                                },
                                option { value: "", "All Status" }
                                option { value: "active", "Active" }
                                option { value: "suspended", "Suspended" }
                            }
                        }
                        button {
                            class: "btn btn--outline",
                            onclick: move |_| {
                                status_filter.set("".to_string());
                                page.set(1);
                                fetch_data();
                            },
                            "Reset Filters"
                        }
                    }

                    DataTable {
                        columns: columns.clone(),
                        loading: *loading.read(),
                        for user in users_list {
                            {
                                let user_id = user.id;
                                let user_status = user.status.clone();
                                let is_active = user.status == "active";

                                rsx! {
                                    tr { class: "table-row", key: "{user.id}",
                                        td { class: "table-cell", "{user.email}" }
                                        td { class: "table-cell table-cell--bold", "{user.name}" }
                                        td { class: "table-cell", "{user.phone}" }
                                        td { class: "table-cell",
                                            span { class: "role-badge", "{user.role}" }
                                        }
                                        // Active/Suspended Switch
                                        td { class: "table-cell",
                                            label { class: "switch",
                                                input {
                                                    r#type: "checkbox",
                                                    checked: is_active,
                                                    onchange: {
                                                        let status = user_status.clone();
                                                        move |_| toggle_status(user_id, status.clone())
                                                    }
                                                }
                                                span { class: "switch__slider" }
                                            }
                                        }
                                        td { class: "table-cell",
                                            "{user.created_at}"
                                        }
                                        td { class: "table-cell table-actions-cell",
                                            Link {
                                                to: Route::UserDetail { id: user.id.to_string() },
                                                class: "btn btn--text btn--primary-text",
                                                "View"
                                            }
                                            button {
                                                class: "btn btn--text btn--danger-text",
                                                onclick: move |_| show_delete_confirm.set(Some(user_id)),
                                                "Delete"
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

            ConfirmDialog {
                title: "Delete User".to_string(),
                message: "Are you sure you want to delete this user? This action cannot be undone.".to_string(),
                is_open: show_delete_confirm.read().is_some(),
                on_confirm: handle_delete_confirm,
                on_cancel: move |_| show_delete_confirm.set(None),
            }
        }
    }
}

#[component]
pub fn UserDetail(id: String) -> Element {
    let mut user = use_signal(|| Option::<User>::None);
    let mut loading = use_signal(|| true);

    let user_id: u64 = id.parse().unwrap_or(0);

    let fetch_user = move || {
        spawn(async move {
            #[derive(Deserialize)]
            struct Response {
                data: Option<User>,
            }

            match api::get::<Response>(&format!("/users/{}", user_id)).await {
                Ok(resp) => user.set(resp.data),
                Err(_) => {}
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_user();
    });

    // Toggle user status
    let toggle_status = move |current_status: String| {
        spawn(async move {
            let new_status = if current_status == "active" { "suspended" } else { "active" };
            let req = UpdateStatusRequest { status: new_status.to_string() };
            match api::put::<serde_json::Value, _>(&format!("/users/{}/status", user_id), &req).await {
                Ok(_) => fetch_user(),
                Err(_e) => println!("Error updating status"),
            }
        });
    };

    rsx! {
        div { class: "admin-layout",
            Sidebar {}
            div { class: "admin-main",
                Header {}
                main { class: "admin-content",
                    div { class: "page-header",
                        Link {
                            to: Route::Users {},
                            class: "btn btn--text",
                            "← Back to Users"
                        }
                    }

                    if *loading.read() {
                        div { class: "loading-container",
                            p { class: "loading-text", "Loading..." }
                        }
                    } else if let Some(u) = user.read().as_ref() {
                        {
                            let is_active = u.status == "active";
                            let current_status = u.status.clone();

                            rsx! {
                                div { class: "detail-card",
                                    div { class: "detail-card__header",
                                        h1 { class: "detail-card__title", "{u.name}" }
                                        div { class: "detail-card__actions",
                                            button {
                                                class: "btn btn--outline",
                                                onclick: move |_| toggle_status(current_status.clone()),
                                                if is_active { "Suspend" } else { "Activate" }
                                            }
                                        }
                                    }

                                    div { class: "detail-grid",
                                        div { class: "detail-item",
                                            span { class: "detail-item__label", "Email" }
                                            span { class: "detail-item__value", "{u.email}" }
                                        }
                                        div { class: "detail-item",
                                            span { class: "detail-item__label", "Name" }
                                            span { class: "detail-item__value", "{u.name}" }
                                        }
                                        div { class: "detail-item",
                                            span { class: "detail-item__label", "Phone" }
                                            span { class: "detail-item__value", "{u.phone}" }
                                        }
                                        div { class: "detail-item",
                                            span { class: "detail-item__label", "Role" }
                                            span { class: "detail-item__value", "{u.role}" }
                                        }
                                        div { class: "detail-item",
                                            span { class: "detail-item__label", "Status" }
                                            span {
                                                class: if is_active { "status-badge status-badge--active" } else { "status-badge status-badge--pending" },
                                                if is_active { "Active" } else { "Suspended" }
                                            }
                                        }
                                        div { class: "detail-item",
                                            span { class: "detail-item__label", "Created" }
                                            span { class: "detail-item__value", "{u.created_at}" }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "empty-state",
                            p { class: "empty-state__text", "User not found" }
                        }
                    }
                }
            }
        }
    }
}
