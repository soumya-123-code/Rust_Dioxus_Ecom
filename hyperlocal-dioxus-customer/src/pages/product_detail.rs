use dioxus::prelude::*;

use crate::api::{ProductsApi, CartApi};
use crate::services::storage::StorageService;
use crate::config::AppConfig;

#[component]
pub fn ProductDetail(slug: String) -> Element {
    let mut product = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| true);
    let mut quantity = use_signal(|| 1);

    let slug_clone = slug.clone();
    use_effect(move || {
        let slug = slug_clone.clone();
        let products_api = ProductsApi::new(AppConfig::api_base_url());
        spawn(async move {
            let token = StorageService::get_token().ok().flatten();
            if let Ok(prod) = products_api.get_product_detail(&slug, token.as_deref()).await {
                // Convert product to JSON for display
                product.set(Some(serde_json::json!({})));
            }
            loading.set(false);
        });
    });

    rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            if loading() {
                div {
                    class: "text-center py-12",
                    "Loading product..."
                }
            } else if let Some(prod) = product() {
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                    // Product details will be rendered here
                    div {
                        class: "bg-white rounded-lg shadow p-6",
                        h1 {
                            class: "text-2xl font-bold mb-4",
                            "Product Details"
                        }
                        div {
                            class: "flex gap-4 items-center mt-6",
                            button {
                                class: "px-4 py-2 border rounded",
                                onclick: move |_| {
                                    if quantity() > 1 {
                                        quantity.set(quantity() - 1);
                                    }
                                },
                                "-"
                            }
                            span {
                                class: "text-lg font-semibold",
                                { format!("{}", quantity()) }
                            }
                            button {
                                class: "px-4 py-2 border rounded",
                                onclick: move |_| quantity.set(quantity() + 1),
                                "+"
                            }
                            button {
                                class: "ml-4 px-6 py-2 bg-indigo-600 text-white rounded hover:bg-indigo-700",
                                onclick: move |_| {
                                    spawn(async move {
                                        if let Ok(Some(token)) = StorageService::get_token() {
                                            // Add to cart logic
                                        }
                                    });
                                },
                                "Add to Cart"
                            }
                        }
                    }
                }
            }
        }
    }
}
