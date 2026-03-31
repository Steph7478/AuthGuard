use axum::response::{IntoResponse, Response};
use http::StatusCode;
use lazy_static::lazy_static;
use prometheus::{register_counter, Counter, Encoder, TextEncoder};

lazy_static! {
    static ref REQUESTS: Counter =
        register_counter!("auth_requests_total", "Total validation requests").unwrap();
}

pub async fn handler() -> Response {
    REQUESTS.inc();
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();

    match encoder.encode(&prometheus::gather(), &mut buffer) {
        Ok(()) => (StatusCode::OK, buffer).into_response(),
        Err(e) => {
            eprintln!("Error encoding metrics: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error encoding metrics").into_response()
        }
    }
}
