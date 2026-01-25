use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::schema::brands;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = brands)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Brand {
    pub id: u64,
    pub uuid: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub banner: Option<String>,
    pub status: String,
    pub is_featured: Option<bool>,
    pub metadata: JsonValue,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = brands)]
pub struct NewBrand {
    pub uuid: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub banner: Option<String>,
    pub status: String,
    pub is_featured: Option<bool>,
    pub metadata: JsonValue,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = brands)]
pub struct UpdateBrand {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub banner: Option<String>,
    pub status: Option<String>,
    pub is_featured: Option<bool>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize)]
pub struct BrandResponse {
    pub id: u64,
    pub uuid: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub banner: Option<String>,
    pub status: String,
    pub is_featured: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
}

impl From<Brand> for BrandResponse {
    fn from(brand: Brand) -> Self {
        Self {
            id: brand.id,
            uuid: brand.uuid,
            slug: brand.slug,
            title: brand.title,
            description: brand.description,
            image: brand.image,
            banner: brand.banner,
            status: brand.status,
            is_featured: brand.is_featured,
            created_at: brand.created_at,
        }
    }
}
