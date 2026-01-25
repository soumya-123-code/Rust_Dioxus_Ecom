use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::tax_classes;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = tax_classes)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TaxClass {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_default: bool,
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<TaxClass>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let results: Vec<TaxClass> = tax_classes::table
        .select(TaxClass::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch tax classes".to_string() }),
            )
        })?;

    Ok(Json(results))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<TaxClass>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let tax_class: Option<TaxClass> = tax_classes::table
        .find(id)
        .select(TaxClass::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch tax class".to_string() }),
            )
        })?;

    tax_class.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Tax class not found".to_string() }),
        )
    }).map(Json)
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = tax_classes)]
pub struct CreateTaxClassRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_default: bool,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaxClassRequest>,
) -> Result<(StatusCode, Json<TaxClass>), (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::insert_into(tax_classes::table)
        .values(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to create tax class: {}", e) }),
            )
        })?;

    let created: TaxClass = tax_classes::table
        .order(tax_classes::id.desc())
        .select(TaxClass::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve created tax class".to_string() }),
            )
        })?;

    Ok((StatusCode::CREATED, Json(created)))
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = tax_classes)]
pub struct UpdateTaxClassRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_default: Option<bool>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTaxClassRequest>,
) -> Result<Json<TaxClass>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(tax_classes::table.find(id))
        .set(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update tax class: {}", e) }),
            )
        })?;

    let updated: TaxClass = tax_classes::table
        .find(id)
        .select(TaxClass::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated tax class".to_string() }),
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

    diesel::delete(tax_classes::table.find(id))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete tax class".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
