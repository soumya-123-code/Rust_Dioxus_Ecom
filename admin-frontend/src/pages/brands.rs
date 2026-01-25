use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api::{self, PaginatedResponse};
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, Modal};

#[derive(Debug, Clone, Deserialize)]
pub struct Brand {
    pub id: u64,
    pub uuid: String,
    pub title: String,
    pub slug: String,
    pub status: String,
    pub is_featured: Option<bool>,
}

#[component]
pub fn Brands() -> Element {
    let mut brands = use_signal(|| Vec::<Brand>::new());
    let mut loading = use_signal(|| true);
    let mut page = use_signal(|| 1i64);
    let mut total = use_signal(|| 0i64);
    let per_page = 10i64;
    
    let mut show_create_modal = use_signal(|| false);

    let fetch_data = move || {
        let current_page = *page.read();
        spawn(async move {
            loading.set(true);
            match api::get::<PaginatedResponse<Brand>>(&format!("/brands/datatable?page={}&per_page={}", current_page, per_page)).await {
                Ok(data) => {
                    brands.set(data.data);
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

    let columns = vec![
        Column { key: "id".to_string(), label: "ID".to_string(), sortable: true },
        Column { key: "title".to_string(), label: "Title".to_string(), sortable: true },
        Column { key: "slug".to_string(), label: "Slug".to_string(), sortable: false },
        Column { key: "status".to_string(), label: "Status".to_string(), sortable: true },
        Column { key: "featured".to_string(), label: "Featured".to_string(), sortable: false },
    ];

    let total_pages = (*total.read() + per_page - 1) / per_page;

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Brands" }
                    button {
                        class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700",
                        onclick: move |_| show_create_modal.set(true),
                        "+ Add Brand"
                    }
                }
                
                DataTable {
                    columns: columns.clone(),
                    loading: *loading.read(),
                    for brand in brands.read().iter() {
                        tr { key: "{brand.id}",
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{brand.id}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{brand.title}" }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{brand.slug}" }
                            td { class: "px-6 py-4 whitespace-nowrap",
                                span {
                                    class: if brand.status == "1" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" },
                                    if brand.status == "1" { "Active" } else { "Inactive" }
                                }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm",
                                if brand.is_featured.unwrap_or(false) {
                                    span { class: "text-yellow-500", "★" }
                                } else {
                                    span { class: "text-gray-300", "☆" }
                                }
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                button {
                                    class: "text-blue-600 hover:text-blue-900 mr-3",
                                    "Edit"
                                }
                                button {
                                    class: "text-red-600 hover:text-red-900",
                                    "Delete"
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
            
            Modal {
                title: "Add Brand".to_string(),
                is_open: *show_create_modal.read(),
                on_close: move |_| show_create_modal.set(false),
                BrandForm {
                    on_submit: move |_| {
                        show_create_modal.set(false);
                        fetch_data();
                    },
                }
            }
        }
    }
}

#[component]
fn BrandForm(on_submit: EventHandler<()>) -> Element {
    let mut title = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut image = use_signal(|| String::new());
    let mut status = use_signal(|| "1".to_string());
    let mut is_featured = use_signal(|| false);
    let mut loading = use_signal(|| false);

    let handle_submit = move |evt: Event<FormData>| {
        let title_val = title.read().clone();
        let description_val = description.read().clone();
        let image_val = image.read().clone();
        let status_val = status.read().clone();
        let is_featured_val = *is_featured.read();
        
        spawn(async move {
            loading.set(true);
            
            #[derive(Serialize)]
            struct CreateBrandRequest {
                title: String,
                description: String,
                image: String,
                banner: Option<String>,
                status: Option<String>,
                is_featured: Option<bool>,
            }
            
            let body = CreateBrandRequest {
                title: title_val,
                description: description_val,
                image: image_val,
                banner: None,
                status: Some(status_val),
                is_featured: Some(is_featured_val),
            };
            
            match api::post::<serde_json::Value, _>("/brands", &body).await {
                Ok(_) => on_submit.call(()),
                Err(e) => println!("Error creating brand: {}", e),
            }
            
            loading.set(false);
        });
    };

    rsx! {
        form { onsubmit: handle_submit,
            div { class: "mb-4",
                label { class: "block text-sm font-medium text-gray-700 mb-1", "Title" }
                input {
                    r#type: "text",
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    value: "{title}",
                    oninput: move |evt| title.set(evt.value()),
                    required: true,
                }
            }
            div { class: "mb-4",
                label { class: "block text-sm font-medium text-gray-700 mb-1", "Description" }
                textarea {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    rows: "3",
                    value: "{description}",
                    oninput: move |evt| description.set(evt.value()),
                    required: true,
                }
            }
            div { class: "mb-4",
                label { class: "block text-sm font-medium text-gray-700 mb-1", "Image URL" }
                input {
                    r#type: "text",
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    value: "{image}",
                    oninput: move |evt| image.set(evt.value()),
                    required: true,
                }
            }
            div { class: "mb-4",
                label { class: "block text-sm font-medium text-gray-700 mb-1", "Status" }
                select {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    value: "{status}",
                    onchange: move |evt| status.set(evt.value()),
                    option { value: "1", "Active" }
                    option { value: "0", "Inactive" }
                }
            }
            div { class: "mb-4 flex items-center",
                input {
                    r#type: "checkbox",
                    id: "is_featured",
                    class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded",
                    checked: "{is_featured}",
                    onchange: move |evt| is_featured.set(evt.value() == "true"),
                }
                label { class: "ml-2 block text-sm text-gray-900", r#for: "is_featured", "Featured Brand" }
            }
            div { class: "flex justify-end space-x-3",
                button {
                    r#type: "submit",
                    class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50",
                    disabled: *loading.read(),
                    if *loading.read() { "Saving..." } else { "Save Brand" }
                }
            }
        }
    }
}
