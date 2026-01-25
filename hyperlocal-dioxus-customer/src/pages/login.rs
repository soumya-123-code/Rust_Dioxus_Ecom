use dioxus::prelude::*;
use crate::components::forms::LoginForm;
use crate::app::Route;

#[component]
pub fn Login() -> Element {
    let navigator = use_navigator();

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8",
            div {
                class: "max-w-md w-full space-y-8",
                div {
                    class: "text-center",
                    h2 {
                        class: "text-3xl font-extrabold text-gray-900",
                        "Sign in to your account"
                    }
                }
                LoginForm {}
                div {
                    class: "text-center mt-4",
                    a {
                        class: "font-medium text-indigo-600 hover:text-indigo-500",
                        href: "#",
                        onclick: move |_| {
                            let _ = navigator.push(Route::Register {});
                        },
                        "Don't have an account? Sign up"
                    }
                }
            }
        }
    }
}
