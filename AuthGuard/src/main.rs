use authguard::config;
use authguard::middleware::auth_middleware;
use authguard::observability;
use authguard::policy::Policy;
use authguard::services::RedisService;
use authguard::state::AppState;
use axum::{middleware, routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(config::load_config());
    let redis = Arc::new(RedisService::new(&config.redis_url));
    let policy = Arc::new(Policy::new(redis.clone()));

    let state = Arc::new(AppState {
        config: config.clone(),
        redis,
        policy,
    });

    let protected_routes = Router::new()
        .route("/validate", get(validate_handler))
        .layer(middleware::from_fn({
            let state = state.clone();
            move |req, next| {
                let state = state.clone();
                async move { auth_middleware(req, next, state).await }
            }
        }));

    let app = Router::new()
        .merge(protected_routes)
        .route("/health", get(health_check))
        .route("/metrics", get(observability::handler))
        .with_state(state);

    let addr: SocketAddr = format!("0.0.0.0:{}", config.port).parse()?;
    println!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn validate_handler() -> &'static str {
    "OK"
}

async fn health_check() -> &'static str {
    "OK"
}
