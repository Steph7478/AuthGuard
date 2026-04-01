use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json, Response},
};
use http::{header, StatusCode};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};

use crate::config::AppConfig;

macro_rules! err {
    ($status:expr, $msg:expr) => {
        return ($status, Json(json!({"error":$msg}))).into_response()
    };
}

pub async fn callback_handler(
    Query(params): Query<HashMap<String, String>>,
    State(config): State<Arc<AppConfig>>,
) -> Response {
    let code = match params.get("code") {
        Some(c) => c,
        None => err!(StatusCode::BAD_REQUEST, "Missing code"),
    };

    let client = reqwest::Client::new();

    let Ok(resp) = client
        .post(config.token_endpoint())
        .form(&[
            ("client_id", &config.client_id as &str),
            ("client_secret", &config.client_secret as &str),
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &config.redirect_uri as &str),
        ])
        .send()
        .await
    else {
        err!(StatusCode::BAD_GATEWAY, "Token failed")
    };

    let Ok(tokens) = resp.json::<serde_json::Value>().await else {
        err!(StatusCode::BAD_GATEWAY, "Parse failed")
    };

    let mut res = Json(json!({"ok":true})).into_response();
    let cookie = |name, val, age| {
        format!("{name}={val}; HttpOnly; Path=/; Max-Age={age}")
            .parse()
            .unwrap()
    };

    res.headers_mut().insert(
        header::SET_COOKIE,
        cookie(
            "access_token",
            tokens["access_token"].as_str().unwrap_or_default(),
            3600,
        ),
    );
    res.headers_mut().append(
        header::SET_COOKIE,
        cookie(
            "refresh_token",
            tokens["refresh_token"].as_str().unwrap_or_default(),
            86400,
        ),
    );

    res
}
