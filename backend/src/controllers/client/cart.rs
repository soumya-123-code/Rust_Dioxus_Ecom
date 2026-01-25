use axum::{Json, http::StatusCode};

pub async fn get_cart() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "data": [], // Empty cart
            "message": "Cart retrieved successfully"
        })),
    )
}

pub async fn add_to_cart() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "message": "Item added to cart"
        })),
    )
}
