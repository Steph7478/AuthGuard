use crate::auth::claims::KeycloakClaims;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Deserialize, Serialize, Clone)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

pub async fn verify(
    token: &str,
    jwks_url: &str,
    redis: &crate::services::redis::RedisService,
) -> Result<KeycloakClaims, String> {
    let header = decode_header(token).map_err(|_| "Invalid header")?;
    let kid = header.kid.ok_or("Missing KID")?;

    let cached_jwks: Option<String> = redis.get_key("jwks_cache").await;

    let jwks: Jwks = if let Some(json_str) = cached_jwks {
        serde_json::from_str(&json_str).map_err(|_| "Parse cache error")?
    } else {
        let resp: Jwks = reqwest::get(jwks_url)
            .await
            .map_err(|_| "Fetch error")?
            .json()
            .await
            .map_err(|_| "Parse error")?;

        let json_to_cache = serde_json::to_string(&resp).unwrap();
        redis.set_key("jwks_cache", &json_to_cache, 86400).await;
        resp
    };

    let jwk = jwks
        .keys
        .into_iter()
        .find(|k| k.kid == kid)
        .ok_or("KID not found")?;

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e).map_err(|_| "RSA error")?;

    let token_data =
        decode::<KeycloakClaims>(token, &decoding_key, &Validation::new(Algorithm::RS256))
            .map_err(|e| e.to_string())?;

    Ok(token_data.claims)
}
