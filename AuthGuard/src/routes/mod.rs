use crate::{
    handler::callback_handler, middleware::auth_middleware, observability, state::AppState,
};
use axum::{middleware::from_fn, routing::get, Router};
use std::sync::Arc;

pub fn public_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/callback", get(callback_handler))
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get(observability::handler))
}

pub fn private_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/validate", get(|| async { "OK" }))
        .layer(from_fn(auth_middleware))
}
