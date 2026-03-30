use crate::config::AppConfig;
use crate::services::redis::RedisService;
use axum::http::StatusCode;

pub async fn check(ip: &str, config: &AppConfig, redis: &RedisService) -> Result<(), StatusCode> {
    let key = format!("ratelimit:{}", ip);
    let count = redis.incr_with_expire(&key, config.window).await;

    if count > config.rate_limit as i32 {
        redis.block_ip(&key, config.window).await;
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(())
}
