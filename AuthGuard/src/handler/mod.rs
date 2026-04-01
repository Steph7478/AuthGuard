use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use http::{header, HeaderValue, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;

use crate::config::AppConfig;

pub async fn callback_handler(
    Query(params): Query<HashMap<String, String>>,
    State(config): State<Arc<AppConfig>>,
) -> impl IntoResponse {
    let code = match params.get("code") {
        Some(c) => c,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "Missing code parameter".to_string(),
            )
                .into_response();
        }
    };

    let client = reqwest::Client::new();
    let token_endpoint = format!(
        "{}/auth/realms/{}/protocol/openid-connect/token",
        config.keycloak_url, config.realm
    );

    let params = [
        ("client_id", config.client_id.as_str()),
        ("client_secret", config.client_secret.as_str()),
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", &config.redirect_uri),
    ];

    match client.post(&token_endpoint).form(&params).send().await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(tokens) => {
                let access_token = tokens["access_token"].as_str().unwrap_or("");
                let refresh_token = tokens["refresh_token"].as_str().unwrap_or("");

                let cookie_header = format!(
                    "access_token={}; HttpOnly; Path=/; SameSite=Lax; Max-Age=3600",
                    access_token
                );

                let refresh_cookie_header = format!(
                    "refresh_token={}; HttpOnly; Path=/; SameSite=Lax; Max-Age=86400",
                    refresh_token
                );

                let response = Response::builder()
                    .status(StatusCode::FOUND)
                    .header(header::SET_COOKIE, cookie_header)
                    .header(header::SET_COOKIE, refresh_cookie_header)
                    .header(header::LOCATION, HeaderValue::from_static("/"))
                    .body(axum::body::Body::empty())
                    .unwrap();

                response.into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse tokens: {}", e),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Token request failed: {}", e),
        )
            .into_response(),
    }
}
