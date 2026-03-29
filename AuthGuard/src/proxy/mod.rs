use axum::{body::Body, http::Request, response::Response};
use reqwest::{Client, Method};

pub async fn forward(req: Request<Body>, target: &str) -> Response {
    let client = Client::new();

    let uri = format!("{}{}", target, req.uri());

    let method = Method::from_bytes(req.method().as_str().as_bytes()).unwrap();

    let mut request_builder = client.request(method, uri);

    for (key, value) in req.headers() {
        request_builder = request_builder.header(key, value);
    }

    let res = request_builder.body(req.into_body()).send().await.unwrap();

    let status = res.status();
    let body = res.bytes().await.unwrap();

    Response::builder()
        .status(status)
        .body(Body::from(body))
        .unwrap()
}
