use axum::{
    extract::{Path, Query, State},
    Json,
};
use diesel::prelude::*;
use crate::models::{Product, Category};
use crate::schema::{products, categories};
use crate::utils::api_response::ApiResponse;
use crate::utils::types::AppState;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProductFilter {
    pub category_slug: Option<String>,
}

pub async fn get_products(
    State(state): State<AppState>,
    Query(filter): Query<ProductFilter>,
) -> Json<ApiResponse<Vec<Product>>> {
    let mut conn = state.db_pool.get().expect("Failed to get db connection");

    let mut query = products::table.into_boxed();

    if let Some(slug) = filter.category_slug {
        if let Ok(cat) = categories::table
            .filter(categories::slug.eq(slug))
            .first::<Category>(&mut conn) 
        {
             query = query.filter(products::category_id.eq(cat.id));
        }
    }

    // Only active products
    query = query.filter(products::status.eq("active"));

    let items = query
        .limit(20)
        .load::<Product>(&mut conn)
        .unwrap_or_default();

    Json(ApiResponse::success(items))
}

pub async fn get_product_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Json<ApiResponse<Product>> {
    let mut conn = state.db_pool.get().expect("Failed to get db connection");

    match products::table
        .filter(products::slug.eq(slug))
        .filter(products::status.eq("active"))
        .first::<Product>(&mut conn)
    {
        Ok(product) => Json(ApiResponse::success(product)),
        Err(_) => Json(ApiResponse::error("Product not found".to_string())),
    }
}
