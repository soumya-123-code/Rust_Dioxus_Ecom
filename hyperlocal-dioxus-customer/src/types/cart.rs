use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cart {
    pub id: Option<i32>,
    pub items: Option<Vec<CartItem>>,
    #[serde(rename = "total_amount")]
    pub total_amount: Option<f64>,
    #[serde(rename = "sub_total")]
    pub sub_total: Option<f64>,
    #[serde(rename = "tax_amount")]
    pub tax_amount: Option<f64>,
    #[serde(rename = "delivery_charge")]
    pub delivery_charge: Option<f64>,
    #[serde(rename = "discount_amount")]
    pub discount_amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    pub id: Option<i32>,
    #[serde(rename = "product_id")]
    pub product_id: Option<i32>,
    pub product: Option<CartProduct>,
    pub quantity: Option<i32>,
    pub price: Option<f64>,
    #[serde(rename = "total_price")]
    pub total_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartProduct {
    pub id: Option<i32>,
    pub slug: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "main_image")]
    pub main_image: Option<String>,
    pub price: Option<f64>,
    #[serde(rename = "discount_price")]
    pub discount_price: Option<f64>,
    pub stock: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToCartRequest {
    #[serde(rename = "product_id")]
    pub product_id: i32,
    pub quantity: i32,
    #[serde(rename = "variant_id")]
    pub variant_id: Option<i32>,
}
