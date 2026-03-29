use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use hyper::body::Bytes;
use reqwest::{
    header::{HeaderName as ReqHeaderName, HeaderValue as ReqHeaderValue},
    Client, Method,
};

pub async fn forward(req: Request<Body>, target: &str) -> Response {
    let client = Client::new();

    let uri = format!("{}{}", target, req.uri());

    let method = Method::from_bytes(req.method().as_str().as_bytes()).unwrap();

    let mut request_builder = client.request(method, &uri);

    for (key, value) in req.headers() {
        let key = ReqHeaderName::from_bytes(key.as_str().as_bytes()).unwrap();
        let value = ReqHeaderValue::from_bytes(value.as_bytes()).unwrap();
        request_builder = request_builder.header(key, value);
    }

    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    request_builder = request_builder.body(body_bytes);

    let res = request_builder.send().await.unwrap();

    let status = StatusCode::from_u16(res.status().as_u16()).unwrap();

    let body = res.bytes().await.unwrap();

    Response::builder()
        .status(status)
        .body(Body::from(body))
        .unwrap()
}
