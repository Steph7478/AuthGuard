use authguard::config;
use authguard::policy::Policy;
use authguard::routes::{private_routes, public_routes};
use authguard::services::RedisService;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(config::load_config());
    let redis = Arc::new(RedisService::new(&config.redis_url));
    let policy = Arc::new(Policy::new(redis.clone()));

    let app = Router::new()
        .merge(public_routes(config.clone()))
        .merge(private_routes(
            config.clone(),
            redis.clone(),
            policy.clone(),
        ));

    let addr: SocketAddr = format!("0.0.0.0:{}", config.port).parse()?;
    println!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
