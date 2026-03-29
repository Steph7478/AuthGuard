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
use std::time::Instant;

mod auth;
mod config;
mod middleware;
mod observability;
mod proxy;
mod services;

use services::redis::RedisService;

#[derive(Clone)]
struct AppState {
    config: Arc<config::config::AppConfig>,
    policy: Arc<config::config::Policy>,
    redis: Arc<RedisService>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (config, policy) = config::config::load_config();

    let state = AppState {
        config: Arc::new(config.clone()),
        policy: Arc::new(policy),
        redis: Arc::new(RedisService::new(&config.redis_url)),
    };

    let app = Router::new()
        .route("/metrics", get(observability::metrics::metrics_handler))
        .route("/*path", any(proxy_handler))
        .layer(from_fn_with_state(
            state.clone(),
            |State(s): State<AppState>, r, n| async move {
                middleware::security::security_layer(r, n, s.config, s.policy, s.redis).await
            },
        ))
        .with_state(state.clone());

    let addr = format!("0.0.0.0:{}", state.config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("🚀 AuthGuard running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn proxy_handler(State(state): State<AppState>, req: Request<Body>) -> impl IntoResponse {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let start = Instant::now();

    let response = proxy::proxy::forward(req, &state.config.target_service).await;
    
    let status = response.status().as_u16().to_string();

    observability::metrics::record(&method, &path, &status);
    observability::metrics::record_duration(&method, &path, start);

    response
}