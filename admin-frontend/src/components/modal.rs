use dioxus::prelude::*;

#[component]
pub fn Modal(
    title: String,
    is_open: bool,
    on_close: EventHandler<()>,
    children: Element,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    rsx! {
        div { class: "fixed inset-0 z-50 overflow-y-auto",
            div { class: "flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0",
                div {
                    class: "fixed inset-0 transition-opacity",
                    onclick: move |_| on_close.call(()),
                    div { class: "absolute inset-0 bg-gray-500 opacity-75" }
                }
                div { class: "inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full",
                    div { class: "bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4",
                        div { class: "flex items-center justify-between mb-4",
                            h3 { class: "text-lg leading-6 font-medium text-gray-900", "{title}" }
                            button {
                                class: "text-gray-400 hover:text-gray-600",
                                onclick: move |_| on_close.call(()),
                                "✕"
                            }
                        }
                        {children}
                    }
                }
            }
        }
    }
}

#[component]
pub fn ConfirmDialog(
    title: String,
    message: String,
    is_open: bool,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    rsx! {
        div { class: "fixed inset-0 z-50 overflow-y-auto",
            div { class: "flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0",
                div { class: "fixed inset-0 transition-opacity",
                    div { class: "absolute inset-0 bg-gray-500 opacity-75" }
                }
                div { class: "inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full",
                    div { class: "bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4",
                        h3 { class: "text-lg leading-6 font-medium text-gray-900 mb-2", "{title}" }
                        p { class: "text-sm text-gray-500", "{message}" }
                    }
                    div { class: "bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse",
                        button {
                            class: "w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-red-600 text-base font-medium text-white hover:bg-red-700 focus:outline-none sm:ml-3 sm:w-auto sm:text-sm",
                            onclick: move |_| on_confirm.call(()),
                            "Confirm"
                        }
                        button {
                            class: "mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm",
                            onclick: move |_| on_cancel.call(()),
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}
