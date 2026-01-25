use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{DeliveryZone, NewDeliveryZone, UpdateDeliveryZone, DeliveryZoneResponse};
use crate::schema::delivery_zones;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<DeliveryZoneResponse>,
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
) -> Result<Json<Vec<DeliveryZoneResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = delivery_zones::table.into_boxed();

    if let Some(status) = params.status {
        query = query.filter(delivery_zones::status.eq(status));
    }

    if let Some(search) = params.search {
        query = query.filter(delivery_zones::name.like(format!("%{}%", search)));
    }

    let results: Vec<DeliveryZone> = query
        .order(delivery_zones::created_at.desc())
        .select(DeliveryZone::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch delivery zones".to_string() }),
            )
        })?;

    let response: Vec<DeliveryZoneResponse> = results.into_iter().map(DeliveryZoneResponse::from).collect();

    Ok(Json(response))
}

pub async fn datatable(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<DeliveryZoneResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let total: i64 = delivery_zones::table
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<DeliveryZone> = delivery_zones::table
        .order(delivery_zones::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .select(DeliveryZone::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch delivery zones".to_string() }),
            )
        })?;

    let data: Vec<DeliveryZoneResponse> = results.into_iter().map(DeliveryZoneResponse::from).collect();

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

    let zone: Option<DeliveryZone> = delivery_zones::table
        .find(id)
        .select(DeliveryZone::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch delivery zone".to_string() }),
            )
        })?;

    match zone {
        Some(z) => Ok(Json(SuccessResponse {
            success: true,
            message: "Delivery zone retrieved successfully".to_string(),
            data: Some(DeliveryZoneResponse::from(z)),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Delivery zone not found".to_string() }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateDeliveryZoneRequest {
    pub name: String,
    pub center_latitude: rust_decimal::Decimal,
    pub center_longitude: rust_decimal::Decimal,
    pub radius_km: f64,
    pub status: Option<String>,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateDeliveryZoneRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let slug = generate_slug(&payload.name);

    let new_zone = NewDeliveryZone {
        name: payload.name,
        slug,
        center_latitude: payload.center_latitude,
        center_longitude: payload.center_longitude,
        radius_km: payload.radius_km,
        boundary_json: None,
        status: payload.status.unwrap_or_else(|| "active".to_string()),
        delivery_time_per_km: None,
        buffer_time: None,
        min_order_amount: None,
        delivery_charge_type: None,
        base_delivery_charge: None,
        delivery_charge_per_km: None,
        free_delivery_above: None,
        per_order_earning: None,
        per_km_earning: None,
        base_earning: None,
    };

    diesel::insert_into(delivery_zones::table)
        .values(&new_zone)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to create delivery zone: {}", e) }),
            )
        })?;

    let created: DeliveryZone = delivery_zones::table
        .order(delivery_zones::id.desc())
        .select(DeliveryZone::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve created zone".to_string() }),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse {
            success: true,
            message: "Delivery zone created successfully".to_string(),
            data: Some(DeliveryZoneResponse::from(created)),
        }),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateDeliveryZone>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(delivery_zones::table.find(id))
        .set(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update delivery zone: {}", e) }),
            )
        })?;

    let updated: DeliveryZone = delivery_zones::table
        .find(id)
        .select(DeliveryZone::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated zone".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Delivery zone updated successfully".to_string(),
        data: Some(DeliveryZoneResponse::from(updated)),
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

    diesel::delete(delivery_zones::table.find(id))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete delivery zone".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}

fn generate_slug(name: &str) -> String {
    name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}
