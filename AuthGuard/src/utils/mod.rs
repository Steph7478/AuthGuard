use axum::body::Body;
use axum::http::Request;

pub fn extract_ip(req: &Request<Body>) -> String {
    req.headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").trim().to_string())
        .or_else(|| {
            req.headers()
                .get("x-real-ip")
                .and_then(|h| h.to_str().ok())
                .map(String::from)
        })
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn extract_token(req: &Request<Body>) -> Option<&str> {
    if let Some(token) = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth| {
            if auth.starts_with("Bearer ") {
                Some(&auth[7..])
            } else {
                None
            }
        })
    {
        return Some(token);
    }

    req.headers()
        .get("Cookie")
        .and_then(|h| h.to_str().ok())
        .and_then(|cookies| {
            for cookie in cookies.split(';') {
                let cookie = cookie.trim();
                if cookie.starts_with("access_token=") {
                    return Some(&cookie["access_token=".len()..]);
                }
            }
            None
        })
}
