use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthModel {
    pub success: Option<bool>,
    pub message: Option<String>,
    #[serde(rename = "access_token")]
    pub access_token: Option<String>,
    #[serde(rename = "token_type")]
    pub token_type: Option<String>,
    pub data: Option<UserData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "iso_2")]
    pub iso2: Option<String>,
    #[serde(rename = "wallet_balance")]
    pub wallet_balance: Option<i32>,
    #[serde(rename = "referral_code")]
    pub referral_code: Option<String>,
    #[serde(rename = "friends_code")]
    pub friends_code: Option<String>,
    #[serde(rename = "reward_points")]
    pub reward_points: Option<i32>,
    #[serde(rename = "profile_image")]
    pub profile_image: Option<String>,
    #[serde(rename = "email_verified_at")]
    pub email_verified_at: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub password: String,
    #[serde(rename = "fcm_token")]
    pub fcm_token: Option<String>,
    #[serde(rename = "device_type")]
    pub device_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub country: String,
    pub iso2: String,
    pub password: String,
    #[serde(rename = "password_confirmation")]
    pub password_confirmation: String,
    #[serde(rename = "fcm_token")]
    pub fcm_token: Option<String>,
    #[serde(rename = "device_type")]
    pub device_type: Option<String>,
}
