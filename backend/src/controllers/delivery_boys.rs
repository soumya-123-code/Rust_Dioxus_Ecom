use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{DeliveryBoy, UpdateDeliveryBoy, DeliveryBoyResponse};
use crate::schema::delivery_boys;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<DeliveryBoyResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub verification_status: Option<String>,
    pub availability_status: Option<String>,
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
) -> Result<Json<Vec<DeliveryBoyResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = delivery_boys::table
        .filter(delivery_boys::deleted_at.is_null())
        .into_boxed();

    if let Some(verification_status) = params.verification_status {
        query = query.filter(delivery_boys::verification_status.eq(verification_status));
    }

    if let Some(availability_status) = params.availability_status {
        query = query.filter(delivery_boys::availability_status.eq(availability_status));
    }

    let results: Vec<DeliveryBoy> = query
        .order(delivery_boys::created_at.desc())
        .select(DeliveryBoy::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch delivery boys".to_string() }),
            )
        })?;

    let response: Vec<DeliveryBoyResponse> = results.into_iter().map(DeliveryBoyResponse::from).collect();

    Ok(Json(response))
}

pub async fn datatable(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<DeliveryBoyResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let total: i64 = delivery_boys::table
        .filter(delivery_boys::deleted_at.is_null())
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<DeliveryBoy> = delivery_boys::table
        .filter(delivery_boys::deleted_at.is_null())
        .order(delivery_boys::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch delivery boys".to_string() }),
            )
        })?;

    let data: Vec<DeliveryBoyResponse> = results.into_iter().map(DeliveryBoyResponse::from).collect();

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

    let boy: Option<DeliveryBoy> = delivery_boys::table
        .find(id)
        .filter(delivery_boys::deleted_at.is_null())
        .select(DeliveryBoy::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch delivery boy".to_string() }),
            )
        })?;

    match boy {
        Some(db) => Ok(Json(SuccessResponse {
            success: true,
            message: "Delivery boy retrieved successfully".to_string(),
            data: Some(DeliveryBoyResponse::from(db)),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Delivery boy not found".to_string() }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateVerificationStatusRequest {
    pub verification_status: String,
}

pub async fn update_verification_status(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateVerificationStatusRequest>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(delivery_boys::table.find(id))
        .set(delivery_boys::verification_status.eq(&payload.verification_status))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update verification status: {}", e) }),
            )
        })?;

    let updated: DeliveryBoy = delivery_boys::table
        .find(id)
        .select(DeliveryBoy::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated delivery boy".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Verification status updated successfully".to_string(),
        data: Some(DeliveryBoyResponse::from(updated)),
    }))
}
