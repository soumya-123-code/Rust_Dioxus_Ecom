use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::schema::promos;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = promos)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Promo {
    pub id: u64,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub min_order_amount: Option<Decimal>,
    pub max_discount_amount: Option<Decimal>,
    pub usage_limit: Option<i32>,
    pub usage_limit_per_user: Option<i32>,
    pub times_used: i32,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = promos)]
pub struct NewPromo {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub min_order_amount: Option<Decimal>,
    pub max_discount_amount: Option<Decimal>,
    pub usage_limit: Option<i32>,
    pub usage_limit_per_user: Option<i32>,
    pub times_used: i32,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub status: String,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = promos)]
pub struct UpdatePromo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub discount_type: Option<String>,
    pub discount_value: Option<Decimal>,
    pub min_order_amount: Option<Decimal>,
    pub max_discount_amount: Option<Decimal>,
    pub usage_limit: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PromoResponse {
    pub id: u64,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub min_order_amount: Option<Decimal>,
    pub max_discount_amount: Option<Decimal>,
    pub usage_limit: Option<i32>,
    pub times_used: i32,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
}

impl From<Promo> for PromoResponse {
    fn from(p: Promo) -> Self {
        Self {
            id: p.id,
            code: p.code,
            title: p.title,
            description: p.description,
            discount_type: p.discount_type,
            discount_value: p.discount_value,
            min_order_amount: p.min_order_amount,
            max_discount_amount: p.max_discount_amount,
            usage_limit: p.usage_limit,
            times_used: p.times_used,
            start_date: p.start_date,
            end_date: p.end_date,
            status: p.status,
            created_at: p.created_at,
        }
    }
}
