use dioxus::prelude::*;
use crate::components::layout::Header;
use crate::components::common::EmptyState;
use crate::api::cart::CartApi;
use crate::services::StorageService;
use crate::config::AppConfig;
use crate::app::Route;

#[component]
pub fn Cart() -> Element {
    let mut cart_items = use_signal(|| Vec::<serde_json::Value>::new());
    let mut loading = use_signal(|| true);
    let mut total = use_signal(|| 0.0);

    use_effect(move || {
        let cart_api = CartApi::new(AppConfig::api_base_url());
        spawn(async move {
            if let Ok(Some(token)) = StorageService::get_token() {
                if let Ok(cart) = cart_api.get_cart(&token).await {
                    if let Some(items) = cart.items {
                        // Convert cart items to JSON for display
                        cart_items.set(vec![]);
                    }
                    if let Some(amt) = cart.total_amount {
                        total.set(amt);
                    }
                }
            }
            loading.set(false);
        });
    });

    rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            Header {
                title: "Shopping Cart".to_string(),
                show_cart: false,
            }
            main {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                if loading() {
                    div {
                        class: "text-center py-12",
                        "Loading cart..."
                    }
                } else if cart_items().is_empty() {
                    EmptyState {
                        title: "Your cart is empty".to_string(),
                        description: "Add some products to get started".to_string(),
                        action_text: "Continue Shopping".to_string(),
                        on_action: None,
                    }
                } else {
                    div {
                        class: "bg-white rounded-lg shadow p-6",
                        // Cart items will be rendered here
                        div {
                            class: "border-t pt-4 mt-4",
                            div {
                                class: "flex justify-between text-lg font-semibold",
                                span { "Total: " }
                                span { "₹{total():.2}" }
                            }
                            button {
                                class: "w-full mt-4 py-3 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 font-medium",
                                "Proceed to Checkout"
                            }
                        }
                    }
                }
            }
        }
    }
}
