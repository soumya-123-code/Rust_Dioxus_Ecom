use dioxus::prelude::*;

use crate::services::storage::StorageService;
use crate::app::Route;

#[component]
pub fn Splash() -> Element {
    let navigator = use_navigator();

    use_effect(move || {
        spawn(async move {
            // Wait 2 seconds for splash screen
            gloo_timers::future::TimeoutFuture::new(2000).await;
            
            // Check if user is logged in
            if let Ok(Some(_token)) = StorageService::get_token() {
                let _ = navigator.push(Route::Home {});
            } else {
                let _ = navigator.push(Route::Login {});
            }
        });
    });

    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen bg-gradient-to-br from-blue-500 to-purple-600",
            div {
                class: "text-center",
                h1 {
                    class: "text-4xl font-bold text-white mb-4",
                    "HyperLocal"
                }
                div {
                    class: "animate-spin rounded-full h-12 w-12 border-b-2 border-white mx-auto",
                }
            }
        }
    }
}
