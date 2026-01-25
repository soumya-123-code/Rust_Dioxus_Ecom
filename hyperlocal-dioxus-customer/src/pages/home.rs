use dioxus::prelude::*;

use crate::components::layout::{Header, BottomNav};
use crate::components::ui::CategoryCard;
use crate::api::products::ProductsApi;
use crate::services::StorageService;
use crate::config::AppConfig;
use crate::app::Route;

#[component]
pub fn Home() -> Element {
    let mut categories = use_signal(|| Vec::<serde_json::Value>::new());
    let mut products = use_signal(|| Vec::<serde_json::Value>::new());
    let mut loading = use_signal(|| true);
    let navigator = use_navigator();

    use_effect(move || {
        let products_api = ProductsApi::new(AppConfig::api_base_url());
        spawn(async move {
            // Load categories
            if let Ok(data) = products_api.get_categories().await {
                if let Some(cats) = data.get("data").and_then(|d| d.as_array()) {
                    categories.set(cats.clone());
                }
            }

            // Load products
            let token = StorageService::get_token().ok().flatten();
            if let Ok(prod_data) = products_api.get_products(None, token.as_deref()).await {
                if let Some(prods) = prod_data.data {
                    // Convert products to JSON for display
                    products.set(vec![]);
                }
            }

            loading.set(false);
        });
    });

    rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            Header {
                show_cart: true,
                show_search: true,
            }

            // Main Content
            main {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                if loading() {
                    div {
                        class: "text-center py-12",
                        "Loading..."
                    }
                } else {
                    // Categories Section
                    div {
                        class: "mb-8",
                        h2 {
                            class: "text-xl font-semibold text-gray-900 mb-4",
                            "Categories"
                        }
                        div {
                            class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4",
                            for category in categories().iter().cloned() {
                                div {
                                    class: "bg-white rounded-lg shadow p-4 cursor-pointer hover:shadow-md transition-shadow",
                                    onclick: move |_| {
                                        if let Some(slug) = category.get("slug").and_then(|s| s.as_str()) {
                                            let _ = navigator.push(crate::app::Route::ProductListing {});
                                        }
                                    },
                                    if let Some(image) = category.get("image").and_then(|i| i.as_str()) {
                                        img {
                                            class: "w-full h-24 object-cover rounded mb-2",
                                            src: image.to_string(),
                                            alt: "Category"
                                        }
                                    }
                                    if let Some(name) = category.get("name").and_then(|n| n.as_str()) {
                                        p {
                                            class: "text-sm font-medium text-gray-900 text-center",
                                            {name.to_string()}
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Products Section
                    div {
                        h2 {
                            class: "text-xl font-semibold text-gray-900 mb-4",
                            "Featured Products"
                        }
                        div {
                            class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6",
                            // Product cards will be rendered here
                            p {
                                class: "text-gray-500",
                                "Products will be displayed here"
                            }
                        }
                    }
                }
            }

            BottomNav {}
        }
    }
}
