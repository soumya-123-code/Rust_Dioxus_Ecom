use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::schema::categories;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Category {
    pub id: u64,
    pub uuid: String,
    pub parent_id: Option<u64>,
    pub title: String,
    pub slug: String,
    pub image: String,
    pub banner: Option<String>,
    pub description: String,
    pub status: String,
    pub requires_approval: bool,
    pub commission: Option<Decimal>,
    pub background_type: Option<String>,
    pub background_color: Option<String>,
    pub font_color: Option<String>,
    pub metadata: JsonValue,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub uuid: String,
    pub parent_id: Option<u64>,
    pub title: String,
    pub slug: String,
    pub image: String,
    pub banner: Option<String>,
    pub description: String,
    pub status: String,
    pub requires_approval: bool,
    pub commission: Option<Decimal>,
    pub background_type: Option<String>,
    pub background_color: Option<String>,
    pub font_color: Option<String>,
    pub metadata: JsonValue,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = categories)]
pub struct UpdateCategory {
    pub parent_id: Option<u64>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub image: Option<String>,
    pub banner: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub requires_approval: Option<bool>,
    pub commission: Option<Decimal>,
    pub background_type: Option<String>,
    pub background_color: Option<String>,
    pub font_color: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: u64,
    pub uuid: String,
    pub parent_id: Option<u64>,
    pub title: String,
    pub slug: String,
    pub image: String,
    pub banner: Option<String>,
    pub description: String,
    pub status: String,
    pub requires_approval: bool,
    pub commission: Option<Decimal>,
    pub background_type: Option<String>,
    pub background_color: Option<String>,
    pub font_color: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

impl From<Category> for CategoryResponse {
    fn from(cat: Category) -> Self {
        Self {
            id: cat.id,
            uuid: cat.uuid,
            parent_id: cat.parent_id,
            title: cat.title,
            slug: cat.slug,
            image: cat.image,
            banner: cat.banner,
            description: cat.description,
            status: cat.status,
            requires_approval: cat.requires_approval,
            commission: cat.commission,
            background_type: cat.background_type,
            background_color: cat.background_color,
            font_color: cat.font_color,
            created_at: cat.created_at,
        }
    }
}
