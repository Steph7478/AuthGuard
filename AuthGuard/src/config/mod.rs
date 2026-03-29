use std::env;
use serde::Deserialize;
use std::fs;

#[derive(Clone)]
pub struct AppConfig {
    pub port: String,
    pub redis_url: String,
    pub jwks_url: String,
    pub target_service: String,
    pub rate_limit: i32,
    pub window: usize,
}

#[derive(Deserialize, Clone)]
pub struct Policy {
    pub require_auth: bool,
    pub rate_limit_by: String,
    pub block_ips: Vec<String>,
}

pub fn load_config() -> (AppConfig, Policy) {
    dotenv::dotenv().ok();

    let policy_str = fs::read_to_string("config.json").unwrap();
    let policy: Policy = serde_json::from_str(&policy_str).unwrap();

    let config = AppConfig {
        port: env::var("PORT").unwrap(),
        redis_url: env::var("REDIS_URL").unwrap(),
        jwks_url: env::var("JWKS_URL").unwrap(),
        target_service: env::var("TARGET_SERVICE").unwrap(),
        rate_limit: env::var("RATE_LIMIT").unwrap().parse().unwrap(),
        window: env::var("WINDOW_SECONDS").unwrap().parse().unwrap(),
    };

    (config, policy)
}