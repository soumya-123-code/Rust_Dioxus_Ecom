pub mod auth;
pub mod admin;
pub mod client;

use axum::{middleware, Router};
use crate::utils::types::AppState;
use crate::middlewares::auth::auth_middleware;

pub fn app_router(state: AppState) -> Router {
    // Protect all admin routes with authentication middleware
    let protected_routes = admin::routes()
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    let api_routes = Router::new()
        .merge(auth::routes())
        .merge(protected_routes);

    let client_routes = client::routes();

    Router::new()
        .nest("/api/admin", api_routes)
        .nest("/api/client", client_routes)
        .with_state(state)
}
