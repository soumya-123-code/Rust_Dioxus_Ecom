use dioxus::prelude::*;
use crate::types::Cart;

#[derive(Clone, Debug)]
pub struct CartContext {
    pub cart: Option<Cart>,
    pub item_count: i32,
    pub total: f64,
}

impl Default for CartContext {
    fn default() -> Self {
        Self {
            cart: None,
            item_count: 0,
            total: 0.0,
        }
    }
}

pub fn use_cart() -> Signal<CartContext> {
    use_context::<Signal<CartContext>>()
}
