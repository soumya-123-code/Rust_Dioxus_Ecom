use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::schema::products;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Product {
    pub id: u64,
    pub uuid: String,
    pub seller_id: u64,
    pub category_id: u64,
    pub brand_id: Option<u64>,
    pub product_condition_id: u64,
    pub provider: Option<String>,
    pub provider_product_id: Option<u64>,
    pub slug: String,
    pub title: String,
    pub product_identity: Option<i32>,
    pub product_type: String,
    pub short_description: String,
    pub description: String,
    pub indicator: Option<String>,
    pub download_allowed: String,
    pub download_link: Option<String>,
    pub minimum_order_quantity: i32,
    pub quantity_step_size: i32,
    pub total_allowed_quantity: i32,
    pub is_inclusive_tax: String,
    pub hsn_code: Option<String>,
    pub is_returnable: String,
    pub returnable_days: Option<i32>,
    pub is_cancelable: String,
    pub cancelable_till: Option<String>,
    pub is_attachment_required: String,
    pub base_prep_time: Option<i32>,
    pub status: String,
    pub verification_status: Option<String>,
    pub rejection_reason: Option<String>,
    pub featured: String,
    pub requires_otp: Option<bool>,
    pub video_type: Option<String>,
    pub video_link: Option<String>,
    pub cloned_from_id: Option<u64>,
    pub tags: String,
    pub warranty_period: Option<String>,
    pub guarantee_period: Option<String>,
    pub made_in: Option<String>,
    pub image_fit: Option<String>,
    pub metadata: JsonValue,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub uuid: String,
    pub seller_id: u64,
    pub category_id: u64,
    pub brand_id: Option<u64>,
    pub product_condition_id: u64,
    pub provider: Option<String>,
    pub provider_product_id: Option<u64>,
    pub slug: String,
    pub title: String,
    pub product_identity: Option<i32>,
    pub product_type: String,
    pub short_description: String,
    pub description: String,
    pub indicator: Option<String>,
    pub download_allowed: String,
    pub download_link: Option<String>,
    pub minimum_order_quantity: i32,
    pub quantity_step_size: i32,
    pub total_allowed_quantity: i32,
    pub is_inclusive_tax: String,
    pub hsn_code: Option<String>,
    pub is_returnable: String,
    pub returnable_days: Option<i32>,
    pub is_cancelable: String,
    pub cancelable_till: Option<String>,
    pub is_attachment_required: String,
    pub base_prep_time: Option<i32>,
    pub status: String,
    pub verification_status: Option<String>,
    pub featured: String,
    pub requires_otp: Option<bool>,
    pub video_type: Option<String>,
    pub video_link: Option<String>,
    pub tags: String,
    pub warranty_period: Option<String>,
    pub guarantee_period: Option<String>,
    pub made_in: Option<String>,
    pub image_fit: Option<String>,
    pub metadata: JsonValue,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = products)]
pub struct UpdateProduct {
    pub category_id: Option<u64>,
    pub brand_id: Option<u64>,
    pub title: Option<String>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub indicator: Option<String>,
    pub status: Option<String>,
    pub verification_status: Option<String>,
    pub rejection_reason: Option<String>,
    pub featured: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: u64,
    pub uuid: String,
    pub seller_id: u64,
    pub category_id: u64,
    pub brand_id: Option<u64>,
    pub slug: String,
    pub title: String,
    pub product_type: String,
    pub short_description: String,
    pub status: String,
    pub verification_status: Option<String>,
    pub featured: String,
    pub created_at: Option<NaiveDateTime>,
}

impl From<Product> for ProductResponse {
    fn from(p: Product) -> Self {
        Self {
            id: p.id,
            uuid: p.uuid,
            seller_id: p.seller_id,
            category_id: p.category_id,
            brand_id: p.brand_id,
            slug: p.slug,
            title: p.title,
            product_type: p.product_type,
            short_description: p.short_description,
            status: p.status,
            verification_status: p.verification_status,
            featured: p.featured,
            created_at: p.created_at,
        }
    }
}
