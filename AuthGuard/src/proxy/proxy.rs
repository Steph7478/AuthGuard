use axum::{body::Body, extract::Request, response::Response};
use hyper::Uri;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use std::str::FromStr;

pub async fn forward(mut req: Request<Body>, target_service: &str) -> Response {
    let path_query = req.uri().path_and_query().map(|v| v.as_str()).unwrap_or("");
    let dest_uri = format!("{}{}", target_service, path_query);

    if let Ok(uri) = Uri::from_str(&dest_uri) {
        *req.uri_mut() = uri;
    }

    let client = Client::builder(TokioExecutor::new()).build_http();

    match client.request(req).await {
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
