use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::schema::sellers;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = sellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Seller {
    pub id: u64,
    pub user_id: u64,
    pub address: String,
    pub city: String,
    pub landmark: String,
    pub state: String,
    pub zipcode: String,
    pub country: String,
    pub country_code: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub business_license: String,
    pub articles_of_incorporation: String,
    pub national_identity_card: String,
    pub authorized_signature: String,
    pub verification_status: String,
    pub metadata: JsonValue,
    pub visibility_status: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = sellers)]
pub struct NewSeller {
    pub user_id: u64,
    pub address: String,
    pub city: String,
    pub landmark: String,
    pub state: String,
    pub zipcode: String,
    pub country: String,
    pub country_code: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub business_license: String,
    pub articles_of_incorporation: String,
    pub national_identity_card: String,
    pub authorized_signature: String,
    pub verification_status: String,
    pub metadata: JsonValue,
    pub visibility_status: String,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = sellers)]
pub struct UpdateSeller {
    pub address: Option<String>,
    pub city: Option<String>,
    pub landmark: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub verification_status: Option<String>,
    pub visibility_status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SellerResponse {
    pub id: u64,
    pub user_id: u64,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub verification_status: String,
    pub visibility_status: String,
    pub created_at: Option<NaiveDateTime>,
}

impl From<Seller> for SellerResponse {
    fn from(s: Seller) -> Self {
        Self {
            id: s.id,
            user_id: s.user_id,
            address: s.address,
            city: s.city,
            state: s.state,
            country: s.country,
            verification_status: s.verification_status,
            visibility_status: s.visibility_status,
            created_at: s.created_at,
        }
    }
}
