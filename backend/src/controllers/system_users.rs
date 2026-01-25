use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{User, UserResponse, NewUser, UpdateUser};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use rust_decimal::Decimal;
use crate::schema::users;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<UserResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<UserResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let total: i64 = users::table
        .filter(users::access_panel.eq("admin"))
        .filter(users::deleted_at.is_null())
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<User> = users::table
        .filter(users::access_panel.eq("admin"))
        .filter(users::deleted_at.is_null())
        .order(users::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch system users".to_string() }),
            )
        })?;

    let data: Vec<UserResponse> = results.into_iter().map(UserResponse::from).collect();

    Ok(Json(PaginatedResponse {
        data,
        total,
        page,
        per_page,
    }))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let user: Option<User> = users::table
        .find(id)
        .filter(users::access_panel.eq("admin"))
        .filter(users::deleted_at.is_null())
        .select(User::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch user".to_string() }),
            )
        })?;

    match user {
        Some(u) => Ok(Json(SuccessResponse {
            success: true,
            message: "User retrieved successfully".to_string(),
            data: Some(UserResponse::from(u)),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "User not found".to_string() }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub mobile: String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to hash password".to_string() }),
            )
        })?
        .to_string();

    let new_user = NewUser {
        name: payload.name,
        email: payload.email,
        password: password_hash,
        mobile: payload.mobile,
        referral_code: None,
        friends_code: None,
        reward_points: Decimal::new(0, 0),
        status: "active".to_string(),
        access_panel: Some("admin".to_string()),
        iso_2: None,
        country: None,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: format!("Failed to create user: {}", e) }),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse {
            success: true,
            message: "User created successfully".to_string(),
            data: None,
        }),
    ))
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub status: Option<String>,
    pub password: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let password_hash = if let Some(ref pwd) = payload.password {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Some(argon2.hash_password(pwd.as_bytes(), &salt)
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse { message: "Failed to hash password".to_string() }),
                )
            })?
            .to_string())
    } else {
        None
    };

    let update_data = UpdateUser {
        name: payload.name,
        email: payload.email,
        password: password_hash,
        mobile: payload.mobile,
        status: payload.status,
        access_panel: None,
    };

    diesel::update(users::table.find(id))
        .set(&update_data)
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to update user".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "User updated successfully".to_string(),
        data: None,
    }))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    // Soft delete
    diesel::update(users::table.find(id))
        .set(users::deleted_at.eq(chrono::Utc::now().naive_utc()))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete user".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
