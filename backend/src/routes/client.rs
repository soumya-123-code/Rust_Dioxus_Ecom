use axum::{routing::{get, post}, Router};
use crate::controllers::client;
use crate::utils::types::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(client::auth::login))
        .route("/register", post(client::auth::register))
        .route("/user/cart", get(client::cart::get_cart))
        .route("/user/cart/add", post(client::cart::add_to_cart))
        .route("/products", get(client::products::get_products))
        .route("/products/:slug", get(client::products::get_product_detail))
        .route("/categories", get(client::categories::get_categories))
        .route("/banners", get(client::banners::get_banners))
        .route("/delivery-zone/products", get(client::products::get_products))
}
