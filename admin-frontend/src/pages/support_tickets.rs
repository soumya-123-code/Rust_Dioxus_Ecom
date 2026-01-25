use dioxus::prelude::*;
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, Modal};
use crate::api::{self, PaginatedResponse};

#[derive(PartialEq, Clone, Debug, serde::Deserialize)]
pub struct SupportTicket {
    pub id: u64,
    pub ticket_number: String,
    pub subject: String,
    pub priority: String,
    pub status: String,
    pub user_name: String,
    pub type_name: String,
    pub created_at: Option<String>,
}

#[component]
pub fn SupportTickets() -> Element {
    let mut page = use_signal(|| 1);
    let per_page = use_signal(|| 10);
    let mut tickets = use_signal(|| PaginatedResponse::<SupportTicket>::default());
    let mut error = use_signal(|| Option::<String>::None);
    let refresh_trigger = use_signal(|| 0);
    
    let mut show_view_modal = use_signal(|| false);
    let mut selected_ticket = use_signal(|| None::<SupportTicket>);

    use_effect(move || {
        spawn(async move {
            match api::get::<PaginatedResponse<SupportTicket>>(&format!("/support-tickets?page={}&per_page={}", page(), per_page())).await {
                Ok(res) => tickets.set(res),
                Err(e) => error.set(Some(e.to_string())),
            }
        });
    });

    use_effect(move || {
        if refresh_trigger() > 0 {
             spawn(async move {
                match api::get::<PaginatedResponse<SupportTicket>>(&format!("/support-tickets?page={}&per_page={}", page(), per_page())).await {
                    Ok(res) => tickets.set(res),
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        }
    });

    let mut view_ticket = move |ticket: SupportTicket| {
        selected_ticket.set(Some(ticket));
        show_view_modal.set(true);
    };

    let columns = vec![
        Column::new("Ticket #", "ticket_number"),
        Column::new("Subject", "subject"),
        Column::new("Customer", "user_name"),
        Column::new("Type", "type_name"),
        Column::new("Priority", "priority"),
        Column::new("Status", "status"),
        Column::new("Date", "created_at"),
        Column::new("Actions", "actions"),
    ];

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Support Tickets" }
                }

                if let Some(err) = error() {
                    div { class: "bg-red-500 text-white p-4 rounded mb-4", "{err}" }
                }

                div { class: "bg-white rounded-lg shadow overflow-hidden",
                    DataTable {
                        columns: columns,
                        loading: false,
                        {tickets().data.iter().map(|ticket| {
                            let ticket_view = ticket.clone();
                            rsx! {
                                tr { class: "border-b border-gray-200 hover:bg-gray-50",
                                    td { class: "px-6 py-4", "{ticket.ticket_number}" }
                                    td { class: "px-6 py-4", "{ticket.subject}" }
                                    td { class: "px-6 py-4", "{ticket.user_name}" }
                                    td { class: "px-6 py-4", "{ticket.type_name}" }
                                    td { class: "px-6 py-4", 
                                        span { 
                                            class: match ticket.priority.as_str() {
                                                "high" | "urgent" => "text-red-600 font-bold",
                                                "medium" => "text-yellow-600",
                                                _ => "text-green-600"
                                            },
                                            "{ticket.priority}"
                                        }
                                    }
                                    td { class: "px-6 py-4",
                                        span { 
                                            class: "px-2 py-1 bg-gray-100 rounded text-xs text-gray-800",
                                            "{ticket.status}"
                                        }
                                    }
                                    td { class: "px-6 py-4", "{ticket.created_at.clone().unwrap_or_default()}" }
                                    td { class: "px-6 py-4",
                                        button {
                                            class: "text-blue-600 hover:text-blue-800 mr-2",
                                            onclick: move |_| view_ticket(ticket_view.clone()),
                                            "View"
                                        }
                                    }
                                }
                            }
                        })}
                    }
                    Pagination {
                        current_page: page(),
                        total_pages: (tickets().total as f64 / per_page() as f64).ceil() as i64,
                        on_page_change: move |p| page.set(p),
                    }
                }
            }
            
            if let Some(ticket) = selected_ticket.read().clone() {
                Modal {
                    title: format!("Ticket #{}", ticket.ticket_number),
                    is_open: *show_view_modal.read(),
                    on_close: move |_| show_view_modal.set(false),
                    div { class: "space-y-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700", "Subject" }
                            p { class: "mt-1 text-sm text-gray-900", "{ticket.subject}" }
                        }
                        div { class: "grid grid-cols-2 gap-4",
                            div {
                                label { class: "block text-sm font-medium text-gray-700", "Customer" }
                                p { class: "mt-1 text-sm text-gray-900", "{ticket.user_name}" }
                            }
                            div {
                                label { class: "block text-sm font-medium text-gray-700", "Type" }
                                p { class: "mt-1 text-sm text-gray-900", "{ticket.type_name}" }
                            }
                        }
                        div { class: "grid grid-cols-2 gap-4",
                            div {
                                label { class: "block text-sm font-medium text-gray-700", "Priority" }
                                span { 
                                    class: match ticket.priority.as_str() {
                                        "high" | "urgent" => "mt-1 inline-block px-2 py-1 text-xs rounded-full bg-red-100 text-red-800",
                                        "medium" => "mt-1 inline-block px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800",
                                        _ => "mt-1 inline-block px-2 py-1 text-xs rounded-full bg-green-100 text-green-800"
                                    },
                                    "{ticket.priority}"
                                }
                            }
                            div {
                                label { class: "block text-sm font-medium text-gray-700", "Status" }
                                span { 
                                    class: "mt-1 inline-block px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-800",
                                    "{ticket.status}"
                                }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700", "Created At" }
                            p { class: "mt-1 text-sm text-gray-900", "{ticket.created_at.clone().unwrap_or_default()}" }
                        }
                        div { class: "flex justify-end pt-4",
                            button {
                                class: "bg-gray-200 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-300",
                                onclick: move |_| show_view_modal.set(false),
                                "Close"
                            }
                        }
                    }
                }
            }
        }
    }
}

