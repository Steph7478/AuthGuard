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
        if path.starts_with("/admin") && !claims.groups.contains(&"/TI/Infraestrutura".to_string())
        {
            return StatusCode::FORBIDDEN.into_response();
        }

        if let Some(ccs) = &claims.cost_center {
            let cc_header = ccs.join(",");
            if let Ok(value) = cc_header.parse() {
                req.headers_mut().insert("X-Cost-Center", value);
            }
        }
    }

    let response = crate::proxy::proxy::forward(req, &state.config.target_service).await;

    let status = response.status().as_u16().to_string();
    crate::observability::metrics::record(&method, &path, &status);
    crate::observability::metrics::record_duration(&method, &path, start);

    response
}
