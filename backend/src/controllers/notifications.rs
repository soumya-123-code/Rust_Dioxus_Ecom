use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::schema::notifications;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = notifications)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Notification {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub target_type: Option<String>,
    pub target_id: Option<u64>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = notifications)]
pub struct NewNotification {
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub target_type: Option<String>,
    pub target_id: Option<u64>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<Notification>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let results: Vec<Notification> = notifications::table
        .select(Notification::as_select())
        .order(notifications::created_at.desc())
        .limit(100)
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch notifications".to_string() }),
            )
        })?;

    Ok(Json(results))
}

#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub target_type: Option<String>,
    pub target_id: Option<u64>,
}

pub async fn send(
    State(state): State<AppState>,
    Json(payload): Json<SendNotificationRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let new_notification = NewNotification {
        title: payload.title,
        message: payload.message,
        notification_type: payload.notification_type,
        target_type: payload.target_type,
        target_id: payload.target_id,
        created_at: Some(chrono::Utc::now().naive_utc()),
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };

    diesel::insert_into(notifications::table)
        .values(&new_notification)
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to send notification".to_string() }),
            )
        })?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Notification sent successfully"
    })))
}
