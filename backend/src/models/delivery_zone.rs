use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::schema::delivery_zones;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = delivery_zones)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct DeliveryZone {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub center_latitude: Decimal,
    pub center_longitude: Decimal,
    pub radius_km: f64,
    pub boundary_json: Option<JsonValue>,
    pub status: String,
    pub delivery_time_per_km: Option<i32>,
    pub buffer_time: Option<i32>,
    pub min_order_amount: Option<Decimal>,
    pub delivery_charge_type: Option<String>,
    pub base_delivery_charge: Option<Decimal>,
    pub delivery_charge_per_km: Option<Decimal>,
    pub free_delivery_above: Option<Decimal>,
    pub per_order_earning: Option<Decimal>,
    pub per_km_earning: Option<Decimal>,
    pub base_earning: Option<Decimal>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = delivery_zones)]
pub struct NewDeliveryZone {
    pub name: String,
    pub slug: String,
    pub center_latitude: Decimal,
    pub center_longitude: Decimal,
    pub radius_km: f64,
    pub boundary_json: Option<JsonValue>,
    pub status: String,
    pub delivery_time_per_km: Option<i32>,
    pub buffer_time: Option<i32>,
    pub min_order_amount: Option<Decimal>,
    pub delivery_charge_type: Option<String>,
    pub base_delivery_charge: Option<Decimal>,
    pub delivery_charge_per_km: Option<Decimal>,
    pub free_delivery_above: Option<Decimal>,
    pub per_order_earning: Option<Decimal>,
    pub per_km_earning: Option<Decimal>,
    pub base_earning: Option<Decimal>,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = delivery_zones)]
pub struct UpdateDeliveryZone {
    pub name: Option<String>,
    pub center_latitude: Option<Decimal>,
    pub center_longitude: Option<Decimal>,
    pub radius_km: Option<f64>,
    pub status: Option<String>,
    pub min_order_amount: Option<Decimal>,
    pub base_delivery_charge: Option<Decimal>,
}

#[derive(Debug, Serialize)]
pub struct DeliveryZoneResponse {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub center_latitude: Decimal,
    pub center_longitude: Decimal,
    pub radius_km: f64,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
}

impl From<DeliveryZone> for DeliveryZoneResponse {
    fn from(dz: DeliveryZone) -> Self {
        Self {
            id: dz.id,
            name: dz.name,
            slug: dz.slug,
            center_latitude: dz.center_latitude,
            center_longitude: dz.center_longitude,
            radius_km: dz.radius_km,
            status: dz.status,
            created_at: dz.created_at,
        }
    }
}
