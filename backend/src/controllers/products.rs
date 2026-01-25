use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{Product, UpdateProduct, ProductResponse};
use crate::schema::products;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<ProductResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub verification_status: Option<String>,
    pub category_id: Option<u64>,
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
) -> Result<Json<Vec<ProductResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = products::table
        .filter(products::deleted_at.is_null())
        .into_boxed();

    if let Some(status) = params.status {
        query = query.filter(products::status.eq(status));
    }

    if let Some(verification_status) = params.verification_status {
        query = query.filter(products::verification_status.eq(verification_status));
    }

    if let Some(search) = params.search {
        query = query.filter(products::title.like(format!("%{}%", search)));
    }

    if let Some(category_id) = params.category_id {
        query = query.filter(products::category_id.eq(category_id));
    }

    if let Some(seller_id) = params.seller_id {
        query = query.filter(products::seller_id.eq(seller_id));
    }

    let results: Vec<Product> = query
        .order(products::created_at.desc())
        .select(Product::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch products".to_string() }),
            )
        })?;

    let response: Vec<ProductResponse> = results.into_iter().map(ProductResponse::from).collect();

    Ok(Json(response))
}

pub async fn datatable(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<ProductResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let mut query = products::table
        .filter(products::deleted_at.is_null())
        .into_boxed();

    let mut count_query = products::table
        .filter(products::deleted_at.is_null())
        .into_boxed();

    if let Some(status) = &params.status {
        query = query.filter(products::status.eq(status));
        count_query = count_query.filter(products::status.eq(status));
    }

    if let Some(verification_status) = &params.verification_status {
        query = query.filter(products::verification_status.eq(verification_status));
        count_query = count_query.filter(products::verification_status.eq(verification_status));
    }

    if let Some(search) = &params.search {
        query = query.filter(products::title.like(format!("%{}%", search)));
        count_query = count_query.filter(products::title.like(format!("%{}%", search)));
    }

    if let Some(category_id) = params.category_id {
        query = query.filter(products::category_id.eq(category_id));
        count_query = count_query.filter(products::category_id.eq(category_id));
    }

    if let Some(seller_id) = params.seller_id {
        query = query.filter(products::seller_id.eq(seller_id));
        count_query = count_query.filter(products::seller_id.eq(seller_id));
    }

    let total: i64 = count_query
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<Product> = query
        .order(products::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .select(Product::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch products".to_string() }),
            )
        })?;

    let data: Vec<ProductResponse> = results.into_iter().map(ProductResponse::from).collect();

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

    let product: Option<Product> = products::table
        .find(id)
        .filter(products::deleted_at.is_null())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch product".to_string() }),
            )
        })?;

    match product {
        Some(p) => Ok(Json(SuccessResponse {
            success: true,
            message: "Product retrieved successfully".to_string(),
            data: Some(ProductResponse::from(p)),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Product not found".to_string() }),
        )),
    }
}

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<ProductResponse>>, (StatusCode, Json<ErrorResponse>)> {
    list(State(state), Query(params)).await
}

#[derive(Debug, Deserialize)]
pub struct UpdateVerificationStatusRequest {
    pub verification_status: String,
    pub rejection_reason: Option<String>,
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

    diesel::update(products::table.find(id))
        .set((
            products::verification_status.eq(&payload.verification_status),
            products::rejection_reason.eq(&payload.rejection_reason),
        ))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update verification status: {}", e) }),
            )
        })?;

    let updated: Product = products::table
        .find(id)
        .select(Product::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated product".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Verification status updated successfully".to_string(),
        data: Some(ProductResponse::from(updated)),
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

pub async fn update_status(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateStatusRequest>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(products::table.find(id))
        .set(products::status.eq(&payload.status))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update status: {}", e) }),
            )
        })?;

    let updated: Product = products::table
        .find(id)
        .select(Product::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated product".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Status updated successfully".to_string(),
        data: Some(ProductResponse::from(updated)),
    }))
}
