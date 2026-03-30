use crate::services::redis::RedisService;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeycloakClaims {
    pub sub: String,
    pub aud: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
    #[serde(rename = "costCenter")]
    pub cost_center: Option<Vec<String>>,
    pub email: Option<String>,
    pub preferred_username: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

impl Jwk {
    fn into_decoding_key(self) -> Result<DecodingKey, &'static str> {
        DecodingKey::from_rsa_components(&self.n, &self.e).map_err(|_| "RSA error")
    }
}

#[derive(Deserialize, Serialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

impl Jwks {
    async fn fetch(jwks_url: &str) -> Result<Self, &'static str> {
        reqwest::get(jwks_url)
            .await
            .map_err(|_| "Fetch error")?
            .json()
            .await
            .map_err(|_| "Parse error")
    }

    fn find_key(&self, kid: &str) -> Option<&Jwk> {
        self.keys.iter().find(|k| k.kid == kid)
    }
}

pub async fn verify(
    token: &str,
    jwks_url: &str,
    redis: &RedisService,
) -> Result<KeycloakClaims, String> {
    let header = decode_header(token).map_err(|_| "Invalid header")?;
    let kid = header.kid.ok_or("Missing KID")?;

    let jwks = get_jwks(jwks_url, redis).await?;
    let jwk = jwks.find_key(&kid).ok_or("KID not found")?;
    let decoding_key = jwk.clone().into_decoding_key().map_err(|e| e.to_string())?;

    let validation = create_validation();

    decode::<KeycloakClaims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| e.to_string())
}

async fn get_jwks(jwks_url: &str, redis: &RedisService) -> Result<Jwks, String> {
    const CACHE_KEY: &str = "jwks_cache";
    const CACHE_TTL: u64 = 86400;

    if let Some(cached) = redis.get_key(CACHE_KEY).await {
        serde_json::from_str(&cached).map_err(|_| "Parse cache error".to_string())
    } else {
        let jwks = Jwks::fetch(jwks_url).await.map_err(|e| e.to_string())?;
        if let Ok(json) = serde_json::to_string(&jwks) {
            redis.set_key(CACHE_KEY, &json, CACHE_TTL).await;
        }
        Ok(jwks)
    }
}

fn create_validation() -> Validation {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_aud = true;
    validation.set_audience(&["authguard-service"]);
    validation.set_issuer(&["http://localhost:8080/realms/authguard"]);
    validation
}
