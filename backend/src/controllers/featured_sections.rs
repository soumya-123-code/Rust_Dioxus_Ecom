use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::featured_sections;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = featured_sections)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct FeaturedSection {
    pub id: u64,
    pub uuid: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub section_type: String,
    pub layout_type: Option<String>,
    pub product_filter: Option<String>,
    pub product_limit: Option<i32>,
    pub sort_order: Option<i32>,
    pub status: String,
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<FeaturedSection>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let results: Vec<FeaturedSection> = featured_sections::table
        .select(FeaturedSection::as_select())
        .order(featured_sections::sort_order.asc())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch featured sections".to_string() }),
            )
        })?;

    Ok(Json(results))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<FeaturedSection>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let section: Option<FeaturedSection> = featured_sections::table
        .find(id)
        .select(FeaturedSection::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch section".to_string() }),
            )
        })?;

    section.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Section not found".to_string() }),
        )
    }).map(Json)
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = featured_sections)]
pub struct CreateSectionRequest {
    pub uuid: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub section_type: String,
    pub layout_type: Option<String>,
    pub product_filter: Option<String>,
    pub product_limit: Option<i32>,
    pub sort_order: Option<i32>,
    pub status: String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateSectionRequest>,
) -> Result<(StatusCode, Json<FeaturedSection>), (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::insert_into(featured_sections::table)
        .values(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to create section: {}", e) }),
            )
        })?;

    let created: FeaturedSection = featured_sections::table
        .order(featured_sections::id.desc())
        .select(FeaturedSection::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve created section".to_string() }),
            )
        })?;

    Ok((StatusCode::CREATED, Json(created)))
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = featured_sections)]
pub struct UpdateSectionRequest {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub section_type: Option<String>,
    pub layout_type: Option<String>,
    pub product_filter: Option<String>,
    pub product_limit: Option<i32>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateSectionRequest>,
) -> Result<Json<FeaturedSection>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(featured_sections::table.find(id))
        .set(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update section: {}", e) }),
            )
        })?;

    let updated: FeaturedSection = featured_sections::table
        .find(id)
        .select(FeaturedSection::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated section".to_string() }),
            )
        })?;

    Ok(Json(updated))
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

    diesel::delete(featured_sections::table.find(id))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete section".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
