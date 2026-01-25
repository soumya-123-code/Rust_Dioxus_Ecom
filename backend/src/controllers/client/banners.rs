use axum::{
    extract::{Query, State},
    Json,
};
use diesel::prelude::*;
use crate::models::{Banner, BannerResponse};
use crate::schema::banners;
use crate::utils::api_response::ApiResponse;
use crate::utils::types::AppState;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BannerFilter {
    pub category_slug: Option<String>, // We might not filter banners by category yet, but keeping structure
}

pub async fn get_banners(
    State(state): State<AppState>,
    Query(_filter): Query<BannerFilter>,
) -> Json<ApiResponse<Vec<BannerResponse>>> {
    let mut conn = state.db_pool.get().expect("Failed to get db connection");

    let items = banners::table
        .filter(banners::status.eq("active"))
        .load::<Banner>(&mut conn)
        .unwrap_or_default();

    let response: Vec<BannerResponse> = items.into_iter().map(BannerResponse::from).collect();

    Json(ApiResponse::success(response))
}
