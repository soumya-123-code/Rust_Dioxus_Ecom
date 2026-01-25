use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use diesel::dsl::count_star;
use serde::Serialize;
use rust_decimal::Decimal;

use crate::schema::{orders, products, sellers, users};
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub total_orders: i64,
    pub total_products: i64,
    pub total_sellers: i64,
    pub total_customers: i64,
    pub total_revenue: Decimal,
    pub pending_orders: i64,
    pub delivered_orders: i64,
    pub pending_seller_approvals: i64,
}

#[derive(Debug, Serialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub revenue: Vec<Decimal>,
    pub orders: Vec<i64>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<DashboardStats>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    // Count total orders
    let total_orders: i64 = orders::table
        .select(count_star())
        .first(&mut conn)
        .unwrap_or(0);

    // Count total products
    let total_products: i64 = products::table
        .filter(products::deleted_at.is_null())
        .select(count_star())
        .first(&mut conn)
        .unwrap_or(0);

    // Count total sellers
    let total_sellers: i64 = sellers::table
        .filter(sellers::deleted_at.is_null())
        .select(count_star())
        .first(&mut conn)
        .unwrap_or(0);

    // Count total customers
    let total_customers: i64 = users::table
        .filter(users::access_panel.eq("user").or(users::access_panel.is_null()))
        .filter(users::deleted_at.is_null())
        .select(count_star())
        .first(&mut conn)
        .unwrap_or(0);

    // Calculate total revenue
    let total_revenue: Option<Decimal> = orders::table
        .select(diesel::dsl::sum(orders::final_total))
        .first(&mut conn)
        .unwrap_or(None);

    // Count pending orders
    let pending_orders: i64 = orders::table
        .filter(orders::status.eq("pending"))
        .select(count_star())
        .first(&mut conn)
        .unwrap_or(0);

    // Count delivered orders
    let delivered_orders: i64 = orders::table
        .filter(orders::status.eq("delivered"))
        .select(count_star())
        .first(&mut conn)
        .unwrap_or(0);

    // Count pending seller approvals
    let pending_seller_approvals: i64 = sellers::table
        .filter(sellers::verification_status.eq("not_approved"))
        .filter(sellers::deleted_at.is_null())
        .select(count_star())
        .first(&mut conn)
        .unwrap_or(0);

    Ok(Json(DashboardStats {
        total_orders,
        total_products,
        total_sellers,
        total_customers,
        total_revenue: total_revenue.unwrap_or(Decimal::ZERO),
        pending_orders,
        delivered_orders,
        pending_seller_approvals,
    }))
}

pub async fn get_chart_data(
    State(state): State<AppState>,
) -> Result<Json<ChartData>, (StatusCode, Json<ErrorResponse>)> {
    // This is a simplified version - implement proper date-based grouping
    let labels = vec![
        "Day 1".to_string(),
        "Day 2".to_string(),
        "Day 3".to_string(),
        "Day 4".to_string(),
        "Day 5".to_string(),
        "Day 6".to_string(),
        "Day 7".to_string(),
    ];

    let revenue = vec![
        Decimal::from(1000),
        Decimal::from(1500),
        Decimal::from(1200),
        Decimal::from(1800),
        Decimal::from(2000),
        Decimal::from(1700),
        Decimal::from(2200),
    ];

    let orders = vec![10, 15, 12, 18, 20, 17, 22];

    Ok(Json(ChartData {
        labels,
        revenue,
        orders,
    }))
}
