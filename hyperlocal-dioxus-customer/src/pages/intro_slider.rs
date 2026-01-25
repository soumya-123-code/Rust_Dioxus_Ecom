use dioxus::prelude::*;

#[component]
pub fn IntroSlider() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50 flex items-center justify-center",
            div {
                class: "text-center",
                h1 {
                    class: "text-3xl font-bold mb-4",
                    "Welcome to HyperLocal"
                }
                p {
                    class: "text-gray-600",
                    "Introduction slider page"
                }
            }
        }
    }
}
