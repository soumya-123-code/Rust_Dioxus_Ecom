use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Column {
    pub key: String,
    pub label: String,
    pub sortable: bool,
}

impl Column {
    pub fn new(label: &str, key: &str) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            sortable: true,
        }
    }
}

#[component]
pub fn DataTable(
    columns: Vec<Column>,
    children: Element,
    loading: bool,
) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow overflow-hidden",
            div { class: "overflow-x-auto",
                table { class: "min-w-full divide-y divide-gray-200",
                    thead { class: "bg-gray-50",
                        tr {
                            for column in columns.iter() {
                                th {
                                    key: "{column.key}",
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "{column.label}"
                                }
                            }
                            th { class: "px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Actions"
                            }
                        }
                    }
                    tbody { class: "bg-white divide-y divide-gray-200",
                        if loading {
                            tr {
                                td {
                                    colspan: "{columns.len() + 1}",
                                    class: "px-6 py-4 text-center text-gray-500",
                                    "Loading..."
                                }
                            }
                        } else {
                            {children}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Pagination(
    current_page: i64,
    total_pages: i64,
    on_page_change: EventHandler<i64>,
) -> Element {
    rsx! {
        div { class: "flex items-center justify-between px-4 py-3 bg-white border-t border-gray-200 sm:px-6",
            div { class: "flex justify-between sm:hidden",
                button {
                    class: "relative inline-flex items-center px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50",
                    disabled: current_page <= 1,
                    onclick: move |_| on_page_change.call(current_page - 1),
                    "Previous"
                }
                button {
                    class: "relative inline-flex items-center px-4 py-2 ml-3 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50",
                    disabled: current_page >= total_pages,
                    onclick: move |_| on_page_change.call(current_page + 1),
                    "Next"
                }
            }
            div { class: "hidden sm:flex sm:flex-1 sm:items-center sm:justify-between",
                div {
                    p { class: "text-sm text-gray-700",
                        "Page "
                        span { class: "font-medium", "{current_page}" }
                        " of "
                        span { class: "font-medium", "{total_pages}" }
                    }
                }
                div {
                    nav { class: "relative z-0 inline-flex rounded-md shadow-sm -space-x-px",
                        button {
                            class: "relative inline-flex items-center px-2 py-2 rounded-l-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50",
                            disabled: current_page <= 1,
                            onclick: move |_| on_page_change.call(current_page - 1),
                            "←"
                        }
                        span { class: "relative inline-flex items-center px-4 py-2 border border-gray-300 bg-white text-sm font-medium text-gray-700",
                            "{current_page}"
                        }
                        button {
                            class: "relative inline-flex items-center px-2 py-2 rounded-r-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50",
                            disabled: current_page >= total_pages,
                            onclick: move |_| on_page_change.call(current_page + 1),
                            "→"
                        }
                    }
                }
            }
        }
    }
}
