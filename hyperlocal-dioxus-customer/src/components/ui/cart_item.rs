use dioxus::prelude::*;
use crate::types::CartItem;

#[derive(Props, PartialEq, Clone)]
pub struct CartItemProps {
    pub item: CartItem,
    pub on_remove: Option<EventHandler<MouseEvent>>,
    pub on_update_quantity: Option<EventHandler<(i32, i32)>>, // (item_id, new_quantity)
}

#[component]
pub fn CartItemComponent(props: CartItemProps) -> Element {
    // copy primitive fields so closures don't borrow `props` for 'static
    let item_id = props.item.id;
    let item_qty = props.item.quantity;
    let item_total_price = props.item.total_price;
    let item = props.item.clone();

    rsx! {
        div {
            class: "flex items-center gap-4 p-4 border-b border-gray-200",
            if let Some(product) = &item.product {
                if let Some(image) = &product.main_image {
                    img {
                        class: "w-20 h-20 object-cover rounded",
                        src: image.clone(),
                        alt: product.name.as_deref().unwrap_or("Product")
                    }
                }
            }
            div {
                class: "flex-1",
                if let Some(product) = &item.product {
                    if let Some(name) = &product.name {
                        h4 {
                            class: "font-semibold text-gray-900",
                            { name.clone() }
                        }
                    }
                }
                div {
                    class: "flex items-center gap-2 mt-2",
                    button {
                        class: "px-2 py-1 border rounded",
                    onclick: move |_| {
                            if let (Some(id), Some(qty)) = (item_id, item_qty) {
                                if qty > 1 {
                                    if let Some(handler) = props.on_update_quantity {
                                        handler.call((id, qty - 1));
                                    }
                                }
                            }
                        },
                        "-"
                    }
                    span {
                        class: "px-3",
                        { format!("{}", item_qty.unwrap_or(0)) }
                    }
                    button {
                        class: "px-2 py-1 border rounded",
                        onclick: move |_| {
                            if let (Some(id), Some(qty)) = (item_id, item_qty) {
                                if let Some(handler) = props.on_update_quantity {
                                    handler.call((id, qty + 1));
                                }
                            }
                        },
                        "+"
                    }
                }
            }
            div {
                class: "text-right",
                if let Some(total) = item_total_price {
                    p {
                        class: "font-semibold text-gray-900",
                        { format!("₹{:.2}", total) }
                    }
                }
                button {
                    class: "mt-2 text-red-600 hover:text-red-800 text-sm",
                    onclick: move |evt| {
                        if let Some(handler) = props.on_remove {
                            handler.call(evt);
                        }
                    },
                    "Remove"
                }
            }
        }
    }
}
