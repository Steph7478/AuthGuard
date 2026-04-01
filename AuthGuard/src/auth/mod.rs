use crate::services::RedisService;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

const JWKS_CACHE_KEY: &str = "jwks";
const CACHE_TTL: u64 = 86400;
const TOKEN_LEEWAY: u64 = 60;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub groups: Option<Vec<String>>,
    pub email: Option<String>,
    pub preferred_username: Option<String>,
    pub exp: usize,
}

#[derive(Deserialize, Serialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Deserialize, Serialize)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

pub async fn verify(
    token: &str,
    jwks_url: &str,
    issuer: &str,
    redis: &RedisService,
) -> Result<Claims, String> {
    let kid = decode_header(token)
        .map_err(|e| format!("Invalid token header: {e}"))?
        .kid
        .ok_or("Missing key ID (kid) in token")?;

    let jwk = get_jwk(jwks_url, &kid, redis).await?;
    let key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|_| "Invalid RSA key components")?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[issuer]);
    validation.validate_aud = false;
    validation.leeway = TOKEN_LEEWAY;

    decode::<Claims>(token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|e| format!("JWT validation failed: {e}"))
}

async fn get_jwk(url: &str, kid: &str, redis: &RedisService) -> Result<Jwk, String> {
    let jwks = get_jwks(url, redis).await?;
    jwks.keys
        .into_iter()
        .find(|k| k.kid == kid)
        .ok_or_else(|| format!("Key ID {kid} not found in JWKS"))
}

async fn get_jwks(url: &str, redis: &RedisService) -> Result<Jwks, String> {
    if let Some(cached) = redis.get(JWKS_CACHE_KEY).await {
        if let Ok(jwks) = serde_json::from_str(&cached) {
            return Ok(jwks);
        }
    }

    let jwks: Jwks = reqwest::get(url)
        .await
        .map_err(|e| format!("JWKS fetch failed: {e}"))?
        .json()
        .await
        .map_err(|e| format!("JWKS parse failed: {e}"))?;

    if let Ok(json) = serde_json::to_string(&jwks) {
        let _ = redis.set(JWKS_CACHE_KEY, &json, CACHE_TTL).await;
    }

    Ok(jwks)
}
