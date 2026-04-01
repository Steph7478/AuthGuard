use axum::response::{IntoResponse, Response};
use http::StatusCode;
use lazy_static::lazy_static;
use prometheus::{register_counter, Counter, Encoder, TextEncoder};

lazy_static! {
    static ref REQUESTS: Counter = register_counter!("auth_requests_total", "Total authentication requests").unwrap();
}

pub async fn handler() -> Response {
    REQUESTS.inc();
    
    let mut buffer = Vec::new();
    match TextEncoder::new().encode(&prometheus::gather(), &mut buffer) {
        Ok(()) => (StatusCode::OK, buffer).into_response(),
        Err(e) => {
            eprintln!("Metrics encoding error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to encode metrics").into_response()
        }
    }
}