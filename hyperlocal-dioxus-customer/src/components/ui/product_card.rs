use dioxus::prelude::*;
use crate::app::Route;
use crate::types::Product;
use crate::context::use_theme;

#[derive(Props, PartialEq, Clone)]
pub struct ProductCardProps {
    pub product: crate::types::Product,
    #[props(default)]
    pub layout: ProductCardLayout,
}

#[derive(PartialEq, Clone)]
pub enum ProductCardLayout {
    Grid,    // Square card for grid view (Laza style)
    List,    // Horizontal card for list view
    Compact, // Smaller card
}

impl Default for ProductCardLayout {
    fn default() -> Self {
        Self::Grid
    }
}

#[component]
pub fn ProductCard(props: ProductCardProps) -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;
    let navigator = use_navigator();
    let product = &props.product;
    let slug = product.slug.clone();
    
    let mut is_favorited = use_signal(|| false);
    
    let handle_favorite = move |evt: Event<MouseData>| {
        evt.stop_propagation();
        is_favorited.set(!is_favorited());
    };
    
    let handle_click = move |_| {
        if let Some(slug) = &slug {
            let _ = navigator.push(Route::ProductDetail { slug: slug.clone() });
        }
    };
    
    match props.layout {
        ProductCardLayout::Grid => rsx! {
            GridProductCard {
                product: product.clone(),
                colors: colors,
                is_favorited: is_favorited(),
                on_favorite: handle_favorite,
                on_click: handle_click,
            }
        },
        ProductCardLayout::List => rsx! {
            ListProductCard {
                product: product.clone(),
                colors: colors,
                is_favorited: is_favorited(),
                on_favorite: handle_favorite,
                on_click: handle_click,
            }
        },
        ProductCardLayout::Compact => rsx! {
            CompactProductCard {
                product: product.clone(),
                colors: colors,
                on_click: handle_click,
            }
        },
    }
}

#[component]
fn GridProductCard(
    product: Product,
    colors: &'static crate::config::theme::LazaColors,
    is_favorited: bool,
    on_favorite: EventHandler<Event<MouseData>>,
    on_click: EventHandler<Event<MouseData>>,
) -> Element {
    rsx! {
        div {
            class: "bg-white rounded-2xl overflow-hidden cursor-pointer transition-all duration-300 hover:shadow-2xl hover:scale-[1.02] group",
            style: "border: 1px solid {colors.border};",
            onclick: on_click,
            
            // Image Container
            div {
                class: "relative aspect-square w-full overflow-hidden bg-gray-50",
                
                // Product Image
                if let Some(image) = &product.main_image {
                    img {
                        class: "w-full h-full object-cover transition-transform duration-500 group-hover:scale-110",
                        src: image.clone(),
                        alt: product.name.as_deref().unwrap_or("Product"),
                        loading: "lazy",
                    }
                }
                
                // Badges Overlay (Top Left)
                div {
                    class: "absolute top-3 left-3 flex flex-col gap-2",
                    
                    // Discount Badge
                    if let (Some(price), Some(discount_price)) = (product.price, product.discount_price) {
                        if discount_price < price {
                            div {
                                class: "px-3 py-1 rounded-full text-xs font-bold text-white",
                                style: "background: linear-gradient(135deg, {colors.secondary} 0%, {colors.secondary_dark} 100%);",
                                { format!("-{}%", ((price - discount_price) / price * 100.0) as u32) }
                            }
                        }
                    }
                    
                    // New Badge
                    if product.is_new.unwrap_or(false) {
                        div {
                            class: "px-3 py-1 rounded-full text-xs font-bold text-white",
                            style: "background: linear-gradient(135deg, {colors.accent} 0%, {colors.accent_light} 100%);",
                            "NEW"
                        }
                    }
                }
                
                // Favorite Button (Top Right)
                button {
                    class: "absolute top-3 right-3 w-10 h-10 rounded-full bg-white shadow-md flex items-center justify-center transition-all duration-300 hover:scale-110 active:scale-95",
                    onclick: on_favorite,
                    
                    // Heart Icon
                    svg {
                        class: "w-5 h-5 transition-colors",
                        style: if is_favorited { 
                            format!("fill: {}; stroke: {};", colors.error, colors.error)
                        } else {
                            format!("fill: none; stroke: {};", colors.text_secondary)
                        },
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path {
                            d: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"
                        }
                    }
                }
            }
            
            // Product Info
            div {
                class: "p-4",
                
                // Product Name
                if let Some(name) = &product.name {
                    h3 {
                        class: "text-base font-bold mb-2 line-clamp-2 min-h-[48px]",
                        style: "color: {colors.text_primary};",
                        { name.clone() }
                    }
                }
                
                // Rating
                if let Some(rating) = product.rating {
                    div {
                        class: "flex items-center gap-2 mb-3",
                        div {
                            class: "flex items-center",
                            // Star Icons
                            for i in 0..5 {
                                svg {
                                    key: "{i}",
                                    class: "w-4 h-4",
                                    view_box: "0 0 20 20",
                                    fill: if (i as f64) < rating { colors.warning } else { colors.divider },
                                    path {
                                        d: "M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"
                                    }
                                }
                            }
                        }
                        span {
                            class: "text-sm font-medium",
                            style: "color: {colors.text_secondary};",
                            { format!("{:.1}", rating) }
                        }
                    }
                }
                
                // Price Section
                div {
                    class: "flex items-center justify-between",
                    div {
                        class: "flex items-center gap-2",
                        
                        // Current Price
                        if let Some(price) = product.discount_price.or(product.price) {
                            span {
                                class: "text-xl font-bold",
                                style: "color: {colors.primary};",
                                { format!("₹{:.0}", price) }
                            }
                        }
                        
                        // Original Price (if discounted)
                        if let (Some(original), Some(discount)) = (product.price, product.discount_price) {
                            if discount < original {
                                span {
                                    class: "text-sm line-through",
                                    style: "color: {colors.text_tertiary};",
                                    { format!("₹{:.0}", original) }
                                }
                            }
                        }
                    }
                    
                    // Stock Status
                    if let Some(stock) = product.in_stock {
                        if !stock {
                            span {
                                class: "text-xs font-semibold px-2 py-1 rounded-full",
                                style: "background-color: {colors.error}; color: white;",
                                "Out of Stock"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ListProductCard(
    product: Product,
    colors: &'static crate::config::theme::LazaColors,
    is_favorited: bool,
    on_favorite: EventHandler<Event<MouseData>>,
    on_click: EventHandler<Event<MouseData>>,
) -> Element {
    rsx! {
        div {
            class: "bg-white rounded-2xl overflow-hidden cursor-pointer transition-all duration-300 hover:shadow-lg",
            style: "border: 1px solid {colors.border};",
            onclick: on_click,
            
            div {
                class: "flex gap-4 p-4",
                
                // Image (Left Side)
                div {
                    class: "relative w-24 h-24 flex-shrink-0 rounded-xl overflow-hidden bg-gray-50",
                    if let Some(image) = &product.main_image {
                        img {
                            class: "w-full h-full object-cover",
                            src: image.clone(),
                            alt: product.name.as_deref().unwrap_or("Product"),
                            loading: "lazy",
                        }
                    }
                }
                
                // Product Info (Middle)
                div {
                    class: "flex-1",
                    if let Some(name) = &product.name {
                        h3 {
                            class: "text-base font-bold mb-2 line-clamp-2",
                            style: "color: {colors.text_primary};",
                            { name.clone() }
                        }
                    }
                    
                    // Price
                    div {
                        class: "flex items-center gap-2 mb-2",
                        if let Some(price) = product.discount_price.or(product.price) {
                            span {
                                class: "text-lg font-bold",
                                style: "color: {colors.primary};",
                                { format!("₹{:.0}", price) }
                            }
                        }
                        if let (Some(original), Some(discount)) = (product.price, product.discount_price) {
                            if discount < original {
                                span {
                                    class: "text-sm line-through",
                                    style: "color: {colors.text_tertiary};",
                                    { format!("₹{:.0}", original) }
                                }
                            }
                        }
                    }
                    
                    // Rating
                    if let Some(rating) = product.rating {
                        div {
                            class: "flex items-center gap-1",
                            span {
                                class: "text-sm",
                                style: "color: {colors.warning};",
                                "★"
                            }
                            span {
                                class: "text-sm font-medium",
                                style: "color: {colors.text_secondary};",
                                { format!("{:.1}", rating) }
                            }
                        }
                    }
                }
                
                // Favorite Button (Right Side)
                button {
                    class: "flex-shrink-0 w-10 h-10 rounded-full bg-gray-50 flex items-center justify-center transition-all duration-300 hover:scale-110",
                    onclick: on_favorite,
                    svg {
                        class: "w-5 h-5",
                        style: if is_favorited { 
                            format!("fill: {}; stroke: {};", colors.error, colors.error)
                        } else {
                            format!("fill: none; stroke: {};", colors.text_secondary)
                        },
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        path {
                            d: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CompactProductCard(
    product: Product,
    colors: &'static crate::config::theme::LazaColors,
    on_click: EventHandler<Event<MouseData>>,
) -> Element {
    rsx! {
        div {
            class: "cursor-pointer transition-transform duration-300 hover:scale-105",
            onclick: on_click,
            
            // Image
            div {
                class: "relative aspect-square w-full rounded-xl overflow-hidden bg-gray-50 mb-2",
                if let Some(image) = &product.main_image {
                    img {
                        class: "w-full h-full object-cover",
                        src: image.clone(),
                        alt: product.name.as_deref().unwrap_or("Product"),
                        loading: "lazy",
                    }
                }
            }
            
            // Name
            if let Some(name) = &product.name {
                h4 {
                    class: "text-sm font-semibold mb-1 line-clamp-1",
                    style: "color: {colors.text_primary};",
                    { name.clone() }
                }
            }
            
            // Price
            if let Some(price) = product.discount_price.or(product.price) {
                span {
                    class: "text-base font-bold",
                    style: "color: {colors.primary};",
                    { format!("₹{:.0}", price) }
                }
            }
        }
    }
}
