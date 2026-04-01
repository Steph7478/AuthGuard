// src/config.rs
use dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub port: String,
    pub redis_url: String,
    pub jwks_url: String,
    pub jwt_issuer: String,
    pub keycloak_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

pub fn load_config() -> AppConfig {
    dotenv::dotenv().ok();

    AppConfig {
        port: env::var("PORT").unwrap_or("3000".into()),
        redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
        jwks_url: env::var("JWKS_URL").expect("JWKS_URL must be set"),
        jwt_issuer: env::var("JWT_ISSUER").expect("JWT_ISSUER must be set"),
        keycloak_url: env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set"),
        realm: env::var("REALM").expect("REALM must be set"),
        client_id: env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
        client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set"),
        redirect_uri: env::var("REDIRECT_URI").expect("REDIRECT_URI must be set"),
    }
}
