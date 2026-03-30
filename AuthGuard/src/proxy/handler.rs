use crate::proxy::proxy;
use crate::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use std::time::Instant;

const ADMIN_PATH: &str = "/admin";
const REQUIRED_GROUP: &str = "/TI/Infraestrutura";

pub async fn proxy_handler(State(state): State<AppState>, req: Request<Body>) -> Response {
    let start = Instant::now();
    let (method, path) = extract_route_info(&req);

    if let Some(claims) = req.extensions().get::<crate::auth::jwt::KeycloakClaims>() {
        if path.starts_with(ADMIN_PATH) && !has_group(claims, REQUIRED_GROUP) {
            return StatusCode::FORBIDDEN.into_response();
        }
    }

    let response = proxy::forward(req, &state.config.target_service).await;

    record_metrics(&method, &path, response.status().as_u16(), start);
    response
}

fn extract_route_info(req: &Request<Body>) -> (String, String) {
    (req.method().to_string(), req.uri().path().to_string())
}

fn has_group(claims: &crate::auth::jwt::KeycloakClaims, group: &str) -> bool {
    claims
        .groups
        .as_ref()
        .map(|g| g.contains(&group.to_string()))
        .unwrap_or(false)
}

fn record_metrics(method: &str, path: &str, status: u16, start: Instant) {
    let status_str = status.to_string();
    crate::observability::metrics::record(method, path, &status_str);
    crate::observability::metrics::record_duration(method, path, start);
}
