use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, Modal, ConfirmDialog};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Banner {
    pub id: u64,
    pub title: String,
    pub image: String,
    pub status: String,
    pub banner_type: Option<String>,
    pub position: Option<String>,
    pub link_type: Option<String>,
    pub link_value: Option<String>,
    pub sort_order: Option<i32>,
}

#[component]
pub fn Banners() -> Element {
    let mut banners = use_signal(|| Vec::<Banner>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;

    let mut show_create_modal = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| false);
    let mut show_delete_dialog = use_signal(|| false);
    let mut selected_banner = use_signal(|| None::<Banner>);

    // Form signals
    let mut title = use_signal(|| "".to_string());
    let mut image = use_signal(|| "".to_string());
    // banner_type is not used
    
    let mut link_type = use_signal(|| "".to_string());
    let mut link_value = use_signal(|| "".to_string());
    let mut position = use_signal(|| "main_banner".to_string());
    let mut sort_order = use_signal(|| 0i32);
    let mut status = use_signal(|| "active".to_string());
    let mut saving = use_signal(|| false);

    let fetch_data = move || {
        let current_page = *page.read();
        spawn(async move {
            loading.set(true);
            match api::get::<PaginatedResponse<Banner>>(&format!("/banners/datatable?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => {
                    banners.set(data.data);
                    total.set(data.total);
                }
                Err(_) => {}
            }
            loading.set(false);
        });
    };

    use_effect(move || { fetch_data(); });

    let mut reset_form = move || {
        title.set("".to_string());
        image.set("".to_string());
        link_type.set("".to_string());
        link_value.set("".to_string());
        position.set("main_banner".to_string());
        sort_order.set(0);
        status.set("active".to_string());
    };

    let open_create_modal = move |_| {
        reset_form();
        show_create_modal.set(true);
    };

    let mut open_edit_modal = move |banner: Banner| {
        selected_banner.set(Some(banner.clone()));
        title.set(banner.title);
        image.set(banner.image);
        link_type.set(banner.link_type.unwrap_or_default());
        link_value.set(banner.link_value.unwrap_or_default());
        position.set(banner.position.unwrap_or_else(|| "main_banner".to_string()));
        sort_order.set(banner.sort_order.unwrap_or(0));
        status.set(banner.status);
        show_edit_modal.set(true);
    };

    let mut open_delete_dialog = move |banner: Banner| {
        selected_banner.set(Some(banner));
        show_delete_dialog.set(true);
    };

    let handle_create = move |_| {
        spawn(async move {
            saving.set(true);
            let req = serde_json::json!({
                "title": title(),
                "image": image(),
                "link_type": if link_type().is_empty() { None } else { Some(link_type()) },
                "link_value": if link_value().is_empty() { None } else { Some(link_value()) },
                "position": position(),
                "sort_order": sort_order(),
                "status": status()
            });

            match api::post::<serde_json::Value, _>("/banners", &req).await {
                Ok(_) => {
                    show_create_modal.set(false);
                    fetch_data();
                }
                Err(e) => println!("Error creating banner: {}", e),
            }
            saving.set(false);
        });
    };

    let handle_update = move |_| {
        if let Some(banner) = selected_banner.read().as_ref() {
            let id = banner.id;
            spawn(async move {
                saving.set(true);
                let req = serde_json::json!({
                    "title": title(),
                    "image": image(),
                    "link_type": if link_type().is_empty() { None } else { Some(link_type()) },
                    "link_value": if link_value().is_empty() { None } else { Some(link_value()) },
                    "position": position(),
                    "sort_order": sort_order(),
                    "status": status()
                });

                match api::put::<serde_json::Value, _>(&format!("/banners/{}", id), &req).await {
                    Ok(_) => {
                        show_edit_modal.set(false);
                        fetch_data();
                    }
                    Err(e) => println!("Error updating banner: {}", e),
                }
                saving.set(false);
            });
        }
    };

    let handle_delete = move |_| {
        if let Some(banner) = selected_banner.read().as_ref() {
            let id = banner.id;
            spawn(async move {
                match api::delete(&format!("/banners/{}", id)).await {
                    Ok(_) => {
                        show_delete_dialog.set(false);
                        fetch_data();
                    }
                    Err(e) => println!("Error deleting banner: {}", e),
                }
            });
        }
    };

    let columns = vec![
        Column { key: "id".to_string(), label: "ID".to_string(), sortable: true },
        Column { key: "title".to_string(), label: "Title".to_string(), sortable: true },
        Column { key: "position".to_string(), label: "Position".to_string(), sortable: false },
        Column { key: "status".to_string(), label: "Status".to_string(), sortable: true },
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;
    let current_banners = banners.read().clone();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Banners" }
                    button {
                        class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700",
                        onclick: open_create_modal,
                        "+ Add Banner"
                    }
                }
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for banner in current_banners {
                        {
                            let b1 = banner.clone();
                            let b2 = banner.clone();
                            rsx! {
                                tr { key: "{banner.id}", class: "border-b bg-white hover:bg-gray-50",
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{banner.id}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                        div { class: "flex items-center",
                                            img { class: "h-10 w-10 rounded-full object-cover mr-3", src: "{banner.image}", alt: "" }
                                            "{banner.title}"
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{banner.position.clone().unwrap_or_default()}" }
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        span {
                                            class: if banner.status == "active" || banner.status == "1" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" },
                                            if banner.status == "active" || banner.status == "1" { "Active" } else { "Inactive" }
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                        button {
                                            class: "text-blue-600 hover:text-blue-900 mr-3",
                                            onclick: move |_| open_edit_modal(b1.clone()),
                                            "Edit"
                                        }
                                        button {
                                            class: "text-red-600 hover:text-red-900",
                                            onclick: move |_| open_delete_dialog(b2.clone()),
                                            "Delete"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if total_pages > 1 { Pagination { current_page: *page.read(), total_pages: total_pages, on_page_change: move |new_page| { page.set(new_page); fetch_data(); } } }
            }

            // Create Modal
            Modal {
                title: "Add Banner".to_string(),
                is_open: *show_create_modal.read(),
                on_close: move |_| show_create_modal.set(false),
                div { class: "space-y-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Title" }
                        input {
                            r#type: "text",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{title}",
                            oninput: move |e| title.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Image URL" }
                        input {
                            r#type: "text",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{image}",
                            oninput: move |e| image.set(e.value())
                        }
                    }
                    div { class: "grid grid-cols-2 gap-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700", "Position" }
                            select {
                                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                                value: "{position}",
                                onchange: move |e| position.set(e.value()),
                                option { value: "main_banner", "Main Banner" }
                                option { value: "popup_banner", "Popup Banner" }
                                option { value: "footer_banner", "Footer Banner" }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700", "Sort Order" }
                            input {
                                r#type: "number",
                                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                                value: "{sort_order}",
                                oninput: move |e| sort_order.set(e.value().parse().unwrap_or(0))
                            }
                        }
                    }
                    div { class: "grid grid-cols-2 gap-4",
                         div {
                            label { class: "block text-sm font-medium text-gray-700", "Link Type" }
                            select {
                                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                                value: "{link_type}",
                                onchange: move |e| link_type.set(e.value()),
                                option { value: "", "Select Type" }
                                option { value: "store", "Store" }
                                option { value: "product", "Product" }
                                option { value: "category", "Category" }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700", "Link Value (ID)" }
                            input {
                                r#type: "text",
                                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                                value: "{link_value}",
                                oninput: move |e| link_value.set(e.value())
                            }
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Status" }
                        select {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{status}",
                            onchange: move |e| status.set(e.value()),
                            option { value: "active", "Active" }
                            option { value: "inactive", "Inactive" }
                        }
                    }
                    div { class: "flex justify-end pt-4",
                        button {
                            class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50",
                            disabled: *saving.read(),
                            onclick: handle_create,
                            if *saving.read() { "Saving..." } else { "Create Banner" }
                        }
                    }
                }
            }

            // Edit Modal
            Modal {
                title: "Edit Banner".to_string(),
                is_open: *show_edit_modal.read(),
                on_close: move |_| show_edit_modal.set(false),
                div { class: "space-y-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Title" }
                        input {
                            r#type: "text",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{title}",
                            oninput: move |e| title.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Image URL" }
                        input {
                            r#type: "text",
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{image}",
                            oninput: move |e| image.set(e.value())
                        }
                    }
                    div { class: "grid grid-cols-2 gap-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700", "Position" }
                            select {
                                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                                value: "{position}",
                                onchange: move |e| position.set(e.value()),
                                option { value: "main_banner", "Main Banner" }
                                option { value: "popup_banner", "Popup Banner" }
                                option { value: "footer_banner", "Footer Banner" }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700", "Sort Order" }
                            input {
                                r#type: "number",
                                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                                value: "{sort_order}",
                                oninput: move |e| sort_order.set(e.value().parse().unwrap_or(0))
                            }
                        }
                    }
                    div { class: "grid grid-cols-2 gap-4",
                         div {
                            label { class: "block text-sm font-medium text-gray-700", "Link Type" }
                            select {
                                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                                value: "{link_type}",
                                onchange: move |e| link_type.set(e.value()),
                                option { value: "", "Select Type" }
                                option { value: "store", "Store" }
                                option { value: "product", "Product" }
                                option { value: "category", "Category" }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700", "Link Value (ID)" }
                            input {
                                r#type: "text",
                                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                                value: "{link_value}",
                                oninput: move |e| link_value.set(e.value())
                            }
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Status" }
                        select {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{status}",
                            onchange: move |e| status.set(e.value()),
                            option { value: "active", "Active" }
                            option { value: "inactive", "Inactive" }
                        }
                    }
                    div { class: "flex justify-end pt-4",
                        button {
                            class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50",
                            disabled: *saving.read(),
                            onclick: handle_update,
                            if *saving.read() { "Saving..." } else { "Save Changes" }
                        }
                    }
                }
            }

            // Delete Dialog
            ConfirmDialog {
                title: "Delete Banner".to_string(),
                message: "Are you sure you want to delete this banner? This action cannot be undone.".to_string(),
                is_open: *show_delete_dialog.read(),
                on_confirm: handle_delete,
                on_cancel: move |_| show_delete_dialog.set(false),
            }
        }
    }
}
