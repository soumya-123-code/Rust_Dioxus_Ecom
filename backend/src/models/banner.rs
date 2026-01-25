use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::banners;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = banners)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Banner {
    pub id: u64,
    pub title: String,
    pub image: String,
    pub link_type: Option<String>,
    pub link_value: Option<String>,
    pub position: Option<String>,
    pub sort_order: Option<i32>,
    pub status: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = banners)]
pub struct NewBanner {
    pub title: String,
    pub image: String,
    pub link_type: Option<String>,
    pub link_value: Option<String>,
    pub position: Option<String>,
    pub sort_order: Option<i32>,
    pub status: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = banners)]
pub struct UpdateBanner {
    pub title: Option<String>,
    pub image: Option<String>,
    pub link_type: Option<String>,
    pub link_value: Option<String>,
    pub position: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BannerResponse {
    pub id: u64,
    pub title: String,
    pub image: String,
    pub link_type: Option<String>,
    pub link_value: Option<String>,
    pub position: Option<String>,
    pub sort_order: Option<i32>,
    pub status: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

impl From<Banner> for BannerResponse {
    fn from(b: Banner) -> Self {
        Self {
            id: b.id,
            title: b.title,
            image: b.image,
            link_type: b.link_type,
            link_value: b.link_value,
            position: b.position,
            sort_order: b.sort_order,
            status: b.status,
            start_date: b.start_date,
            end_date: b.end_date,
            created_at: b.created_at,
        }
    }
}
