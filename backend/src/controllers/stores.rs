use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{Store, UpdateStore, StoreResponse};
use crate::schema::stores;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<StoreResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub verification_status: Option<String>,
    pub seller_id: Option<u64>,
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
) -> Result<Json<Vec<StoreResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = stores::table
        .filter(stores::deleted_at.is_null())
        .into_boxed();

    if let Some(verification_status) = params.verification_status {
        query = query.filter(stores::verification_status.eq(verification_status));
    }

    if let Some(seller_id) = params.seller_id {
        query = query.filter(stores::seller_id.eq(seller_id));
    }

    if let Some(search) = params.search {
        query = query.filter(stores::name.like(format!("%{}%", search)));
    }

    let results: Vec<Store> = query
        .order(stores::created_at.desc())
        .select(Store::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch stores".to_string() }),
            )
        })?;

    let response: Vec<StoreResponse> = results.into_iter().map(StoreResponse::from).collect();

    Ok(Json(response))
}

pub async fn datatable(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<StoreResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let total: i64 = stores::table
        .filter(stores::deleted_at.is_null())
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<Store> = stores::table
        .filter(stores::deleted_at.is_null())
        .order(stores::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .select(Store::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch stores".to_string() }),
            )
        })?;

    let data: Vec<StoreResponse> = results.into_iter().map(StoreResponse::from).collect();

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

    let store: Option<Store> = stores::table
        .find(id)
        .filter(stores::deleted_at.is_null())
        .select(Store::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch store".to_string() }),
            )
        })?;

    match store {
        Some(s) => Ok(Json(SuccessResponse {
            success: true,
            message: "Store retrieved successfully".to_string(),
            data: Some(StoreResponse::from(s)),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Store not found".to_string() }),
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

    diesel::update(stores::table.find(id))
        .set(stores::verification_status.eq(&payload.verification_status))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update verification status: {}", e) }),
            )
        })?;

    let updated: Store = stores::table
        .find(id)
        .select(Store::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated store".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Verification status updated successfully".to_string(),
        data: Some(StoreResponse::from(updated)),
    }))
}
