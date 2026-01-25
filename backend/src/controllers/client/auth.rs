use axum::{Json, http::StatusCode};
use crate::models::user::{NewUser, User};
use crate::utils::types::AppState;
use axum::extract::State;
use serde::{Deserialize};
use crate::utils::{jwt, password};
use diesel::prelude::*;
use crate::schema::users;
use rust_decimal::Decimal;

// Re-use login from admin auth for now
pub use crate::controllers::auth::login;
pub use crate::controllers::auth::logout;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub country: String,
    #[serde(rename = "iso2")]
    pub iso2: String,
    pub password: String,
    #[serde(rename = "password_confirmation")]
    pub password_confirmation: String,
    #[serde(rename = "fcm_token")]
    pub fcm_token: Option<String>,
    #[serde(rename = "device_type")]
    pub device_type: Option<String>,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    if payload.password != payload.password_confirmation {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": "Passwords do not match"
            })),
        );
    }

    let mut conn = match state.db_pool.get() {
        Ok(conn) => conn,
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "message": "Database connection failed"
            })),
        ),
    };

    // Check if user exists
    let count: i64 = users::table
        .filter(users::email.eq(&payload.email))
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    if count > 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": "Email already registered"
            })),
        );
    }

    let hashed_password = match password::hash(&payload.password) {
        Ok(h) => h,
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "message": "Password hashing failed"
            })),
        ),
    };

    let new_user = NewUser {
        name: payload.name,
        email: payload.email,
        password: hashed_password,
        mobile: payload.mobile,
        referral_code: None,
        friends_code: None,
        reward_points: Decimal::new(0, 0),
        status: "active".to_string(),
        access_panel: Some("user".to_string()),
        iso_2: Some(payload.iso2),
        country: Some(payload.country),
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
    {
        Ok(_) => {},
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "message": format!("Registration failed: {}", e)
            })),
        ),
    };

    // Retrieve the newly created user
    let user: User = match users::table
        .filter(users::email.eq(&new_user.email))
        .first(&mut conn)
    {
        Ok(u) => u,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "message": format!("Failed to retrieve user: {}", e)
            })),
        ),
    };

    let token = match jwt::generate_token(&state.jwt_secret, user.id) {
        Ok(t) => t,
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "message": "Token generation failed"
            })),
        ),
    };

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "message": "Registration successful",
            "access_token": token,
            "token_type": "Bearer",
            "data": {
                "id": user.id,
                "name": user.name,
                "email": user.email,
                "mobile": user.mobile,
                "country": user.country,
                "iso_2": user.iso_2,
                "wallet_balance": 0,
                "reward_points": 0,
                "referral_code": user.referral_code,
                "status": user.status,
                "created_at": user.created_at,
                "updated_at": user.updated_at
            }
        })),
    )
}
