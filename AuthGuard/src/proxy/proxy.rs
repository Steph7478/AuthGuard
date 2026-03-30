use axum::{body::Body, extract::Request, response::Response};
use hyper::Uri;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use std::str::FromStr;

pub async fn forward(mut req: Request<Body>, target_service: &str) -> Response {
    if let Ok(uri) = Uri::from_str(&format!(
        "{}{}",
        target_service,
        req.uri().path_and_query().map(|v| v.as_str()).unwrap_or("")
    )) {
        *req.uri_mut() = uri;
    }

    match Client::builder(TokioExecutor::new())
        .build_http()
        .request(req)
        .await
    {
        Ok(res) => {
            let (parts, body) = res.into_parts();
            Response::from_parts(parts, Body::new(body))
        }
        Err(_) => Response::builder()
            .status(502)
            .body(Body::from("Bad Gateway"))
            .unwrap(),
    }
}
