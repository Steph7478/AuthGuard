use crate::{auth, config::AppConfig, policy::Policy, services::RedisService, utils};
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use http::StatusCode;
use std::sync::Arc;

pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
    config: Arc<AppConfig>,
    redis: Arc<RedisService>,
    policy: Arc<Policy>,
) -> Result<Response, StatusCode> {
    let ip = utils::extract_ip(&req);
    
    if !policy.check_rate_limit(&ip).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    let token = utils::extract_token(&req).ok_or(StatusCode::UNAUTHORIZED)?;
    
    let claims = auth::verify(token, &config.jwks_url, &config.jwt_issuer, &redis)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}