use prometheus::{
    Encoder, TextEncoder,
    CounterVec, HistogramVec,
    register_counter_vec, register_histogram_vec
};

use std::time::Instant;

lazy_static::lazy_static! {
    pub static ref HTTP_REQUESTS: CounterVec =
        register_counter_vec!(
            "http_requests_total",
            "Total HTTP Requests",
            &["method", "path", "status"]
        ).unwrap();

    pub static ref HTTP_DURATION: HistogramVec =
        register_histogram_vec!(
            "http_request_duration_seconds",
            "Request duration in seconds",
            &["method", "path"]
        ).unwrap();
}

pub fn record(method: &str, path: &str, status: &str) {
    HTTP_REQUESTS
        .with_label_values(&[method, path, status])
        .inc();
}

pub fn record_duration(method: &str, path: &str, start: Instant) {
    let duration = start.elapsed().as_secs_f64();

    HTTP_DURATION
        .with_label_values(&[method, path])
        .observe(duration);
}

pub async fn metrics_handler() -> String {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();

    encoder.encode(&prometheus::gather(), &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}