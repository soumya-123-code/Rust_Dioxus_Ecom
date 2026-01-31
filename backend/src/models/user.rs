use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::schema::users;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: u64,
    pub uuid: String,
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub phone: Option<String>,
    pub phone_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub remember_token: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub gender: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub referral_code: Option<String>,
    pub referred_by: Option<u64>,
    pub status: Option<String>,
    pub last_login_at: Option<NaiveDateTime>,
    pub last_login_ip: Option<String>,
    pub fcm_token: Option<String>,
    pub device_type: Option<String>,
    pub metadata: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub uuid: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub referral_code: Option<String>,
    pub referred_by: Option<u64>,
}

#[derive(Debug, Clone, Default, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub gender: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u64,
    pub uuid: String,
    pub email: String,
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar: Option<String>,
    pub referral_code: Option<String>,
    pub status: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            uuid: user.uuid,
            email: user.email,
            phone: user.phone,
            first_name: user.first_name,
            last_name: user.last_name,
            avatar: user.avatar,
            referral_code: user.referral_code,
            status: user.status,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}
