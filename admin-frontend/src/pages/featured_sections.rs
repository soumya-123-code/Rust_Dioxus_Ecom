use dioxus::prelude::*;
use crate::components::{Sidebar, Header, DataTable, Column, Modal, ConfirmDialog};
use crate::api;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FeaturedSection {
    pub id: u64,
    pub uuid: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub section_type: String,
    pub layout_type: Option<String>,
    pub product_filter: Option<String>,
    pub product_limit: Option<i32>,
    pub sort_order: Option<i32>,
    pub status: String,
}

#[component]
pub fn FeaturedSections() -> Element {
    let mut sections = use_signal(|| Vec::<FeaturedSection>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);
    
    // Modal states
    let mut show_create_modal = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| false);
    let mut show_delete_dialog = use_signal(|| false);
    let mut selected_section = use_signal(|| Option::<FeaturedSection>::None);
    let mut saving = use_signal(|| false);

    // Form signals
    let mut title = use_signal(|| String::new());
    let mut subtitle = use_signal(|| String::new());
    let mut section_type = use_signal(|| "products".to_string());
    let mut layout_type = use_signal(|| "grid".to_string());
    let mut product_filter = use_signal(|| "".to_string());
    let mut product_limit = use_signal(|| 10);
    let mut sort_order = use_signal(|| 0);
    let mut status = use_signal(|| "active".to_string());

    let fetch_data = move || {
        spawn(async move {
            loading.set(true);
            match api::get::<Vec<FeaturedSection>>("/featured-sections").await {
                Ok(data) => sections.set(data),
                Err(e) => error.set(Some(e.to_string())),
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let mut reset_form = move || {
        title.set(String::new());
        subtitle.set(String::new());
        section_type.set("products".to_string());
        layout_type.set("grid".to_string());
        product_filter.set("".to_string());
        product_limit.set(10);
        sort_order.set(0);
        status.set("active".to_string());
    };

    let open_create_modal = move |_| {
        reset_form();
        show_create_modal.set(true);
    };

    let mut open_edit_modal = move |s: FeaturedSection| {
        selected_section.set(Some(s.clone()));
        title.set(s.title);
        subtitle.set(s.subtitle.unwrap_or_default());
        section_type.set(s.section_type);
        layout_type.set(s.layout_type.unwrap_or("grid".to_string()));
        product_filter.set(s.product_filter.unwrap_or_default());
        product_limit.set(s.product_limit.unwrap_or(10));
        sort_order.set(s.sort_order.unwrap_or(0));
        status.set(s.status);
        show_edit_modal.set(true);
    };

    let mut open_delete_dialog = move |s: FeaturedSection| {
        selected_section.set(Some(s));
        show_delete_dialog.set(true);
    };

    let generate_uuid = || {
        format!("fs-{}", chrono::Utc::now().timestamp_millis())
    };

    let handle_create = move |_| {
        spawn(async move {
            saving.set(true);
            let req = serde_json::json!({
                "uuid": generate_uuid(),
                "title": title(),
                "subtitle": if subtitle().is_empty() { None } else { Some(subtitle()) },
                "section_type": section_type(),
                "layout_type": if layout_type().is_empty() { None } else { Some(layout_type()) },
                "product_filter": if product_filter().is_empty() { None } else { Some(product_filter()) },
                "product_limit": Some(product_limit()),
                "sort_order": Some(sort_order()),
                "status": status()
            });

            match api::post::<serde_json::Value, _>("/featured-sections", &req).await {
                Ok(_) => {
                    show_create_modal.set(false);
                    fetch_data();
                }
                Err(e) => println!("Error creating section: {}", e),
            }
            saving.set(false);
        });
    };

    let handle_update = move |_| {
        spawn(async move {
            if let Some(s) = selected_section() {
                saving.set(true);
                let req = serde_json::json!({
                    "title": Some(title()),
                    "subtitle": if subtitle().is_empty() { None } else { Some(subtitle()) },
                    "section_type": Some(section_type()),
                    "layout_type": if layout_type().is_empty() { None } else { Some(layout_type()) },
                    "product_filter": if product_filter().is_empty() { None } else { Some(product_filter()) },
                    "product_limit": Some(product_limit()),
                    "sort_order": Some(sort_order()),
                    "status": Some(status())
                });

                match api::put::<serde_json::Value, _>(&format!("/featured-sections/{}", s.id), &req).await {
                    Ok(_) => {
                        show_edit_modal.set(false);
                        fetch_data();
                    }
                    Err(e) => println!("Error updating section: {}", e),
                }
                saving.set(false);
            }
        });
    };

    let handle_delete = move |_| {
        spawn(async move {
            if let Some(s) = selected_section() {
                match api::delete(&format!("/featured-sections/{}", s.id)).await {
                    Ok(_) => {
                        show_delete_dialog.set(false);
                        fetch_data();
                    }
                    Err(e) => println!("Error deleting section: {}", e),
                }
            }
        });
    };

    let handle_save = move |_| {
        if *show_create_modal.read() {
            handle_create(());
        } else {
            handle_update(());
        }
    };

    let columns = vec![
        Column::new("ID", "id"),
        Column::new("Title", "title"),
        Column::new("Type", "section_type"),
        Column::new("Layout", "layout_type"),
        Column::new("Status", "status"),
        Column::new("Order", "sort_order"),
    ];

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Featured Sections" }
                    button {
                        class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg flex items-center",
                        onclick: open_create_modal,
                        "Add Section"
                    }
                }

                DataTable {
                    columns: columns,
                    loading: *loading.read(),
                    for s in sections.read().iter() {
                        {
                            let s1 = s.clone();
                            let s2 = s.clone();
                            rsx! {
                                tr { key: "{s.id}", class: "hover:bg-gray-50",
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{s.id}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{s.title}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{s.section_type}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{s.layout_type.clone().unwrap_or_default()}" }
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        span {
                                            class: if s.status == "active" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" },
                                            "{s.status}"
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{s.sort_order.unwrap_or(0)}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                        button {
                                            class: "text-blue-600 hover:text-blue-900 mr-3",
                                            onclick: move |_| open_edit_modal(s1.clone()),
                                            "Edit"
                                        }
                                        button {
                                            class: "text-red-600 hover:text-red-900",
                                            onclick: move |_| open_delete_dialog(s2.clone()),
                                            "Delete"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Create/Edit Modal Content
        Modal {
            title: if *show_create_modal.read() { "Create Section".to_string() } else { "Edit Section".to_string() },
            is_open: *show_create_modal.read() || *show_edit_modal.read(),
            on_close: move |_| {
                show_create_modal.set(false);
                show_edit_modal.set(false);
            },
            div { class: "space-y-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Title" }
                    input {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        r#type: "text",
                        value: "{title}",
                        oninput: move |e| title.set(e.value())
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Subtitle" }
                    input {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        r#type: "text",
                        value: "{subtitle}",
                        oninput: move |e| subtitle.set(e.value())
                    }
                }
                div { class: "grid grid-cols-2 gap-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Section Type" }
                        select {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{section_type}",
                            onchange: move |e| section_type.set(e.value()),
                            option { value: "products", "Products" }
                            option { value: "categories", "Categories" }
                            option { value: "brands", "Brands" }
                            option { value: "banner", "Banner" }
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Layout Type" }
                        select {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{layout_type}",
                            onchange: move |e| layout_type.set(e.value()),
                            option { value: "grid", "Grid" }
                            option { value: "slider", "Slider" }
                            option { value: "list", "List" }
                        }
                    }
                }
                div { class: "grid grid-cols-2 gap-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Product Limit" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            r#type: "number",
                            value: "{product_limit}",
                            oninput: move |e| product_limit.set(e.value().parse().unwrap_or(10))
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Sort Order" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            r#type: "number",
                            value: "{sort_order}",
                            oninput: move |e| sort_order.set(e.value().parse().unwrap_or(0))
                        }
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Product Filter (e.g. 'new_arrival', 'best_selling')" }
                    input {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        r#type: "text",
                        value: "{product_filter}",
                        oninput: move |e| product_filter.set(e.value())
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
                        class: "mr-3 px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50",
                        onclick: move |_| {
                            show_create_modal.set(false);
                            show_edit_modal.set(false);
                        },
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 disabled:opacity-50",
                        onclick: handle_save,
                        disabled: *saving.read(),
                        if *saving.read() { "Saving..." } else { "Save" }
                    }
                }
            }
        }

        ConfirmDialog {
            title: "Delete Section".to_string(),
            message: "Are you sure you want to delete this featured section? This action cannot be undone.".to_string(),
            is_open: *show_delete_dialog.read(),
            on_confirm: handle_delete,
            on_cancel: move |_| show_delete_dialog.set(false),
        }
    }
}
