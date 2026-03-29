use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use tokio::time::{sleep, Duration};

#[derive(Deserialize)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}
#[derive(Deserialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

pub async fn verify(token: &str, jwks_url: &str) -> Result<(), String> {
    for _ in 0..3 {
        if let Ok(_) = try_verify(token, jwks_url).await {
            return Ok(());
        }
        sleep(Duration::from_millis(300)).await;
    }
    Err("verification failed".into())
}

async fn try_verify(token: &str, jwks_url: &str) -> Result<(), String> {
    let header = decode_header(token).map_err(|_| "invalid header")?;
    let kid = header.kid.ok_or("missing kid")?;
    let jwks: Jwks = reqwest::get(jwks_url)
        .await
        .map_err(|_| "fetch error")?
        .json()
        .await
        .map_err(|_| "parse error")?;

    let jwk = jwks
        .keys
        .into_iter()
        .find(|k| k.kid == kid)
        .ok_or("key not found")?;
    let key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e).map_err(|_| "invalid key")?;

    decode::<serde_json::Value>(token, &key, &Validation::new(Algorithm::RS256))
        .map_err(|_| "invalid token")?;
    Ok(())
}
