use axum::{body::Body, http::Request};

pub fn extract_ip(req: &Request<Body>) -> String {
    req.headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            req.headers()
                .get("x-real-ip")
                .and_then(|h| h.to_str().ok().map(String::from))
        })
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn extract_token(req: &Request<Body>) -> Option<&str> {
    req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .or_else(|| {
            req.headers()
                .get("Cookie")
                .and_then(|h| h.to_str().ok())
                .and_then(|cookies| {
                    cookies
                        .split(';')
                        .find(|c| c.trim().starts_with("access_token="))
                        .map(|c| c.trim().trim_start_matches("access_token="))
                })
        })
}
