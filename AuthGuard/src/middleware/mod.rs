use crate::{auth, state::AppState};
use axum::{
    body::Body,
    extract::{Extension, Request},
    middleware::Next,
    response::Response,
};
use http::StatusCode;
use std::sync::Arc;

pub fn extract_token(req: &Request<Body>) -> Option<&str> {
    req.headers()
        .get("Authorization")?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}

pub async fn auth_middleware(
    Extension(state): Extension<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token(&req).ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = auth::verify(
        token,
        &state.config.jwks_url,
        &state.config.jwt_issuer,
        &state.redis,
    )
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
