use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{Promo, NewPromo, UpdatePromo, PromoResponse};
use crate::schema::promos;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<PromoResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
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
) -> Result<Json<Vec<PromoResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = promos::table.into_boxed();

    if let Some(status) = params.status {
        query = query.filter(promos::status.eq(status));
    }

    let results: Vec<Promo> = query
        .order(promos::created_at.desc())
        .select(Promo::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch promos".to_string() }),
            )
        })?;

    let response: Vec<PromoResponse> = results.into_iter().map(PromoResponse::from).collect();

    Ok(Json(response))
}

pub async fn datatable(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<PromoResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let total: i64 = promos::table
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<Promo> = promos::table
        .order(promos::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .select(Promo::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch promos".to_string() }),
            )
        })?;

    let data: Vec<PromoResponse> = results.into_iter().map(PromoResponse::from).collect();

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

    let promo: Option<Promo> = promos::table
        .find(id)
        .select(Promo::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch promo".to_string() }),
            )
        })?;

    match promo {
        Some(p) => Ok(Json(SuccessResponse {
            success: true,
            message: "Promo retrieved successfully".to_string(),
            data: Some(PromoResponse::from(p)),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Promo not found".to_string() }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePromoRequest {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: rust_decimal::Decimal,
    pub min_order_amount: Option<rust_decimal::Decimal>,
    pub max_discount_amount: Option<rust_decimal::Decimal>,
    pub usage_limit: Option<i32>,
    pub usage_limit_per_user: Option<i32>,
    pub status: Option<String>,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreatePromoRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let new_promo = NewPromo {
        code: payload.code.to_uppercase(),
        title: payload.title,
        description: payload.description,
        discount_type: payload.discount_type,
        discount_value: payload.discount_value,
        min_order_amount: payload.min_order_amount,
        max_discount_amount: payload.max_discount_amount,
        usage_limit: payload.usage_limit,
        usage_limit_per_user: payload.usage_limit_per_user,
        times_used: 0,
        start_date: None,
        end_date: None,
        status: payload.status.unwrap_or_else(|| "active".to_string()),
    };

    diesel::insert_into(promos::table)
        .values(&new_promo)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to create promo: {}", e) }),
            )
        })?;

    let created: Promo = promos::table
        .order(promos::id.desc())
        .select(Promo::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve created promo".to_string() }),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse {
            success: true,
            message: "Promo created successfully".to_string(),
            data: Some(PromoResponse::from(created)),
        }),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdatePromo>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(promos::table.find(id))
        .set(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update promo: {}", e) }),
            )
        })?;

    let updated: Promo = promos::table
        .find(id)
        .select(Promo::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated promo".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Promo updated successfully".to_string(),
        data: Some(PromoResponse::from(updated)),
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

    diesel::delete(promos::table.find(id))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete promo".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
