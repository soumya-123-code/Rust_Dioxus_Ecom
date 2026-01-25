use axum::{routing::{post, get}, Router};
use crate::controllers::auth;
use crate::utils::types::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/me", get(auth::me))
}
