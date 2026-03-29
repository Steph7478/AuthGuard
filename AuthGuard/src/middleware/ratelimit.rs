use crate::config::config::AppConfig;
use crate::services::redis::RedisService;
use axum::http::StatusCode;

pub async fn check(ip: &str, config: &AppConfig, redis: &RedisService) -> Result<(), StatusCode> {
    let rate_key = format!("rate_limit:{}", ip);
    let block_key = format!("ublock:{}", ip);

    if redis.is_blocked(&block_key).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    let count = redis.incr_with_expire(&rate_key, config.window).await;

    if count > config.rate_limit {
        redis.block_ip(&block_key, config.window * 2).await; 
        
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(())
}