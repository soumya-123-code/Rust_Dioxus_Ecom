use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{Seller, UpdateSeller, SellerResponse};
use crate::schema::sellers;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<SellerResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub verification_status: Option<String>,
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
) -> Result<Json<Vec<SellerResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = sellers::table
        .filter(sellers::deleted_at.is_null())
        .into_boxed();

    if let Some(verification_status) = params.verification_status {
        query = query.filter(sellers::verification_status.eq(verification_status));
    }

    let results: Vec<Seller> = query
        .order(sellers::created_at.desc())
        .select(Seller::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch sellers".to_string() }),
            )
        })?;

    let response: Vec<SellerResponse> = results.into_iter().map(SellerResponse::from).collect();

    Ok(Json(response))
}

pub async fn datatable(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<SellerResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let total: i64 = sellers::table
        .filter(sellers::deleted_at.is_null())
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<Seller> = sellers::table
        .filter(sellers::deleted_at.is_null())
        .order(sellers::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .select(Seller::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch sellers".to_string() }),
            )
        })?;

    let data: Vec<SellerResponse> = results.into_iter().map(SellerResponse::from).collect();

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

    let seller: Option<Seller> = sellers::table
        .find(id)
        .filter(sellers::deleted_at.is_null())
        .select(Seller::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch seller".to_string() }),
            )
        })?;

    match seller {
        Some(s) => Ok(Json(SuccessResponse {
            success: true,
            message: "Seller retrieved successfully".to_string(),
            data: Some(SellerResponse::from(s)),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Seller not found".to_string() }),
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

    diesel::update(sellers::table.find(id))
        .set(sellers::verification_status.eq(&payload.verification_status))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update verification status: {}", e) }),
            )
        })?;

    let updated: Seller = sellers::table
        .find(id)
        .select(Seller::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated seller".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Verification status updated successfully".to_string(),
        data: Some(SellerResponse::from(updated)),
    }))
}
