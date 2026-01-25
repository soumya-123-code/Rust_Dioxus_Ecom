use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::schema::delivery_boys;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = delivery_boys)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct DeliveryBoy {
    pub id: u64,
    pub user_id: u64,
    pub delivery_zone_id: Option<u64>,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub country: String,
    pub national_identity_card: String,
    pub driving_license: String,
    pub vehicle_registration: Option<String>,
    pub vehicle_number: Option<String>,
    pub vehicle_type: Option<String>,
    pub verification_status: String,
    pub metadata: Option<JsonValue>,
    pub availability_status: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = delivery_boys)]
pub struct NewDeliveryBoy {
    pub user_id: u64,
    pub delivery_zone_id: Option<u64>,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub country: String,
    pub national_identity_card: String,
    pub driving_license: String,
    pub vehicle_registration: Option<String>,
    pub vehicle_number: Option<String>,
    pub vehicle_type: Option<String>,
    pub verification_status: String,
    pub availability_status: String,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = delivery_boys)]
pub struct UpdateDeliveryBoy {
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub verification_status: Option<String>,
    pub availability_status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeliveryBoyResponse {
    pub id: u64,
    pub user_id: u64,
    pub delivery_zone_id: Option<u64>,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub vehicle_number: Option<String>,
    pub vehicle_type: Option<String>,
    pub verification_status: String,
    pub availability_status: String,
    pub created_at: Option<NaiveDateTime>,
}

impl From<DeliveryBoy> for DeliveryBoyResponse {
    fn from(db: DeliveryBoy) -> Self {
        Self {
            id: db.id,
            user_id: db.user_id,
            delivery_zone_id: db.delivery_zone_id,
            address: db.address,
            city: db.city,
            state: db.state,
            country: db.country,
            vehicle_number: db.vehicle_number,
            vehicle_type: db.vehicle_type,
            verification_status: db.verification_status,
            availability_status: db.availability_status,
            created_at: db.created_at,
        }
    }
}
