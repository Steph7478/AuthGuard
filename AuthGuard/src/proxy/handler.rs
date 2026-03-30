use crate::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use std::time::Instant;

pub async fn proxy_handler(State(state): State<AppState>, mut req: Request<Body>) -> Response {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let start = Instant::now();

    if let Some(claims) = req
        .extensions()
        .get::<crate::auth::claims::KeycloakClaims>()
    {
        if path.starts_with("/admin") {
            let allowed = claims
            .groups
            .as_ref()
            .map(|g| g.contains(&"/TI/Infraestrutura".to_string()))
            .unwrap_or(false);

            if !allowed {
                return StatusCode::FORBIDDEN.into_response();
            }
        }
    }

    let response = crate::proxy::proxy::forward(req, &state.config.target_service).await;

    let status = response.status().as_u16().to_string();
    crate::observability::metrics::record(&method, &path, &status);
    crate::observability::metrics::record_duration(&method, &path, start);

    response
}
