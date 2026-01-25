use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;
use crate::state::SidebarState;

#[component]
pub fn Sidebar() -> Element {
    let mut sidebar_state = use_context::<Signal<SidebarState>>();
    let is_open = sidebar_state.read().is_open;

    let sidebar_class = if is_open {
        "translate-x-0"
    } else {
        "-translate-x-full md:translate-x-0"
    };

    let close_sidebar = move |_| {
        sidebar_state.write().close();
    };

    rsx! {
        // Overlay for mobile
        if is_open {
            div {
                class: "fixed inset-0 bg-black bg-opacity-50 z-30 md:hidden",
                onclick: close_sidebar
            }
        }

        aside { class: "w-64 bg-gray-800 text-white h-screen fixed left-0 top-0 flex flex-col transition-transform duration-300 ease-in-out z-40 {sidebar_class}",
            div { class: "p-4 border-b border-gray-700 flex items-center justify-between flex-shrink-0",
                h1 { class: "text-xl font-bold text-blue-400", "HYPER LOCAL" }
                button {
                    class: "text-gray-400 hover:text-white md:hidden",
                    onclick: close_sidebar,
                    "✕"
                }
            }
            nav { class: "mt-4 pb-10 flex-1 overflow-y-auto",
                SidebarLink { to: Route::Dashboard {}, icon: "🏠", label: "Dashboard" }
                SidebarLink { to: Route::Orders {}, icon: "📦", label: "Orders" }
                SidebarLink { to: Route::Categories {}, icon: "📁", label: "Categories" }
                SidebarLink { to: Route::Brands {}, icon: "🏷️", label: "Brands" }
                
                SidebarDropdown { 
                    icon: "👥", 
                    label: "Seller Management",
                    children: rsx! {
                        SidebarSubLink { to: Route::Sellers {}, label: "Sellers" }
                        // SidebarSubLink { to: Route::SellerSettlement {}, label: "Settlement Overview" } // TODO
                        SidebarSubLink { to: Route::SellerWithdrawals {}, label: "Seller Withdrawals" }
                        // SidebarSubLink { to: Route::WithdrawalHistory {}, label: "Withdrawal History" } // TODO
                    }
                }

                SidebarLink { to: Route::Stores {}, icon: "🏪", label: "Stores" }

                SidebarDropdown { 
                    icon: "🛒", 
                    label: "Products",
                    children: rsx! {
                        SidebarSubLink { to: Route::Products {}, label: "Products" }
                        SidebarSubLink { to: Route::PendingProducts {}, label: "Pending Approval" }
                        SidebarSubLink { to: Route::Reviews {}, label: "Product Reviews" }
                    }
                }

                SidebarLink { to: Route::TaxRates {}, icon: "💰", label: "Tax Rates" }

                SidebarDropdown { 
                    icon: "🚴", 
                    label: "Manage Delivery Boys",
                    children: rsx! {
                        SidebarSubLink { to: Route::DeliveryBoys {}, label: "Delivery Boys" }
                        // SidebarSubLink { to: Route::DeliveryBoyEarnings {}, label: "Delivery Boy Earnings" } // TODO
                        // SidebarSubLink { to: Route::EarningHistory {}, label: "Earning History" } // TODO
                        // SidebarSubLink { to: Route::DeliveryBoyCashCollections {}, label: "Cash Collections" } // TODO
                        // SidebarSubLink { to: Route::CashCollectionHistory {}, label: "Collection History" } // TODO
                        // SidebarSubLink { to: Route::DeliveryBoyWithdrawals {}, label: "Withdrawals" } // TODO
                        // SidebarSubLink { to: Route::DeliveryBoyWithdrawalHistory {}, label: "Withdrawal History" } // TODO
                    }
                }

                SidebarLink { to: Route::Reviews {}, icon: "⭐", label: "Reviews" }
                SidebarLink { to: Route::Refunds {}, icon: "💸", label: "Refunds" }
                SidebarLink { to: Route::Banners {}, icon: "🖼️", label: "Banners" }
                SidebarLink { to: Route::FeaturedSections {}, icon: "⭐", label: "Manage Featured Section" }
                SidebarLink { to: Route::Promos {}, icon: "🎟️", label: "Promos" }
                SidebarLink { to: Route::Faqs {}, icon: "❓", label: "FAQs" }
                SidebarLink { to: Route::SupportTickets {}, icon: "🎫", label: "Support Tickets" }
                SidebarLink { to: Route::DeliveryZones {}, icon: "📍", label: "Delivery Zones" }
                SidebarLink { to: Route::Notifications {}, icon: "🔔", label: "Notifications" }

                SidebarDropdown { 
                    icon: "🛡️", 
                    label: "Roles & Permissions",
                    children: rsx! {
                        SidebarSubLink { to: Route::Roles {}, label: "Roles" }
                        SidebarSubLink { to: Route::SystemUsers {}, label: "System Users" }
                    }
                }

                SidebarLink { to: Route::Settings {}, icon: "⚙️", label: "Settings" }
                SidebarLink { to: Route::SystemUpdates {}, icon: "🔄", label: "System Updates" }
                
                // Logout link (using Link for now, effectively same styling)
                div { 
                    class: "flex items-center px-4 py-2 text-gray-300 hover:bg-gray-700 hover:text-white transition-colors cursor-pointer mt-4 border-t border-gray-700 pt-4",
                    onclick: move |_| {
                        // TODO: Implement actual logout logic
                    },
                    span { class: "mr-3", "🚪" }
                    span { "Logout" }
                }
            }
        }
    }
}

#[component]
fn SidebarLink(to: Route, icon: &'static str, label: &'static str) -> Element {
    rsx! {
        Link {
            to: to,
            class: "flex items-center px-4 py-3 text-gray-300 hover:bg-gray-700 hover:text-white transition-colors",
            active_class: "bg-gray-900 text-white border-l-4 border-blue-500",
            span { class: "mr-3 text-lg", "{icon}" }
            span { class: "text-sm font-medium", "{label}" }
        }
    }
}

#[component]
fn SidebarSubLink(to: Route, label: &'static str) -> Element {
    rsx! {
        Link {
            to: to,
            class: "flex items-center pl-12 pr-4 py-2 text-gray-400 hover:text-white transition-colors text-sm",
            active_class: "text-white font-medium",
            span { "• {label}" }
        }
    }
}

#[component]
fn SidebarDropdown(icon: &'static str, label: &'static str, children: Element) -> Element {
    let mut is_open = use_signal(|| false);
    let arrow_class = if is_open() { "rotate-180" } else { "" };

    rsx! {
        div {
            div {
                class: "flex items-center px-4 py-3 text-gray-300 hover:bg-gray-700 hover:text-white transition-colors cursor-pointer justify-between",
                onclick: move |_| is_open.set(!is_open()),
                div { class: "flex items-center",
                    span { class: "mr-3 text-lg", "{icon}" }
                    span { class: "text-sm font-medium", "{label}" }
                }
                span { 
                    class: "transform transition-transform duration-200 {arrow_class}",
                    "▼" 
                }
            }
            if is_open() {
                div { class: "bg-gray-800 pb-2",
                    {children}
                }
            }
        }
    }
}
