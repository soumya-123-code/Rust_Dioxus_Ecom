use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::schema::stores;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = stores)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Store {
    pub id: u64,
    pub seller_id: u64,
    pub delivery_zone_id: Option<u64>,
    pub name: String,
    pub slug: String,
    pub address: String,
    pub city: String,
    pub landmark: String,
    pub state: String,
    pub zipcode: String,
    pub country: String,
    pub country_code: String,
    pub latitude: Decimal,
    pub longitude: Decimal,
    pub contact_email: String,
    pub contact_number: String,
    pub description: Option<String>,
    pub store_url: Option<String>,
    pub timing: Option<String>,
    pub address_proof: String,
    pub voided_check: String,
    pub tax_name: String,
    pub tax_number: String,
    pub bank_name: String,
    pub bank_branch_code: String,
    pub account_holder_name: String,
    pub account_number: String,
    pub routing_number: String,
    pub bank_account_type: String,
    pub currency_code: String,
    pub permissions: Option<String>,
    pub time_slot_config: Option<JsonValue>,
    pub max_delivery_distance: f64,
    pub shipping_min_free_delivery_amount: f64,
    pub shipping_charge_priority: Option<String>,
    pub allowed_order_per_time_slot: Option<i32>,
    pub order_preparation_time: i32,
    pub carrier_partner: Option<String>,
    pub promotional_text: Option<String>,
    pub about_us: String,
    pub return_replacement_policy: Option<String>,
    pub refund_policy: Option<String>,
    pub terms_and_conditions: Option<String>,
    pub delivery_policy: Option<String>,
    pub shipping_preference: Option<String>,
    pub domestic_shipping_charges: Option<Decimal>,
    pub international_shipping_charges: Option<Decimal>,
    pub metadata: JsonValue,
    pub verification_status: String,
    pub visibility_status: String,
    pub fulfillment_type: String,
    pub status: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = stores)]
pub struct UpdateStore {
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub verification_status: Option<String>,
    pub visibility_status: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StoreResponse {
    pub id: u64,
    pub seller_id: u64,
    pub name: String,
    pub slug: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub contact_email: String,
    pub contact_number: String,
    pub verification_status: String,
    pub visibility_status: String,
    pub fulfillment_type: String,
    pub created_at: Option<NaiveDateTime>,
}

impl From<Store> for StoreResponse {
    fn from(s: Store) -> Self {
        Self {
            id: s.id,
            seller_id: s.seller_id,
            name: s.name,
            slug: s.slug,
            city: s.city,
            state: s.state,
            country: s.country,
            contact_email: s.contact_email,
            contact_number: s.contact_number,
            verification_status: s.verification_status,
            visibility_status: s.visibility_status,
            fulfillment_type: s.fulfillment_type,
            created_at: s.created_at,
        }
    }
}
