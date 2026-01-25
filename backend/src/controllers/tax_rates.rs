use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::schema::tax_rates;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = tax_rates)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TaxRate {
    pub id: u64,
    pub name: String,
    pub rate: Decimal,
    pub country: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub priority: i32,
    pub compound: bool,
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<TaxRate>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let results: Vec<TaxRate> = tax_rates::table
        .order(tax_rates::priority.asc())
        .select(TaxRate::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch tax rates".to_string() }),
            )
        })?;

    Ok(Json(results))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<TaxRate>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let tax_rate: Option<TaxRate> = tax_rates::table
        .find(id)
        .select(TaxRate::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch tax rate".to_string() }),
            )
        })?;

    tax_rate.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Tax rate not found".to_string() }),
        )
    }).map(Json)
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = tax_rates)]
pub struct CreateTaxRateRequest {
    pub name: String,
    pub rate: Decimal,
    pub country: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub priority: i32,
    pub compound: bool,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaxRateRequest>,
) -> Result<(StatusCode, Json<TaxRate>), (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::insert_into(tax_rates::table)
        .values(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to create tax rate: {}", e) }),
            )
        })?;

    let created: TaxRate = tax_rates::table
        .order(tax_rates::id.desc())
        .select(TaxRate::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve created tax rate".to_string() }),
            )
        })?;

    Ok((StatusCode::CREATED, Json(created)))
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = tax_rates)]
pub struct UpdateTaxRateRequest {
    pub name: Option<String>,
    pub rate: Option<Decimal>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub priority: Option<i32>,
    pub compound: Option<bool>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTaxRateRequest>,
) -> Result<Json<TaxRate>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(tax_rates::table.find(id))
        .set(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update tax rate: {}", e) }),
            )
        })?;

    let updated: TaxRate = tax_rates::table
        .find(id)
        .select(TaxRate::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated tax rate".to_string() }),
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

    diesel::delete(tax_rates::table.find(id))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete tax rate".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
