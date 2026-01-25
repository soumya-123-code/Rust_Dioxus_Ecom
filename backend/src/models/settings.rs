use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::settings;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = settings)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Setting {
    pub id: u64,
    pub variable: String,
    pub value: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = settings)]
pub struct NewSetting {
    pub variable: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = settings)]
pub struct UpdateSetting {
    pub value: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SettingResponse {
    pub id: u64,
    pub variable: String,
    pub value: Option<String>,
}

impl From<Setting> for SettingResponse {
    fn from(s: Setting) -> Self {
        Self {
            id: s.id,
            variable: s.variable,
            value: s.value,
        }
    }
}
