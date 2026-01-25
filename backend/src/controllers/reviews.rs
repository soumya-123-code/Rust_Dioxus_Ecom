use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{reviews, users, products};
use crate::utils::api_response::{self, ApiResponse};
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ReviewResponse {
    pub id: u64,
    pub rating: i32,
    pub comment: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
    pub user_name: String,
    pub product_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<Vec<ReviewResponse>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Database connection failed".to_string())),
        )
    })?;

    let mut query = reviews::table
        .inner_join(users::table.on(reviews::user_id.eq(users::id)))
        .inner_join(products::table.on(reviews::product_id.eq(products::id)))
        .select((
            reviews::id,
            reviews::rating,
            reviews::comment,
            reviews::status,
            reviews::created_at,
            users::name,
            products::title,
        ))
        .into_boxed();

    if let Some(status) = params.status {
        query = query.filter(reviews::status.eq(status));
    }

    let results = query
        .limit(params.per_page.unwrap_or(10))
        .offset((params.page.unwrap_or(1) - 1) * params.per_page.unwrap_or(10))
        .load::<(u64, i32, Option<String>, String, Option<chrono::NaiveDateTime>, String, String)>(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to fetch reviews: {}", e))),
            )
        })?;

    let response: Vec<ReviewResponse> = results
        .into_iter()
        .map(|(id, rating, comment, status, created_at, user_name, product_name)| ReviewResponse {
            id,
            rating,
            comment,
            status,
            created_at: created_at.map(|d| d.to_string()),
            user_name,
            product_name,
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

pub async fn update_status(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateStatusRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Database connection failed".to_string())),
        )
    })?;

    diesel::update(reviews::table.find(id))
        .set(reviews::status.eq(&payload.status))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to update review status: {}", e))),
            )
        })?;

    Ok(Json(ApiResponse::success_message("Review status updated successfully")))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Database connection failed".to_string())),
        )
    })?;

    diesel::delete(reviews::table.find(id))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to delete review: {}", e))),
            )
        })?;

    Ok(Json(ApiResponse::success_message("Review deleted successfully")))
}
