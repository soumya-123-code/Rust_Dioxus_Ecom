use dioxus::prelude::*;
use dioxus_free_icons::icons::{hi_outline_icons, hi_solid_icons};
use dioxus_free_icons::Icon;
use crate::app::Route;
use crate::context::use_theme;

#[component]
pub fn BottomNav() -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;
    let navigator = use_navigator();
    let current_route: Route = use_route();

    // Helper to check if route is active
    let is_active = |route_name: &str| -> bool {
        match current_route {
            Route::Home {} => route_name == "Home",
            Route::Categories {} => route_name == "Categories",
            Route::Cart {} => route_name == "Cart",
            Route::Wishlist {} => route_name == "Wishlist",
            Route::Account {} => route_name == "Account",
            _ => false,
        }
    };

    rsx! {
        nav {
            class: "fixed bottom-0 left-0 right-0 z-50 transition-all duration-300",
            style: "
                background-color: {colors.surface}; 
                border-top: 1px solid {colors.border};
                padding-bottom: env(safe-area-inset-bottom);
                box-shadow: 0 -4px 6px -1px rgba(0, 0, 0, 0.1);
            ",
            
            div {
                class: "max-w-screen-xl mx-auto px-2",
                
                div {
                    class: "flex justify-around items-center h-20",
                    
                    // Home
                    NavItem {
                        name: "Home",
                        active: is_active("Home"),
                        colors: colors,
                        icon_outline: hi_outline_icons::HiHome,
                        icon_solid: hi_solid_icons::HiHome,
                        onclick: move |_| {
                            let _ = navigator.push(Route::Home {});
                        },
                    }
                    
                    // Categories
                    NavItem {
                        name: "Categories",
                        active: is_active("Categories"),
                        colors: colors,
                        icon_outline: hi_outline_icons::HiViewGrid,
                        icon_solid: hi_solid_icons::HiViewGrid,
                        onclick: move |_| {
                            let _ = navigator.push(Route::Categories {});
                        },
                    }
                    
                    // Cart (Center with special styling)
                    button {
                        class: "relative flex flex-col items-center justify-center -mt-6 transition-transform duration-300 active:scale-95",
                        onclick: move |_| {
                            let _ = navigator.push(Route::Cart {});
                        },
                        
                        // Floating Cart Button
                        div {
                            class: "w-16 h-16 rounded-full flex items-center justify-center shadow-2xl transition-all duration-300 hover:scale-110",
                            style: "background: linear-gradient(135deg, {} 0%, {} 100%);",
                            
                            Icon {
                                icon: if is_active("Cart") { hi_solid_icons::HiShoppingCart } else { hi_outline_icons::HiShoppingCart },
                                width: 28,
                                height: 28,
                                fill: "white",
                            }
                            
                            // Cart Badge (number of items)
                            div {
                                class: "absolute -top-1 -right-1 w-5 h-5 rounded-full flex items-center justify-center text-xs font-bold",
                                style: "background-color: {colors.error}; color: white;",
                                "3"
                            }
                        }
                    }
                    
                    // Wishlist
                    NavItem {
                        name: "Wishlist",
                        active: is_active("Wishlist"),
                        colors: colors,
                        icon_outline: hi_outline_icons::HiHeart,
                        icon_solid: hi_solid_icons::HiHeart,
                        onclick: move |_| {
                            let _ = navigator.push(Route::Wishlist {});
                        },
                    }
                    
                    // Account
                    NavItem {
                        name: "Account",
                        active: is_active("Account"),
                        colors: colors,
                        icon_outline: hi_outline_icons::HiUserCircle,
                        icon_solid: hi_solid_icons::HiUserCircle,
                        onclick: move |_| {
                            let _ = navigator.push(Route::Account {});
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn NavItem(
    name: String,
    active: bool,
    colors: &'static crate::config::theme::LazaColors,
    icon_outline: dioxus_free_icons::Icon,
    icon_solid: dioxus_free_icons::Icon,
    onclick: EventHandler<Event<MouseData>>,
) -> Element {
    let icon_color = if active {
        colors.primary
    } else {
        colors.text_secondary
    };
    
    let text_color = if active {
        colors.primary
    } else {
        colors.text_secondary
    };
    
    let font_weight = if active { "font-bold" } else { "font-medium" };
    let font_size = if active { "text-xs" } else { "text-xs" };
    
    rsx! {
        button {
            class: "flex flex-col items-center justify-center flex-1 transition-all duration-300 active:scale-95",
            onclick: onclick,
            
            div {
                class: "relative transition-transform duration-300",
                
                Icon {
                    icon: if active { icon_solid } else { icon_outline },
                    width: 24,
                    height: 24,
                    fill: icon_color,
                }
                
                // Active Indicator Dot
                if active {
                    div {
                        class: "absolute -bottom-1 left-1/2 transform -translate-x-1/2 w-1 h-1 rounded-full",
                        style: "background-color: {colors.primary};",
                    }
                }
            }
            
            span {
                class: "mt-1 {font_size} {font_weight} transition-colors duration-300",
                style: "color: {text_color};",
                {name}
            }
        }
    }
}

/// Alternative Bottom Navigation with Curved Center Design (Laza Style)
#[component]
pub fn BottomNavCurved() -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;
    let navigator = use_navigator();
    let current_route: Route = use_route();

    let is_active = |route_name: &str| -> bool {
        match current_route {
            Route::Home {} => route_name == "Home",
            Route::Categories {} => route_name == "Categories",
            Route::Cart {} => route_name == "Cart",
            Route::Wishlist {} => route_name == "Wishlist",
            Route::Account {} => route_name == "Account",
            _ => false,
        }
    };

    rsx! {
        nav {
            class: "fixed bottom-0 left-0 right-0 z-50",
            style: "padding-bottom: env(safe-area-inset-bottom);",
            
            // SVG Curved Background
            svg {
                class: "absolute bottom-0 w-full",
                height: "80",
                view_box: "0 0 375 80",
                fill: "none",
                preserve_aspect_ratio: "none",
                
                path {
                    d: "M0,20 L0,80 L375,80 L375,20 C360,20 350,20 337.5,20 C325,20 320,0 300,0 L75,0 C55,0 50,20 37.5,20 C25,20 15,20 0,20 Z",
                    fill: colors.surface,
                    stroke: colors.border,
                    stroke_width: "1",
                }
            }
            
            div {
                class: "relative max-w-screen-xl mx-auto px-4",
                
                div {
                    class: "flex justify-around items-end h-20",
                    
                    // Home
                    SimpleNavItem {
                        name: "Home",
                        active: is_active("Home"),
                        colors: colors,
                        icon: if is_active("Home") { hi_solid_icons::HiHome } else { hi_outline_icons::HiHome },
                        onclick: move |_| {
                            let _ = navigator.push(Route::Home {});
                        },
                    }
                    
                    // Categories
                    SimpleNavItem {
                        name: "Shop",
                        active: is_active("Categories"),
                        colors: colors,
                        icon: if is_active("Categories") { hi_solid_icons::HiViewGrid } else { hi_outline_icons::HiViewGrid },
                        onclick: move |_| {
                            let _ = navigator.push(Route::Categories {});
                        },
                    }
                    
                    // Spacer for center button
                    div { class: "w-16" }
                    
                    // Wishlist
                    SimpleNavItem {
                        name: "Wishlist",
                        active: is_active("Wishlist"),
                        colors: colors,
                        icon: if is_active("Wishlist") { hi_solid_icons::HiHeart } else { hi_outline_icons::HiHeart },
                        onclick: move |_| {
                            let _ = navigator.push(Route::Wishlist {});
                        },
                    }
                    
                    // Account
                    SimpleNavItem {
                        name: "Account",
                        active: is_active("Account"),
                        colors: colors,
                        icon: if is_active("Account") { hi_solid_icons::HiUserCircle } else { hi_outline_icons::HiUserCircle },
                        onclick: move |_| {
                            let _ = navigator.push(Route::Account {});
                        },
                    }
                }
                
                // Floating Center Button (Cart)
                button {
                    class: "absolute left-1/2 transform -translate-x-1/2 -top-6 w-14 h-14 rounded-full flex items-center justify-center shadow-2xl transition-all duration-300 hover:scale-110 active:scale-95",
                    style: "background: linear-gradient(135deg, {} 0%, {} 100%);",
                    onclick: move |_| {
                        let _ = navigator.push(Route::Cart {});
                    },
                    
                    Icon {
                        icon: hi_outline_icons::HiShoppingCart,
                        width: 24,
                        height: 24,
                        fill: "white",
                    }
                    
                    // Badge
                    div {
                        class: "absolute -top-1 -right-1 w-5 h-5 rounded-full flex items-center justify-center text-xs font-bold",
                        style: "background-color: {colors.error}; color: white;",
                        "3"
                    }
                }
            }
        }
    }
}

#[component]
fn SimpleNavItem(
    name: String,
    active: bool,
    colors: &'static crate::config::theme::LazaColors,
    icon: dioxus_free_icons::Icon,
    onclick: EventHandler<Event<MouseData>>,
) -> Element {
    let color = if active { colors.primary } else { colors.text_secondary };
    
    rsx! {
        button {
            class: "flex flex-col items-center justify-center pb-2 transition-all duration-300",
            onclick: onclick,
            
            Icon {
                icon: icon,
                width: 24,
                height: 24,
                fill: color,
            }
            
            span {
                class: "mt-1 text-xs font-medium",
                style: "color: {color};",
                {name}
            }
        }
    }
}
