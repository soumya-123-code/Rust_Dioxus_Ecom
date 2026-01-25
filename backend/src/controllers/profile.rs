use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{User, UserResponse};
use crate::schema::users;
use crate::middlewares::auth::AuthenticatedUser;
use crate::utils::password;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}

pub async fn show(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let user_data: Option<User> = users::table
        .find(user.user_id)
        .select(User::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch user profile".to_string() }),
            )
        })?;

    match user_data {
        Some(u) => Ok(Json(UserResponse::from(u))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "User not found".to_string() }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
struct ProfileUpdate<'a> {
    name: Option<&'a str>,
    email: Option<&'a str>,
    mobile: Option<&'a str>,
}

pub async fn update(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let update_data = ProfileUpdate {
        name: payload.name.as_deref(),
        email: payload.email.as_deref(),
        mobile: payload.mobile.as_deref(),
    };

    diesel::update(users::table.find(user.user_id))
        .set(&update_data)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update profile: {}", e) }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Profile updated successfully".to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

pub async fn change_password(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    if payload.new_password != payload.confirm_password {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { message: "New passwords do not match".to_string() }),
        ));
    }

    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    // Fetch current user to verify password
    let user_data: User = users::table
        .find(user.user_id)
        .select(User::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse { message: "User not found".to_string() }),
            )
        })?;

    if !password::verify(&payload.current_password, &user_data.password) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { message: "Incorrect current password".to_string() }),
        ));
    }

    let new_hash = password::hash(&payload.new_password).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: format!("Failed to hash password: {}", e) }),
        )
    })?;

    diesel::update(users::table.find(user.user_id))
        .set(users::password.eq(new_hash))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: format!("Failed to update password: {}", e) }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Password changed successfully".to_string(),
    }))
}
