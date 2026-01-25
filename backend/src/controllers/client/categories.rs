use axum::{
    extract::State,
    Json,
};
use diesel::prelude::*;
use crate::models::{Category, CategoryResponse};
use crate::schema::categories;
use crate::utils::api_response::ApiResponse;
use crate::utils::types::AppState;

pub async fn get_categories(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<CategoryResponse>>> {
    let mut conn = state.db_pool.get().expect("Failed to get db connection");

    let items = categories::table
        .filter(categories::status.eq("active"))
        .load::<Category>(&mut conn)
        .unwrap_or_default();

    let response: Vec<CategoryResponse> = items.into_iter().map(CategoryResponse::from).collect();

    Json(ApiResponse::success(response))
}
