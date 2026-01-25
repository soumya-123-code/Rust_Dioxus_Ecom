use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WithdrawalItem {
    pub id: u64,
    pub amount: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

pub async fn seller_list(
    State(_state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<WithdrawalItem>>, (StatusCode, Json<ErrorResponse>)> {
    // Implement seller withdrawal list query
    // This would require a seller_withdrawals table
    Ok(Json(PaginatedResponse {
        data: vec![],
        total: 0,
        page: params.page.unwrap_or(1),
        per_page: params.per_page.unwrap_or(10),
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateWithdrawalRequest {
    pub status: String,
    pub notes: Option<String>,
}

pub async fn update_seller_withdrawal(
    State(_state): State<AppState>,
    Path(_id): Path<u64>,
    Json(_payload): Json<UpdateWithdrawalRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // Implement seller withdrawal update
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Withdrawal updated successfully"
    })))
}

pub async fn delivery_boy_list(
    State(_state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<WithdrawalItem>>, (StatusCode, Json<ErrorResponse>)> {
    // Implement delivery boy withdrawal list query
    Ok(Json(PaginatedResponse {
        data: vec![],
        total: 0,
        page: params.page.unwrap_or(1),
        per_page: params.per_page.unwrap_or(10),
    }))
}

pub async fn update_delivery_boy_withdrawal(
    State(_state): State<AppState>,
    Path(_id): Path<u64>,
    Json(_payload): Json<UpdateWithdrawalRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // Implement delivery boy withdrawal update
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Withdrawal updated successfully"
    })))
}
