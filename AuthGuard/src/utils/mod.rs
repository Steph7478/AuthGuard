use axum::{body::Body, extract::Request};

pub fn extract_token(req: &Request<Body>) -> Option<&str> {
    req.headers()
        .get("Authorization")?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}
