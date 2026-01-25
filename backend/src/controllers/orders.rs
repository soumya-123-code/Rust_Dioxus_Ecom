use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{Order, UpdateOrder, OrderResponse, OrderItem, OrderItemResponse};
use crate::schema::{orders, order_items};
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct OrderDetailResponse {
    pub order: OrderResponse,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<OrderDetailResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub payment_status: Option<String>,
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
) -> Result<Json<Vec<OrderResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = orders::table.into_boxed();

    if let Some(status) = params.status {
        query = query.filter(orders::status.eq(status));
    }

    if let Some(payment_status) = params.payment_status {
        query = query.filter(orders::payment_status.eq(payment_status));
    }

    if let Some(search) = params.search {
        query = query.filter(orders::slug.like(format!("%{}%", search)));
    }

    let results: Vec<Order> = query
        .order(orders::created_at.desc())
        .select(Order::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch orders".to_string() }),
            )
        })?;

    let response: Vec<OrderResponse> = results.into_iter().map(OrderResponse::from).collect();

    Ok(Json(response))
}

pub async fn datatable(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<OrderResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let total: i64 = orders::table
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<Order> = orders::table
        .order(orders::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .select(Order::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch orders".to_string() }),
            )
        })?;

    let data: Vec<OrderResponse> = results.into_iter().map(OrderResponse::from).collect();

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

    let order: Option<Order> = orders::table
        .find(id)
        .select(Order::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch order".to_string() }),
            )
        })?;

    match order {
        Some(o) => {
            let items: Vec<OrderItem> = order_items::table
                .filter(order_items::order_id.eq(id))
                .select(OrderItem::as_select())
                .load(&mut conn)
                .unwrap_or_default();

            let items_response: Vec<OrderItemResponse> = items.into_iter().map(OrderItemResponse::from).collect();

            Ok(Json(SuccessResponse {
                success: true,
                message: "Order retrieved successfully".to_string(),
                data: Some(OrderDetailResponse {
                    order: OrderResponse::from(o),
                    items: items_response,
                }),
            }))
        },
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Order not found".to_string() }),
        )),
    }
}

pub async fn update_status(
    State(state): State<AppState>,
    Path((id, status)): Path<(u64, String)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(orders::table.find(id))
        .set(orders::status.eq(&status))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update order status: {}", e) }),
            )
        })?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order status updated successfully"
    })))
}
