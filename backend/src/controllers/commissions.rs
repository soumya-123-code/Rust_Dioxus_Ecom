use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::schema::order_items;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct CommissionItem {
    pub id: u64,
    pub order_id: u64,
    pub product_title: String,
    pub admin_commission_amount: Decimal,
    pub seller_commission_amount: Decimal,
    pub commission_settled: String,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub settled: Option<String>,
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
) -> Result<Json<PaginatedResponse<CommissionItem>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let settled_filter = params.settled.unwrap_or_else(|| "0".to_string());

    let total: i64 = order_items::table
        .filter(order_items::commission_settled.eq(&settled_filter))
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<(u64, u64, String, Decimal, Decimal, String)> = order_items::table
        .select((
            order_items::id,
            order_items::order_id,
            order_items::title,
            order_items::admin_commission_amount,
            order_items::seller_commission_amount,
            order_items::commission_settled,
        ))
        .filter(order_items::commission_settled.eq(&settled_filter))
        .order(order_items::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch commissions".to_string() }),
            )
        })?;

    let data: Vec<CommissionItem> = results
        .into_iter()
        .map(|(id, order_id, title, admin_amt, seller_amt, settled)| CommissionItem {
            id,
            order_id,
            product_title: title,
            admin_commission_amount: admin_amt,
            seller_commission_amount: seller_amt,
            commission_settled: settled,
        })
        .collect();

    Ok(Json(PaginatedResponse {
        data,
        total,
        page,
        per_page,
    }))
}

pub async fn settle(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(order_items::table.find(id))
        .set(order_items::commission_settled.eq("1"))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to settle commission: {}", e) }),
            )
        })?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Commission settled successfully"
    })))
}
