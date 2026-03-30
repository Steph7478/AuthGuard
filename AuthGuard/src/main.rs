use axum::{
    extract::State,
    middleware::from_fn_with_state,
    routing::{any, get},
    Router,
};
use std::sync::Arc;

mod auth;
mod config;
mod middleware;
mod observability;
mod proxy;
mod services;

use services::redis::RedisService;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<config::AppConfig>,
    pub policy: Arc<config::Policy>,
    pub redis: Arc<RedisService>,
}

#[tokio::main]
async fn main() {
    let (config, policy) = config::load_config();
    let redis_service = Arc::new(RedisService::new(&config.redis_url));

    let port = config.port.clone();

    let state = AppState {
        config: Arc::new(config),
        policy: Arc::new(policy),
        redis: redis_service,
    };

    let app = Router::new()
        .route("/metrics", get(observability::metrics::metrics_handler))
        .route("/*path", any(proxy::handler::proxy_handler))
        .layer(from_fn_with_state(
            state.clone(),
            |State(s): State<AppState>, req, next| async move {
                middleware::security::security_layer(req, next, s.config, s.policy, s.redis).await
            },
        ))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
