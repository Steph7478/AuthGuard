use crate::config::AppConfig;
use crate::handler::callback_handler;
use crate::middleware::auth_middleware;
use crate::observability;
use crate::policy::Policy;
use crate::services::RedisService;
use axum::{middleware, routing::get, Router};
use std::sync::Arc;

pub fn public_routes(config: Arc<AppConfig>) -> Router {
    Router::new()
        .route("/callback", get(callback_handler))
        .route("/health", get(health_check))
        .route("/metrics", get(observability::handler))
        .with_state(config)
}

pub fn private_routes(
    config: Arc<AppConfig>,
    redis: Arc<RedisService>,
    policy: Arc<Policy>,
) -> Router {
    Router::new()
        .route("/validate", get(validate_handler))
        .layer(middleware::from_fn(move |req, next| {
            let config = config.clone();
            let redis = redis.clone();
            let policy = policy.clone();
            async move { auth_middleware(req, next, config, redis, policy).await }
        }))
}

async fn validate_handler() -> &'static str {
    "OK"
}

async fn health_check() -> &'static str {
    "OK"
}
