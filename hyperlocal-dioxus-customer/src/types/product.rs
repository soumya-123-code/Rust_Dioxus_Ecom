use serde::{Deserialize, Serialize};
use super::category::Category;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<i32>,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "main_image")]
    pub main_image: Option<String>,
    pub price: Option<f64>,
    #[serde(rename = "discount_price")]
    pub discount_price: Option<f64>,
    pub stock: Option<i32>,
    pub rating: Option<f64>,
    #[serde(rename = "total_reviews")]
    pub total_reviews: Option<i32>,
    pub seller: Option<Seller>,
    pub category: Option<Category>,
    pub images: Option<Vec<String>>,
    pub variants: Option<Vec<ProductVariant>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Seller {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub logo: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductVariant {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListResponse {
    pub success: Option<bool>,
    pub data: Option<Vec<Product>>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Banner {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub image: Option<String>,
    pub link: Option<String>,
    pub slug: Option<String>,
}
