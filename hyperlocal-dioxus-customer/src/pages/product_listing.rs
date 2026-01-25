use dioxus::prelude::*;
use crate::api::products::ProductsApi;
use crate::services::storage::StorageService;
use crate::config::AppConfig;

#[component]
pub fn ProductListing() -> Element {
    let mut products = use_signal(|| Vec::<serde_json::Value>::new());
    let mut loading = use_signal(|| true);

    use_effect(move || {
        let products_api = ProductsApi::new(AppConfig::api_base_url());
        spawn(async move {
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
            header {
                class: "bg-white shadow-sm",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4",
                    h1 {
                        class: "text-2xl font-bold text-gray-900",
                        "Products"
                    }
                }
            }
            main {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                if loading() {
                    div {
                        class: "text-center py-12",
                        "Loading products..."
                    }
                } else {
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
    }
}
