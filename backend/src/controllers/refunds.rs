use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{order_items, orders, users};
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct RefundRequestResponse {
    pub id: u64,
    pub order_id: u64,
    pub order_number: String,
    pub customer_name: String,
    pub product_name: String,
    pub amount: rust_decimal::Decimal,
    pub status: String,
    pub date: String,
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
) -> Result<Json<PaginatedResponse<RefundRequestResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    // Count query
    let mut count_query = order_items::table
        .inner_join(orders::table.on(order_items::order_id.eq(orders::id)))
        .inner_join(users::table.on(orders::user_id.eq(users::id)))
        .filter(order_items::return_status.is_not_null())
        .into_boxed();

    if let Some(ref status) = params.status {
        count_query = count_query.filter(order_items::return_status.eq(status));
    }

    let total: i64 = count_query
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    // Data query
    let mut data_query = order_items::table
        .inner_join(orders::table.on(order_items::order_id.eq(orders::id)))
        .inner_join(users::table.on(orders::user_id.eq(users::id)))
        .filter(order_items::return_status.is_not_null())
        .select((
            order_items::id,
            orders::id,
            orders::slug,
            users::name,
            order_items::title,
            order_items::subtotal,
            order_items::return_status,
            order_items::created_at,
        ))
        .into_boxed();

    if let Some(ref status) = params.status {
        data_query = data_query.filter(order_items::return_status.eq(status));
    }

    let results = data_query
        .limit(per_page)
        .offset(offset)
        .order(order_items::created_at.desc())
        .load::<(u64, u64, String, String, String, rust_decimal::Decimal, Option<String>, Option<chrono::NaiveDateTime>)>(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: format!("Failed to fetch refund requests: {}", e) }),
            )
        })?;

    let data: Vec<RefundRequestResponse> = results.into_iter().map(|(id, order_id, order_number, customer_name, product_name, amount, status, date)| {
        RefundRequestResponse {
            id,
            order_id,
            order_number,
            customer_name,
            product_name,
            amount,
            status: status.unwrap_or_default(),
            date: date.map(|d| d.to_string()).unwrap_or_default(),
        }
    }).collect();

    Ok(Json(PaginatedResponse {
        data,
        total,
        page,
        per_page,
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

    diesel::update(order_items::table.find(id))
        .set(order_items::return_status.eq(payload.status))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: format!("Failed to update status: {}", e) }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Refund status updated successfully".to_string(),
    }))
}
