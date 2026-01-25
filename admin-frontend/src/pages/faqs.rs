use dioxus::prelude::*;
use crate::components::{Sidebar, Header, DataTable, Column, Modal, ConfirmDialog};
use crate::api;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Faq {
    pub id: u64,
    pub question: String,
    pub answer: String,
    pub sort_order: Option<i32>,
    pub status: String,
}

#[component]
pub fn Faqs() -> Element {
    let mut faqs = use_signal(|| Vec::<Faq>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);
    
    // Modal states
    let mut show_create_modal = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| false);
    let mut show_delete_dialog = use_signal(|| false);
    let mut selected_faq = use_signal(|| Option::<Faq>::None);
    let mut saving = use_signal(|| false);

    // Form signals
    let mut question = use_signal(|| String::new());
    let mut answer = use_signal(|| String::new());
    let mut sort_order = use_signal(|| 0);
    let mut status = use_signal(|| "active".to_string());

    let fetch_data = move || {
        spawn(async move {
            loading.set(true);
            match api::get::<Vec<Faq>>("/faqs").await {
                Ok(data) => faqs.set(data),
                Err(e) => error.set(Some(e.to_string())),
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let mut reset_form = move || {
        question.set(String::new());
        answer.set(String::new());
        sort_order.set(0);
        status.set("active".to_string());
    };

    let open_create_modal = move |_| {
        reset_form();
        show_create_modal.set(true);
    };

    let mut open_edit_modal = move |f: Faq| {
        selected_faq.set(Some(f.clone()));
        question.set(f.question);
        answer.set(f.answer);
        sort_order.set(f.sort_order.unwrap_or(0));
        status.set(f.status);
        show_edit_modal.set(true);
    };

    let mut open_delete_dialog = move |f: Faq| {
        selected_faq.set(Some(f));
        show_delete_dialog.set(true);
    };

    let handle_create = move |_| {
        spawn(async move {
            saving.set(true);
            let req = serde_json::json!({
                "question": question(),
                "answer": answer(),
                "sort_order": sort_order(),
                "status": status()
            });

            match api::post::<serde_json::Value, _>("/faqs", &req).await {
                Ok(_) => {
                    show_create_modal.set(false);
                    fetch_data();
                }
                Err(e) => println!("Error creating FAQ: {}", e),
            }
            saving.set(false);
        });
    };

    let handle_update = move |_| {
        spawn(async move {
            if let Some(f) = selected_faq() {
                saving.set(true);
                let req = serde_json::json!({
                    "question": question(),
                    "answer": answer(),
                    "sort_order": sort_order(),
                    "status": status()
                });

                match api::put::<serde_json::Value, _>(&format!("/faqs/{}", f.id), &req).await {
                    Ok(_) => {
                        show_edit_modal.set(false);
                        fetch_data();
                    }
                    Err(e) => println!("Error updating FAQ: {}", e),
                }
                saving.set(false);
            }
        });
    };

    let handle_delete = move |_| {
        spawn(async move {
            if let Some(f) = selected_faq() {
                match api::delete(&format!("/faqs/{}", f.id)).await {
                    Ok(_) => {
                        show_delete_dialog.set(false);
                        fetch_data();
                    }
                    Err(e) => println!("Error deleting FAQ: {}", e),
                }
            }
        });
    };

    let columns = vec![
        Column::new("ID", "id"),
        Column::new("Question", "question"),
        Column::new("Answer", "answer"),
        Column::new("Status", "status"),
        Column::new("Sort Order", "sort_order"),
    ];

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "FAQs" }
                    button {
                        class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg flex items-center",
                        onclick: open_create_modal,
                        "Add FAQ"
                    }
                }

                DataTable {
                    columns: columns,
                    loading: *loading.read(),
                    for f in faqs.read().iter() {
                        {
                            let f1 = f.clone();
                            let f2 = f.clone();
                            rsx! {
                                tr { key: "{f.id}", class: "hover:bg-gray-50",
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{f.id}" }
                                    td { class: "px-6 py-4 text-sm text-gray-900 max-w-xs truncate", "{f.question}" }
                                    td { class: "px-6 py-4 text-sm text-gray-500 max-w-xs truncate", "{f.answer}" }
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        span {
                                            class: if f.status == "active" { "px-2 py-1 text-xs rounded-full bg-green-100 text-green-800" } else { "px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800" },
                                            "{f.status}"
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{f.sort_order.unwrap_or(0)}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                        button {
                                            class: "text-blue-600 hover:text-blue-900 mr-3",
                                            onclick: move |_| open_edit_modal(f1.clone()),
                                            "Edit"
                                        }
                                        button {
                                            class: "text-red-600 hover:text-red-900",
                                            onclick: move |_| open_delete_dialog(f2.clone()),
                                            "Delete"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Create Modal
        Modal {
            title: "Create FAQ".to_string(),
            is_open: *show_create_modal.read(),
            on_close: move |_| show_create_modal.set(false),
            div { class: "space-y-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Question" }
                    textarea {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        rows: "3",
                        value: "{question}",
                        oninput: move |e| question.set(e.value())
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Answer" }
                    textarea {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        rows: "5",
                        value: "{answer}",
                        oninput: move |e| answer.set(e.value())
                    }
                }
                div { class: "grid grid-cols-2 gap-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Sort Order" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            r#type: "number",
                            value: "{sort_order}",
                            oninput: move |e| sort_order.set(e.value().parse().unwrap_or(0))
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Status" }
                        select {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{status}",
                            onchange: move |e| status.set(e.value()),
                            option { value: "active", "Active" }
                            option { value: "inactive", "Inactive" }
                        }
                    }
                }

                div { class: "flex justify-end pt-4",
                    button {
                        class: "mr-3 px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50",
                        onclick: move |_| show_create_modal.set(false),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 disabled:opacity-50",
                        onclick: handle_create,
                        disabled: *saving.read(),
                        if *saving.read() { "Saving..." } else { "Create" }
                    }
                }
            }
        }

        // Edit Modal
        Modal {
            title: "Edit FAQ".to_string(),
            is_open: *show_edit_modal.read(),
            on_close: move |_| show_edit_modal.set(false),
            div { class: "space-y-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Question" }
                    textarea {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        rows: "3",
                        value: "{question}",
                        oninput: move |e| question.set(e.value())
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Answer" }
                    textarea {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        rows: "5",
                        value: "{answer}",
                        oninput: move |e| answer.set(e.value())
                    }
                }
                div { class: "grid grid-cols-2 gap-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Sort Order" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            r#type: "number",
                            value: "{sort_order}",
                            oninput: move |e| sort_order.set(e.value().parse().unwrap_or(0))
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Status" }
                        select {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{status}",
                            onchange: move |e| status.set(e.value()),
                            option { value: "active", "Active" }
                            option { value: "inactive", "Inactive" }
                        }
                    }
                }

                div { class: "flex justify-end pt-4",
                    button {
                        class: "mr-3 px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50",
                        onclick: move |_| show_edit_modal.set(false),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 disabled:opacity-50",
                        onclick: handle_update,
                        disabled: *saving.read(),
                        if *saving.read() { "Saving..." } else { "Save Changes" }
                    }
                }
            }
        }

        // Delete Confirmation
        ConfirmDialog {
            title: "Delete FAQ".to_string(),
            message: "Are you sure you want to delete this FAQ? This action cannot be undone.".to_string(),
            is_open: *show_delete_dialog.read(),
            on_confirm: handle_delete,
            on_cancel: move |_| show_delete_dialog.set(false),
        }
    }
}
