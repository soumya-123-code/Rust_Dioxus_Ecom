use dioxus::prelude::*;
use crate::api::{self, DashboardStats};
use crate::components::{Sidebar, Header, StatsCard};
use rust_decimal::Decimal;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub revenue: Vec<Decimal>,
    pub orders: Vec<i64>,
}

#[component]
pub fn Dashboard() -> Element {
    let mut stats = use_signal(|| Option::<DashboardStats>::None);
    let mut chart_data = use_signal(|| Option::<ChartData>::None);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            let stats_req = api::get::<DashboardStats>("/dashboard/stats");
            let chart_req = api::get::<ChartData>("/dashboard/chart-data");

            let (stats_res, chart_res) = futures::join!(stats_req, chart_req);

            if let Ok(data) = stats_res {
                stats.set(Some(data));
            }
            if let Ok(data) = chart_res {
                chart_data.set(Some(data));
            }
            loading.set(false);
        });
    });

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            Sidebar {}
            Header {}
            
            main { class: "md:ml-64 pt-16 p-6",
                h1 { class: "text-2xl font-bold text-gray-800 mb-6", "Dashboard" }
                
                if *loading.read() {
                    div { class: "text-center py-8",
                        p { class: "text-gray-500", "Loading..." }
                    }
                } else {
                    if let Some(data) = stats.read().as_ref() {
                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8",
                            StatsCard {
                                title: "Total Orders".to_string(),
                                value: data.total_orders.to_string(),
                                icon: "🛒".to_string(),
                                color: "blue".to_string(),
                            }
                            StatsCard {
                                title: "Total Products".to_string(),
                                value: data.total_products.to_string(),
                                icon: "📦".to_string(),
                                color: "green".to_string(),
                            }
                            StatsCard {
                                title: "Total Sellers".to_string(),
                                value: data.total_sellers.to_string(),
                                icon: "🏪".to_string(),
                                color: "purple".to_string(),
                            }
                            StatsCard {
                                title: "Total Customers".to_string(),
                                value: data.total_customers.to_string(),
                                icon: "👥".to_string(),
                                color: "indigo".to_string(),
                            }
                        }
                        
                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8",
                            StatsCard {
                                title: "Total Revenue".to_string(),
                                value: format!("${}", data.total_revenue),
                                icon: "💰".to_string(),
                                color: "green".to_string(),
                            }
                            StatsCard {
                                title: "Pending Orders".to_string(),
                                value: data.pending_orders.to_string(),
                                icon: "⏳".to_string(),
                                color: "yellow".to_string(),
                            }
                            StatsCard {
                                title: "Delivered Orders".to_string(),
                                value: data.delivered_orders.to_string(),
                                icon: "✅".to_string(),
                                color: "green".to_string(),
                            }
                            StatsCard {
                                title: "Pending Approvals".to_string(),
                                value: data.pending_seller_approvals.to_string(),
                                icon: "🔔".to_string(),
                                color: "red".to_string(),
                            }
                        }
                    }

                    if let Some(chart) = chart_data.read().as_ref() {
                        div { class: "bg-white rounded-lg shadow p-6 mb-8",
                            h3 { class: "text-lg font-semibold text-gray-800 mb-4", "Revenue & Orders Overview" }
                            div { class: "relative h-64",
                                // Simple CSS Bar Chart
                                div { class: "absolute inset-0 flex items-end justify-between space-x-2 px-2",
                                    for (i, label) in chart.labels.iter().enumerate() {
                                        {
                                            // Calculate height percentage (normalized to max value)
                                            // Finding max revenue for scaling
                                            let max_revenue = chart.revenue.iter().fold(Decimal::ZERO, |a, &b| if b > a { b } else { a });
                                            let val = chart.revenue.get(i).unwrap_or(&Decimal::ZERO);
                                            // Avoid division by zero
                                            let height_pct = if max_revenue > Decimal::ZERO {
                                                (val / max_revenue * Decimal::from(100)).to_string()
                                            } else {
                                                "0".to_string()
                                            };
                                            
                                            rsx! {
                                                div { class: "flex flex-col items-center flex-1 group",
                                                    div { 
                                                        class: "w-full bg-blue-500 rounded-t hover:bg-blue-600 transition-all duration-300 relative",
                                                        style: "height: {height_pct}%;",
                                                        
                                                        // Tooltip
                                                        div { class: "absolute bottom-full mb-2 left-1/2 transform -translate-x-1/2 bg-gray-800 text-white text-xs rounded py-1 px-2 opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap z-10",
                                                            "Revenue: ${val} | Orders: {chart.orders.get(i).unwrap_or(&0)}"
                                                        }
                                                    }
                                                    p { class: "text-xs text-gray-500 mt-2 rotate-45 origin-top-left", "{label}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
