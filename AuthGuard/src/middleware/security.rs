use crate::{
    auth::jwt::verify,
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

const IP_HEADER: &str = "x-forwarded-for";
const AUTH_HEADER: &str = "authorization";
const BEARER_PREFIX: &str = "Bearer ";

pub async fn security_layer(
    mut req: Request<Body>,
    next: Next,
    config: Arc<AppConfig>,
    policy: Arc<Policy>,
    redis: Arc<RedisService>,
) -> Result<Response, StatusCode> {
    let ip = extract_ip(&req);

    if policy.block_ips.contains(&ip) {
        return Err(StatusCode::FORBIDDEN);
    }

    if policy.require_auth {
        let claims = authenticate(&mut req, &config.jwks_url, &redis).await?;
        req.extensions_mut().insert(claims);
    }

    crate::middleware::ratelimit::check(&ip, &config, &redis).await?;
    Ok(next.run(req).await)
}

fn extract_ip(req: &Request<Body>) -> String {
    req.headers()
        .get(IP_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

async fn authenticate(
    req: &mut Request<Body>,
    jwks_url: &str,
    redis: &RedisService,
) -> Result<crate::auth::jwt::KeycloakClaims, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTH_HEADER)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .strip_prefix(BEARER_PREFIX)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    verify(token, jwks_url, redis)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)
}
