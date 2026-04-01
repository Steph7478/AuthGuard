mod auth;
mod config;
mod handler;
mod middleware;
mod observability;
mod policy;
mod routes;
mod services;
mod utils;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Arc::new(config::load_config());
    let redis = Arc::new(services::RedisService::new(&cfg.redis_url));
    let policy = Arc::new(policy::Policy::new(redis.clone()));

    let app = Router::new()
        .merge(routes::public_routes(cfg.clone()))
        .merge(routes::private_routes(cfg.clone(), redis, policy));

    let port = cfg.port.clone();
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    println!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
