use crate::{
    config::{AppConfig, Policy},
    services::redis::RedisService,
};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn security_layer(
    req: Request<Body>,
    next: Next,
    config: Arc<AppConfig>,
    policy: Arc<Policy>,
    redis: Arc<RedisService>,
) -> Result<Response, StatusCode> {
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    if policy.block_ips.contains(&ip.to_string()) {
        return Err(StatusCode::FORBIDDEN);
    }

    if policy.require_auth {
        let auth = req
            .headers()
            .get("authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?;
        let token = auth.to_str().unwrap().replace("Bearer ", "");
        crate::auth::jwt::verify(&token, &config.jwks_url)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
    }

    crate::middleware::ratelimit::check(ip, &config, &redis).await?;
    Ok(next.run(req).await)
}
