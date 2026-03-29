use axum::{http::Request, middleware::Next, response::Response, http::StatusCode};
use std::sync::Arc;

use crate::{
    config::{AppConfig, Policy},
    services::redis::RedisService,
};

pub async fn security_layer<B>(
    req: Request<B>,
    next: Next<B>,
    config: Arc<AppConfig>,
    policy: Arc<Policy>,
    redis: Arc<RedisService>,
) -> Result<Response, StatusCode> {

    let ip = req.headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    if policy.block_ips.contains(&ip.to_string()) {
        return Err(StatusCode::FORBIDDEN);
    }

    if policy.require_auth {
        if let Some(auth) = req.headers().get("authorization") {
            let token = auth.to_str().unwrap().replace("Bearer ", "");
            crate::auth::jwt::verify(&token, &config.jwks_url)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    crate::middleware::ratelimit::check(ip, &config, &redis).await?;

    Ok(next.run(req).await)
}