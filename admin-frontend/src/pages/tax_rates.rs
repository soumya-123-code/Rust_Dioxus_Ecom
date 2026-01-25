use dioxus::prelude::*;
use crate::components::{Sidebar, Header, DataTable, Column, Pagination, Modal, ConfirmDialog};
use crate::api;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TaxRate {
    pub id: u64,
    pub name: String,
    pub rate: Decimal,
    pub country: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub priority: i32,
    pub compound: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TaxRateResponse {
    pub id: u64,
    pub name: String,
    pub rate: Decimal,
    pub country: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub priority: i32,
    pub compound: bool,
}

#[component]
pub fn TaxRates() -> Element {
    let mut tax_rates = use_signal(|| Vec::<TaxRate>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);
    
    // Modal states
    let mut show_create_modal = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| false);
    let mut show_delete_dialog = use_signal(|| false);
    let mut selected_tax_rate = use_signal(|| Option::<TaxRate>::None);
    let mut saving = use_signal(|| false);

    // Form signals
    let mut name = use_signal(|| String::new());
    let mut rate = use_signal(|| String::new());
    let mut country = use_signal(|| String::new());
    let mut state = use_signal(|| String::new());
    let mut zip = use_signal(|| String::new());
    let mut priority = use_signal(|| 0);
    let mut compound = use_signal(|| false);

    let fetch_data = move || {
        spawn(async move {
            loading.set(true);
            match api::get::<Vec<TaxRate>>("/tax-rates").await {
                Ok(data) => tax_rates.set(data),
                Err(e) => error.set(Some(e.to_string())),
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let mut reset_form = move || {
        name.set(String::new());
        rate.set(String::new());
        country.set(String::new());
        state.set(String::new());
        zip.set(String::new());
        priority.set(0);
        compound.set(false);
    };

    let open_create_modal = move |_| {
        reset_form();
        show_create_modal.set(true);
    };

    let mut open_edit_modal = move |tr: TaxRate| {
        selected_tax_rate.set(Some(tr.clone()));
        name.set(tr.name);
        rate.set(tr.rate.to_string());
        country.set(tr.country.unwrap_or_default());
        state.set(tr.state.unwrap_or_default());
        zip.set(tr.zip.unwrap_or_default());
        priority.set(tr.priority);
        compound.set(tr.compound);
        show_edit_modal.set(true);
    };

    let mut open_delete_dialog = move |tr: TaxRate| {
        selected_tax_rate.set(Some(tr));
        show_delete_dialog.set(true);
    };

    let handle_create = move |_| {
        spawn(async move {
            saving.set(true);
            
            let rate_val = rate().parse::<f64>().unwrap_or(0.0);
            
            let req = serde_json::json!({
                "name": name(),
                "rate": rate_val,
                "country": if country().is_empty() { None } else { Some(country()) },
                "state": if state().is_empty() { None } else { Some(state()) },
                "zip": if zip().is_empty() { None } else { Some(zip()) },
                "priority": priority(),
                "compound": compound()
            });

            match api::post::<serde_json::Value, _>("/tax-rates", &req).await {
                Ok(_) => {
                    show_create_modal.set(false);
                    fetch_data();
                }
                Err(e) => println!("Error creating tax rate: {}", e),
            }
            saving.set(false);
        });
    };

    let handle_update = move |_| {
        spawn(async move {
            if let Some(tr) = selected_tax_rate() {
                saving.set(true);
                
                let rate_val = rate().parse::<f64>().unwrap_or(0.0);

                let req = serde_json::json!({
                    "name": name(),
                    "rate": rate_val,
                    "country": if country().is_empty() { None } else { Some(country()) },
                    "state": if state().is_empty() { None } else { Some(state()) },
                    "zip": if zip().is_empty() { None } else { Some(zip()) },
                    "priority": priority(),
                    "compound": compound()
                });

                match api::put::<serde_json::Value, _>(&format!("/tax-rates/{}", tr.id), &req).await {
                    Ok(_) => {
                        show_edit_modal.set(false);
                        fetch_data();
                    }
                    Err(e) => println!("Error updating tax rate: {}", e),
                }
                saving.set(false);
            }
        });
    };

    let handle_delete = move |_| {
        spawn(async move {
            if let Some(tr) = selected_tax_rate() {
                match api::delete(&format!("/tax-rates/{}", tr.id)).await {
                    Ok(_) => {
                        show_delete_dialog.set(false);
                        fetch_data();
                    }
                    Err(e) => println!("Error deleting tax rate: {}", e),
                }
            }
        });
    };

    let columns = vec![
        Column::new("ID", "id"),
        Column::new("Name", "name"),
        Column::new("Rate (%)", "rate"),
        Column::new("Location", "location"),
        Column::new("Priority", "priority"),
    ];

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            main { class: "md:ml-64 pt-16 p-6",
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-2xl font-bold text-gray-800", "Tax Rates" }
                    button {
                        class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg flex items-center",
                        onclick: open_create_modal,
                        "Add Tax Rate"
                    }
                }

                DataTable {
                    columns: columns,
                    loading: *loading.read(),
                    for tr in tax_rates.read().iter() {
                        {
                            let tr1 = tr.clone();
                            let tr2 = tr.clone();
                            rsx! {
                                tr { key: "{tr.id}", class: "hover:bg-gray-50",
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{tr.id}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{tr.name}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{tr.rate}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        div { class: "text-sm",
                                            if let Some(c) = &tr.country { "{c}" } else { "All Countries" }
                                            if let Some(s) = &tr.state { ", {s}" }
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{tr.priority}" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                        button {
                                            class: "text-blue-600 hover:text-blue-900 mr-3",
                                            onclick: move |_| open_edit_modal(tr1.clone()),
                                            "Edit"
                                        }
                                        button {
                                            class: "text-red-600 hover:text-red-900",
                                            onclick: move |_| open_delete_dialog(tr2.clone()),
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
            title: "Create Tax Rate".to_string(),
            is_open: *show_create_modal.read(),
            on_close: move |_| show_create_modal.set(false),
            div { class: "space-y-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Name" }
                    input {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        value: "{name}",
                        oninput: move |e| name.set(e.value())
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Rate (%)" }
                    input {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        r#type: "number",
                        step: "0.01",
                        value: "{rate}",
                        oninput: move |e| rate.set(e.value())
                    }
                }
                div { class: "grid grid-cols-3 gap-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Country" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{country}",
                            oninput: move |e| country.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "State" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{state}",
                            oninput: move |e| state.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "ZIP Code" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{zip}",
                            oninput: move |e| zip.set(e.value())
                        }
                    }
                }
                div { class: "flex items-center space-x-4",
                    div { class: "flex-1",
                        label { class: "block text-sm font-medium text-gray-700", "Priority" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            r#type: "number",
                            value: "{priority}",
                            oninput: move |e| priority.set(e.value().parse().unwrap_or(0))
                        }
                    }
                    div { class: "flex items-center mt-6",
                        input {
                            class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded",
                            r#type: "checkbox",
                            checked: "{compound}",
                            onchange: move |e| compound.set(e.value() == "true")
                        }
                        label { class: "ml-2 block text-sm text-gray-900", "Compound Rate" }
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
            title: "Edit Tax Rate".to_string(),
            is_open: *show_edit_modal.read(),
            on_close: move |_| show_edit_modal.set(false),
            div { class: "space-y-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Name" }
                    input {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        value: "{name}",
                        oninput: move |e| name.set(e.value())
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700", "Rate (%)" }
                    input {
                        class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                        r#type: "number",
                        step: "0.01",
                        value: "{rate}",
                        oninput: move |e| rate.set(e.value())
                    }
                }
                div { class: "grid grid-cols-3 gap-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Country" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{country}",
                            oninput: move |e| country.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "State" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{state}",
                            oninput: move |e| state.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "ZIP Code" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            value: "{zip}",
                            oninput: move |e| zip.set(e.value())
                        }
                    }
                }
                div { class: "flex items-center space-x-4",
                    div { class: "flex-1",
                        label { class: "block text-sm font-medium text-gray-700", "Priority" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2",
                            r#type: "number",
                            value: "{priority}",
                            oninput: move |e| priority.set(e.value().parse().unwrap_or(0))
                        }
                    }
                    div { class: "flex items-center mt-6",
                        input {
                            class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded",
                            r#type: "checkbox",
                            checked: "{compound}",
                            onchange: move |e| compound.set(e.value() == "true")
                        }
                        label { class: "ml-2 block text-sm text-gray-900", "Compound Rate" }
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
            title: "Delete Tax Rate".to_string(),
            message: "Are you sure you want to delete this tax rate? This action cannot be undone.".to_string(),
            is_open: *show_delete_dialog.read(),
            on_confirm: handle_delete,
            on_cancel: move |_| show_delete_dialog.set(false),
        }
    }
}
