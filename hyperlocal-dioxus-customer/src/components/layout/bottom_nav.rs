use dioxus::prelude::*;
use dioxus_free_icons::icons::{hi_outline_icons, hi_solid_icons};
use dioxus_free_icons::Icon;

use crate::app::Route;

#[component]
pub fn BottomNav() -> Element {
    let navigator = use_navigator();
    let current_route: Route = use_route();

    // Helper to check if route is active
    let is_active = |route_name: &str| -> bool {
        match current_route {
            Route::Home {} => route_name == "Home",
            Route::Categories {} => route_name == "Categories",
            Route::NearByStores {} => route_name == "Stores",
            Route::Account {} => route_name == "Account",
            _ => false,
        }
    };

    let get_color = |name: &str| {
        if is_active(name) { "#0D1117" } else { "rgba(13, 17, 23, 0.8)" }
    };

    let get_font_weight = |name: &str| {
        if is_active(name) { "bold" } else { "600" }
    };

    let get_font_size = |name: &str| {
        if is_active(name) { "13px" } else { "12px" }
    };

    rsx! {
        nav {
            style: "position: fixed; bottom: 0; left: 0; right: 0; background-color: white; border-top: 1px solid #e5e7eb; z-index: 50; padding-bottom: env(safe-area-inset-bottom); height: 70px;",
            div {
                style: "max-width: 80rem; margin: 0 auto; padding-left: 0.5rem; padding-right: 0.5rem;",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; height: 100%;",

                    // Home
                    button {
                        style: "display: flex; flex-direction: column; align-items: center; justify-content: center; width: 25%; background: none; border: none; cursor: pointer; color: {get_color(\"Home\")};",
                        onclick: move |_| { navigator.push(Route::Home {}); },
                        if is_active("Home") {
                            Icon {
                                width: 24,
                                height: 24,
                                icon: hi_solid_icons::HiHome
                            }
                        } else {
                            Icon {
                                width: 24,
                                height: 24,
                                icon: hi_outline_icons::HiHome
                            }
                        }
                        span {
                            style: "font-size: {get_font_size(\"Home\")}; margin-top: 4px; font-weight: {get_font_weight(\"Home\")};",
                            "Home"
                        }
                    }

                    // Categories
                    button {
                        style: "display: flex; flex-direction: column; align-items: center; justify-content: center; width: 25%; background: none; border: none; cursor: pointer; color: {get_color(\"Categories\")};",
                        onclick: move |_| { navigator.push(Route::Categories {}); },
                        if is_active("Categories") {
                            Icon {
                                width: 24,
                                height: 24,
                                icon: hi_solid_icons::HiViewGrid
                            }
                        } else {
                            Icon {
                                width: 24,
                                height: 24,
                                icon: hi_outline_icons::HiViewGrid
                            }
                        }
                        span {
                            style: "font-size: {get_font_size(\"Categories\")}; margin-top: 4px; font-weight: {get_font_weight(\"Categories\")};",
                            "Categories"
                        }
                    }

                    // Stores
                    button {
                        style: "display: flex; flex-direction: column; align-items: center; justify-content: center; width: 25%; background: none; border: none; cursor: pointer; color: {get_color(\"Stores\")};",
                        onclick: move |_| { navigator.push(Route::NearByStores {}); },
                        if is_active("Stores") {
                            Icon {
                                width: 24,
                                height: 24,
                                icon: hi_solid_icons::HiOfficeBuilding
                            }
                        } else {
                            Icon {
                                width: 24,
                                height: 24,
                                icon: hi_outline_icons::HiOfficeBuilding
                            }
                        }
                        span {
                            style: "font-size: {get_font_size(\"Stores\")}; margin-top: 4px; font-weight: {get_font_weight(\"Stores\")};",
                            "Stores"
                        }
                    }

                    // Account
                    button {
                        style: "display: flex; flex-direction: column; align-items: center; justify-content: center; width: 25%; background: none; border: none; cursor: pointer; color: {get_color(\"Account\")};",
                        onclick: move |_| { navigator.push(Route::Account {}); },
                        if is_active("Account") {
                            Icon {
                                width: 24,
                                height: 24,
                                icon: hi_solid_icons::HiUserCircle
                            }
                        } else {
                            Icon {
                                width: 24,
                                height: 24,
                                icon: hi_outline_icons::HiUserCircle
                            }
                        }
                        span {
                            style: "font-size: {get_font_size(\"Account\")}; margin-top: 4px; font-weight: {get_font_weight(\"Account\")};",
                            "Account"
                        }
                    }
                }
            }
        }
    }
}
