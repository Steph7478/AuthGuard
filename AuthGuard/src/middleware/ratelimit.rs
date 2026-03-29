use crate::{config::AppConfig, services::redis::RedisService};
use axum::http::StatusCode;

pub async fn check(
    ip: &str,
    config: &AppConfig,
    redis: &RedisService,
) -> Result<(), StatusCode> {

    let key = format!("rate_limit:{}", ip);

    let count = redis
        .incr_with_expire(&key, config.window)
        .await;

    if count > config.rate_limit {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(())
}