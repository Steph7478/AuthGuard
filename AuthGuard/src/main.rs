mod auth;
mod config;
mod handler;
mod middleware;
mod observability;
mod routes;
mod services;
mod state;
mod utils;

use crate::state::AppState;
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(config::load_config());
    let state = Arc::new(AppState {
        config: config.clone(),
        redis: Arc::new(services::RedisService::new(&config.redis_url)),
    });

    let app = Router::new()
        .merge(routes::public_routes())
        .merge(routes::private_routes())
        .layer(Extension(state.clone()))
        .with_state(state.clone());

    let addr: SocketAddr = format!("0.0.0.0:{}", state.config.port).parse()?;
    println!("Server listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service(),
    )
    .await?;
    Ok(())
}