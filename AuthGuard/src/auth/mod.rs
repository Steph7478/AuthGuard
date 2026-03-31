use crate::services::RedisService;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub groups: Option<Vec<String>>,
    pub email: Option<String>,
    pub preferred_username: Option<String>,
    pub exp: usize,
}

#[derive(Deserialize, Serialize)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

#[derive(Deserialize, Serialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

pub async fn verify(
    token: &str,
    jwks_url: &str,
    jwt_issuer: &str,
    redis: &RedisService,
) -> Result<Claims, String> {
    let header = decode_header(token).map_err(|e| format!("Invalid header: {}", e))?;
    let kid = header.kid.ok_or("Missing kid in token header")?;

    let jwks = get_jwks(jwks_url, redis).await?;
    let jwk = jwks
        .keys
        .iter()
        .find(|k| k.kid == kid)
        .ok_or("Key ID not found")?;

    let key =
        DecodingKey::from_rsa_components(&jwk.n, &jwk.e).map_err(|_| "Invalid RSA components")?;

    let mut validation = Validation::new(Algorithm::RS256);

    validation.set_issuer(&[jwt_issuer]);
    validation.validate_aud = false;
    validation.leeway = 60;

    match decode::<Claims>(token, &key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => {
            println!("JWT Validation Error: {:?}", e);
            Err(e.to_string())
        }
    }
}

async fn get_jwks(url: &str, redis: &RedisService) -> Result<Jwks, String> {
    const CACHE_KEY: &str = "jwks_cache_key";

    if let Some(cached) = redis.get(CACHE_KEY).await {
        if let Ok(jwks) = serde_json::from_str(&cached) {
            return Ok(jwks);
        }
    }

    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("JWKS Fetch error: {}", e))?;
    let jwks: Jwks = response
        .json()
        .await
        .map_err(|e| format!("JWKS Parse error: {}", e))?;

    if let Ok(json) = serde_json::to_string(&jwks) {
        redis.set(CACHE_KEY, &json, 86400).await;
    }

    Ok(jwks)
}
