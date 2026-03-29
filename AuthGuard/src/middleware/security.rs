use crate::config::config::{AppConfig, Policy};
use crate::services::redis::RedisService;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn security_layer(
    mut req: Request<Body>,
    next: Next,
    config: Arc<AppConfig>,
    policy: Arc<Policy>,
    redis: Arc<RedisService>,
) -> Result<Response, StatusCode> {
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    if policy.block_ips.contains(&ip) {
        return Err(StatusCode::FORBIDDEN);
    }

    if policy.require_auth {
        let auth_header = req
            .headers()
            .get("authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .replace("Bearer ", "");

        let claims = crate::auth::jwt::verify(&token, &config.jwks_url, &redis)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        req.extensions_mut().insert(claims);
    }

    crate::middleware::ratelimit::check(&ip, &config, &redis).await?;

    Ok(next.run(req).await)
}
