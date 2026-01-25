use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{Banner, NewBanner, UpdateBanner, BannerResponse};
use crate::schema::banners;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<BannerResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub position: Option<String>,
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
) -> Result<Json<Vec<BannerResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = banners::table.into_boxed();

    if let Some(status) = params.status {
        query = query.filter(banners::status.eq(status));
    }

    if let Some(position) = params.position {
        query = query.filter(banners::position.eq(position));
    }

    let results: Vec<Banner> = query
        .order(banners::sort_order.asc())
        .select(Banner::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch banners".to_string() }),
            )
        })?;

    let response: Vec<BannerResponse> = results.into_iter().map(BannerResponse::from).collect();

    Ok(Json(response))
}

pub async fn datatable(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<BannerResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let total: i64 = banners::table
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<Banner> = banners::table
        .order(banners::sort_order.asc())
        .limit(per_page)
        .offset(offset)
        .select(Banner::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch banners".to_string() }),
            )
        })?;

    let data: Vec<BannerResponse> = results.into_iter().map(BannerResponse::from).collect();

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

    let banner: Option<Banner> = banners::table
        .find(id)
        .select(Banner::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch banner".to_string() }),
            )
        })?;

    match banner {
        Some(b) => Ok(Json(SuccessResponse {
            success: true,
            message: "Banner retrieved successfully".to_string(),
            data: Some(BannerResponse::from(b)),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Banner not found".to_string() }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateBannerRequest {
    pub title: String,
    pub image: String,
    pub link_type: Option<String>,
    pub link_value: Option<String>,
    pub position: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateBannerRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let new_banner = NewBanner {
        title: payload.title,
        image: payload.image,
        link_type: payload.link_type,
        link_value: payload.link_value,
        position: payload.position,
        sort_order: payload.sort_order,
        status: payload.status.unwrap_or_else(|| "active".to_string()),
        start_date: None,
        end_date: None,
    };

    diesel::insert_into(banners::table)
        .values(&new_banner)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to create banner: {}", e) }),
            )
        })?;

    let created: Banner = banners::table
        .order(banners::id.desc())
        .select(Banner::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve created banner".to_string() }),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse {
            success: true,
            message: "Banner created successfully".to_string(),
            data: Some(BannerResponse::from(created)),
        }),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateBanner>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(banners::table.find(id))
        .set(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update banner: {}", e) }),
            )
        })?;

    let updated: Banner = banners::table
        .find(id)
        .select(Banner::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated banner".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Banner updated successfully".to_string(),
        data: Some(BannerResponse::from(updated)),
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

    diesel::delete(banners::table.find(id))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete banner".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
