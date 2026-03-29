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
    let mut attempts = 0;

    loop {
        let result = try_verify(token, jwks_url).await;

        if result.is_ok() {
            return Ok(());
        }

        attempts += 1;

        if attempts >= 3 {
            return result;
        }

        sleep(Duration::from_millis(300)).await;
    }
}

async fn try_verify(token: &str, jwks_url: &str) -> Result<(), String> {
    let header = decode_header(token).map_err(|_| "invalid header")?;

    let jwks: Jwks = reqwest::get(jwks_url)
        .await
        .map_err(|_| "jwks fetch error")?
        .json()
        .await
        .map_err(|_| "jwks parse error")?;

    let kid = header.kid.ok_or("missing kid")?;

    let jwk = jwks
        .keys
        .into_iter()
        .find(|k| k.kid == kid)
        .ok_or("key not found")?;

    let key =
        DecodingKey::from_rsa_components(&jwk.n, &jwk.e).map_err(|_| "invalid decoding key")?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;

    decode::<serde_json::Value>(token, &key, &validation).map_err(|_| "invalid token")?;

    Ok(())
}
