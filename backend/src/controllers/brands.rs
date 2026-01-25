use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{Brand, NewBrand, UpdateBrand, BrandResponse};
use crate::schema::brands;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<BrandResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
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
) -> Result<Json<Vec<BrandResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = brands::table.into_boxed();

    if let Some(status) = params.status {
        query = query.filter(brands::status.eq(status));
    }

    if let Some(search) = params.search {
        query = query.filter(brands::title.like(format!("%{}%", search)));
    }

    let results: Vec<Brand> = query
        .order(brands::created_at.desc())
        .select(Brand::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch brands".to_string() }),
            )
        })?;

    let response: Vec<BrandResponse> = results.into_iter().map(BrandResponse::from).collect();

    Ok(Json(response))
}

pub async fn datatable(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<BrandResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let total: i64 = brands::table
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<Brand> = brands::table
        .order(brands::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .select(Brand::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch brands".to_string() }),
            )
        })?;

    let data: Vec<BrandResponse> = results.into_iter().map(BrandResponse::from).collect();

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

    let brand: Option<Brand> = brands::table
        .find(id)
        .select(Brand::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch brand".to_string() }),
            )
        })?;

    match brand {
        Some(b) => Ok(Json(SuccessResponse {
            success: true,
            message: "Brand retrieved successfully".to_string(),
            data: Some(BrandResponse::from(b)),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Brand not found".to_string() }),
        )),
    }
}

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<BrandResponse>>, (StatusCode, Json<ErrorResponse>)> {
    list(State(state), Query(params)).await
}

#[derive(Debug, Deserialize)]
pub struct CreateBrandRequest {
    pub title: String,
    pub description: String,
    pub image: String,
    pub banner: Option<String>,
    pub status: Option<String>,
    pub is_featured: Option<bool>,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateBrandRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let slug = generate_slug(&payload.title);
    let uuid = Uuid::new_v4().to_string();

    let new_brand = NewBrand {
        uuid,
        slug,
        title: payload.title,
        description: payload.description,
        image: payload.image,
        banner: payload.banner,
        status: payload.status.unwrap_or_else(|| "1".to_string()),
        is_featured: payload.is_featured,
        metadata: serde_json::json!({}),
    };

    diesel::insert_into(brands::table)
        .values(&new_brand)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to create brand: {}", e) }),
            )
        })?;

    let created: Brand = brands::table
        .order(brands::id.desc())
        .select(Brand::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve created brand".to_string() }),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse {
            success: true,
            message: "Brand created successfully".to_string(),
            data: Some(BrandResponse::from(created)),
        }),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateBrand>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(brands::table.find(id))
        .set(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update brand: {}", e) }),
            )
        })?;

    let updated: Brand = brands::table
        .find(id)
        .select(Brand::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated brand".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Brand updated successfully".to_string(),
        data: Some(BrandResponse::from(updated)),
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

    diesel::delete(brands::table.find(id))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete brand".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}

fn generate_slug(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}
