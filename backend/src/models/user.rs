use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub remember_token: Option<String>,
    pub mobile: String,
    pub referral_code: Option<String>,
    pub friends_code: Option<String>,
    pub reward_points: Decimal,
    pub status: String,
    pub access_panel: Option<String>,
    pub iso_2: Option<String>,
    pub country: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub mobile: String,
    pub referral_code: Option<String>,
    pub friends_code: Option<String>,
    pub reward_points: Decimal,
    pub status: String,
    pub access_panel: Option<String>,
    pub iso_2: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub mobile: Option<String>,
    pub status: Option<String>,
    pub access_panel: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub referral_code: Option<String>,
    pub reward_points: Decimal,
    pub status: String,
    pub access_panel: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            mobile: user.mobile,
            referral_code: user.referral_code,
            reward_points: user.reward_points,
            status: user.status,
            access_panel: user.access_panel,
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
