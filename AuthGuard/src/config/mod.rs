use dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub port: String,
    pub redis_url: String,
    pub jwks_url: String,
    pub jwt_issuer: String,
}

pub fn load_config() -> AppConfig {
    dotenv::dotenv().ok();

    AppConfig {
        port: env::var("PORT").unwrap_or("3000".into()),
        redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
        jwks_url: env::var("JWKS_URL").expect("JWKS_URL must be set"),
        jwt_issuer: env::var("JWT_ISSUER").expect("JWT_ISSUER must be set"),
    }
}
