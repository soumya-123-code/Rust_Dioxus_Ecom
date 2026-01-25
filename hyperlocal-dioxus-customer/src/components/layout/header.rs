use dioxus::prelude::*;

use crate::app::Route;

#[derive(Props, PartialEq, Clone)]
pub struct HeaderProps {
    #[props(default)]
    pub title: String,
    #[props(default)]
    pub show_cart: bool,
    #[props(default)]
    pub show_search: bool,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    let navigator = use_navigator();

    rsx! {
        header {
            class: "bg-white shadow-sm sticky top-0 z-40",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4",
                div {
                    class: "flex items-center justify-between",
                    if props.title.is_empty() {
                        h1 {
                            class: "text-2xl font-bold text-gray-900",
                            "HyperLocal"
                        }
                    } else {
                        h1 {
                            class: "text-2xl font-bold text-gray-900",
                            {props.title}
                        }
                    }
                    div {
                        class: "flex gap-4",
                        if props.show_search {
                            button {
                                class: "p-2 text-gray-600 hover:text-gray-900",
                                onclick: move |_| {
                                    let _ = navigator.push(Route::Search {});
                                },
                                "🔍"
                            }
                        }
                        if props.show_cart {
                            button {
                                class: "p-2 text-gray-600 hover:text-gray-900 relative",
                                onclick: move |_| {
                                    let _ = navigator.push(Route::Cart {});
                                },
                                "🛒"
                            }
                        }
                    }
                }
            }
        }
    }
}
