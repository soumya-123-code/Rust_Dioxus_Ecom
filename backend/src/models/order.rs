use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::schema::{orders, order_items};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Order {
    pub id: u64,
    pub uuid: String,
    pub user_id: u64,
    pub slug: String,
    pub email: String,
    pub ip_address: String,
    pub currency_code: String,
    pub currency_rate: Decimal,
    pub payment_method: String,
    pub payment_status: String,
    pub fulfillment_type: String,
    pub is_rush_order: Option<bool>,
    pub estimated_delivery_time: Option<i32>,
    pub delivery_time_slot_id: Option<u64>,
    pub delivery_boy_id: Option<u64>,
    pub delivery_zone_id: Option<u64>,
    pub wallet_balance: Decimal,
    pub promo_code: Option<String>,
    pub promo_discount: Decimal,
    pub gift_card: Option<String>,
    pub gift_card_discount: Decimal,
    pub delivery_charge: Decimal,
    pub handling_charges: Option<Decimal>,
    pub per_store_drop_off_fee: Option<Decimal>,
    pub subtotal: Decimal,
    pub total_payable: Decimal,
    pub final_total: Decimal,
    pub status: Option<String>,
    pub billing_name: String,
    pub billing_address_1: String,
    pub billing_address_2: Option<String>,
    pub billing_landmark: String,
    pub billing_zip: String,
    pub billing_phone: String,
    pub billing_address_type: String,
    pub billing_latitude: Decimal,
    pub billing_longitude: Decimal,
    pub billing_city: String,
    pub billing_state: String,
    pub billing_country: String,
    pub billing_country_code: String,
    pub shipping_name: String,
    pub shipping_address_1: String,
    pub shipping_address_2: Option<String>,
    pub shipping_landmark: String,
    pub shipping_zip: String,
    pub shipping_phone: String,
    pub shipping_address_type: String,
    pub shipping_latitude: Decimal,
    pub shipping_longitude: Decimal,
    pub shipping_city: String,
    pub shipping_state: String,
    pub shipping_country: String,
    pub shipping_country_code: String,
    pub order_note: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = orders)]
pub struct UpdateOrder {
    pub payment_status: Option<String>,
    pub status: Option<String>,
    pub delivery_boy_id: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: u64,
    pub uuid: String,
    pub slug: String,
    pub user_id: u64,
    pub email: String,
    pub payment_method: String,
    pub payment_status: String,
    pub fulfillment_type: String,
    pub subtotal: Decimal,
    pub final_total: Decimal,
    pub status: Option<String>,
    pub billing_name: String,
    pub billing_city: String,
    pub shipping_name: String,
    pub shipping_city: String,
    pub created_at: Option<NaiveDateTime>,
}

impl From<Order> for OrderResponse {
    fn from(o: Order) -> Self {
        Self {
            id: o.id,
            uuid: o.uuid,
            slug: o.slug,
            user_id: o.user_id,
            email: o.email,
            payment_method: o.payment_method,
            payment_status: o.payment_status,
            fulfillment_type: o.fulfillment_type,
            subtotal: o.subtotal,
            final_total: o.final_total,
            status: o.status,
            billing_name: o.billing_name,
            billing_city: o.billing_city,
            shipping_name: o.shipping_name,
            shipping_city: o.shipping_city,
            created_at: o.created_at,
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = order_items)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct OrderItem {
    pub id: u64,
    pub order_id: u64,
    pub product_id: u64,
    pub product_variant_id: u64,
    pub store_id: u64,
    pub title: String,
    pub variant_title: String,
    pub gift_card_discount: Decimal,
    pub admin_commission_amount: Decimal,
    pub seller_commission_amount: Decimal,
    pub commission_settled: String,
    pub discounted_price: Decimal,
    pub discount: Decimal,
    pub tax_amount: Option<Decimal>,
    pub tax_percent: Option<Decimal>,
    pub promo_discount: Option<Decimal>,
    pub sku: String,
    pub quantity: i32,
    pub price: Decimal,
    pub subtotal: Decimal,
    pub status: String,
    pub return_status: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = order_items)]
pub struct UpdateOrderItem {
    pub status: Option<String>,
    pub return_status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub id: u64,
    pub order_id: u64,
    pub product_id: u64,
    pub title: String,
    pub variant_title: String,
    pub sku: String,
    pub quantity: i32,
    pub price: Decimal,
    pub subtotal: Decimal,
    pub status: String,
}

impl From<OrderItem> for OrderItemResponse {
    fn from(i: OrderItem) -> Self {
        Self {
            id: i.id,
            order_id: i.order_id,
            product_id: i.product_id,
            title: i.title,
            variant_title: i.variant_title,
            sku: i.sku,
            quantity: i.quantity,
            price: i.price,
            subtotal: i.subtotal,
            status: i.status,
        }
    }
}
