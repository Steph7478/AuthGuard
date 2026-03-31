use crate::auth;
use crate::state::AppState;
use crate::utils::{extract_ip, extract_token};
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use http::StatusCode;
use std::sync::Arc;

pub async fn auth_middleware(
    req: Request<Body>,
    next: Next,
    state: Arc<AppState>,
) -> Result<Response, StatusCode> {
    let ip = extract_ip(&req);

    if !state.policy.check_rate_limit(&ip).await {
        return Err(StatusCode::FORBIDDEN);
    }

    let token = extract_token(&req).ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = auth::verify(
        token,
        &state.config.jwks_url,
        &state.config.jwt_issuer,
        &state.redis,
    )
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut req = req;
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
