use axum::Server;
use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::from_fn_with_state,
    response::IntoResponse,
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

use config::{AppConfig, Policy};
use services::redis::RedisService;

#[derive(Clone)]
struct AppState {
    config: Arc<AppConfig>,
    policy: Arc<Policy>,
    redis: Arc<RedisService>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (config, policy) = config::load_config();
    let redis = RedisService::new(&config.redis_url);

    let state = AppState {
        config: Arc::new(config),
        policy: Arc::new(policy),
        redis: Arc::new(redis),
    };

    let app = Router::new()
        .route("/metrics", get(observability::metrics::metrics_handler))
        .route("/*path", any(proxy_handler))
        .layer(from_fn_with_state(
            state.clone(),
            |state: AppState, req, next| async move {
                middleware::security::security_layer(
                    req,
                    next,
                    state.config.clone(),
                    state.policy.clone(),
                    state.redis.clone(),
                )
                .await
            },
        ))
        .with_state(state.clone());

    let addr = format!("0.0.0.0:{}", state.config.port);
    println!("🚀 AuthGuard running on {}", addr);

    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn proxy_handler(State(state): State<AppState>, req: Request<Body>) -> impl IntoResponse {
    proxy::forward(req, &state.config.target_service).await
}
