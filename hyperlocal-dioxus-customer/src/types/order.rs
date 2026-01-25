use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<i32>,
    #[serde(rename = "order_slug")]
    pub order_slug: Option<String>,
    #[serde(rename = "order_number")]
    pub order_number: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "total_amount")]
    pub total_amount: Option<f64>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    pub items: Option<Vec<OrderItem>>,
    pub address: Option<OrderAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Option<i32>,
    pub product: Option<OrderProduct>,
    pub quantity: Option<i32>,
    pub price: Option<f64>,
    #[serde(rename = "total_price")]
    pub total_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderProduct {
    pub id: Option<i32>,
    pub name: Option<String>,
    #[serde(rename = "main_image")]
    pub main_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAddress {
    pub id: Option<i32>,
    #[serde(rename = "address_line1")]
    pub address_line1: Option<String>,
    #[serde(rename = "address_line2")]
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub landmark: Option<String>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
}
