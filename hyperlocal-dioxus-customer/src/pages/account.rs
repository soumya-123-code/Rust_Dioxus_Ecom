use dioxus::prelude::*;

#[component]
pub fn Account() -> Element {
    rsx! {
        div {
            style: "min-height: 100vh; background-color: #f9fafb;",
            header {
                style: "background-color: white; box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);",
                div {
                    style: "max-width: 80rem; margin: 0 auto; padding: 1rem 1rem; padding-top: 1rem; padding-bottom: 1rem;",
                    h1 {
                        style: "font-size: 1.5rem; font-weight: 700; color: #111827;",
                        "Account"
                    }
                }
            }
            main {
                style: "max-width: 80rem; margin: 0 auto; padding: 2rem 1rem;",
                p { "Account page" }
            }
        }
    }
}