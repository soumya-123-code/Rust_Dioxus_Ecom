use dioxus::prelude::*;

use crate::app::Route;
use crate::types::Product;

#[derive(Props, PartialEq, Clone)]
pub struct ProductCardProps {
    pub product: crate::types::Product,
}

#[component]
pub fn ProductCard(props: ProductCardProps) -> Element {
    let navigator = use_navigator();
    let product = &props.product;
    let slug = product.slug.clone();

    rsx! {
        div {
            class: "bg-white rounded-lg shadow-md overflow-hidden cursor-pointer hover:shadow-lg transition-shadow",
            onclick: move |_| {
                if let Some(slug) = &slug {
                    let _ = navigator.push(Route::ProductDetail { slug: slug.clone() });
                }
            },
            if let Some(image) = &product.main_image {
                img {
                    class: "w-full h-48 object-cover",
                    src: image.clone(),
                    alt: product.name.as_deref().unwrap_or("Product")
                }
            }
            div {
                class: "p-4",
                if let Some(name) = &product.name {
                    h3 {
                        class: "text-lg font-semibold text-gray-900 mb-2",
                        { name.clone() }
                    }
                }
                div {
                    class: "flex items-center justify-between",
                    div {
                        class: "flex items-center gap-2",
                        if let Some(price) = product.price {
                            span {
                                class: "text-lg font-bold text-indigo-600",
                                { format!("₹{:.2}", price) }
                            }
                        }
                        if let Some(discount_price) = product.discount_price {
                            span {
                                class: "text-sm text-gray-500 line-through",
                                { format!("₹{:.2}", discount_price) }
                            }
                        }
                    }
                    if let Some(rating) = product.rating {
                        div {
                            class: "flex items-center gap-1",
                            "⭐"
                            span {
                                class: "text-sm text-gray-600",
                                { format!("{:.1}", rating) }
                            }
                        }
                    }
                }
            }
        }
    }
}
