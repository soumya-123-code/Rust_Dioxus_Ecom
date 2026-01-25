use dioxus::prelude::*;


#[component]
pub fn NearByStoreDetails(slug: String) -> Element {
    
    rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            header {
                class: "bg-white shadow-sm",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4",
                    h1 {
                        class: "text-2xl font-bold text-gray-900",
                        "Store Details"
                    }
                }
            }
            main {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                p { { format!("Store: {}", slug) } }
            }
        }
    }
}
