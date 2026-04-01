use crate::auth;
use crate::config::AppConfig;
use crate::policy::Policy;
use crate::services::RedisService;
use crate::utils::{extract_ip, extract_token};
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use http::StatusCode;
use std::sync::Arc;

pub async fn auth_middleware(
    req: Request<Body>,
    next: Next,
    config: Arc<AppConfig>,
    redis: Arc<RedisService>,
    policy: Arc<Policy>,
) -> Result<Response, StatusCode> {
    let ip = extract_ip(&req);

    if !policy.check_rate_limit(&ip).await {
        return Err(StatusCode::FORBIDDEN);
    }

    let token = extract_token(&req).ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = auth::verify(token, &config.jwks_url, &config.jwt_issuer, &redis)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut req = req;
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
