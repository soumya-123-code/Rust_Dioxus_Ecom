use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::models::{UserResponse, LoginRequest, LoginResponse};
use crate::services::auth::AuthService;
use crate::utils::types::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    match AuthService::login(&mut conn, &payload.email, &payload.password, &state.jwt_secret) {
        Ok((token, user)) => Ok(Json(LoginResponse { token, user })),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse { message: e }),
        )),
    }
}

pub async fn logout() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "message": "Logged out successfully" }))
}

pub async fn me(
    State(_state): State<AppState>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    // This is a placeholder - implement JWT verification middleware
    Err((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse { message: "Not authenticated".to_string() }),
    ))
}
