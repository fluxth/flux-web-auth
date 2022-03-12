use anyhow::{bail, format_err, Result};
use url::Url;

use jwt::{PKeyWithDigest, SignWithKey, VerifyWithKey};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use std::collections::BTreeMap;

pub fn validate_next_url(next: &str) -> anyhow::Result<Url> {
    if next.len() == 0 {
        bail!("`next` parameter is required");
    }

    let url = Url::parse(next)?;

    if url.scheme() != "https" {
        bail!("only https scheme is supported");
    }

    let host = url.host_str().ok_or(format_err!("hostname is required"))?;

    if host == "search.flux.ci" {
        Ok(url)
    } else {
        Err(format_err!("hostname not allowed"))
    }
}

pub fn generate_jwt(private_key: &[u8], username: &str) -> Result<String> {
    use serde_json::json;
    let private_key = openssl::ec::EcKey::private_key_from_pem(private_key)?;
    let key = PKeyWithDigest {
        key: PKey::from_ec_key(private_key)?,
        digest: MessageDigest::sha384(),
    };

    let now = chrono::Utc::now();
    let now_unix = now.timestamp();
    let expiry_unix = now
        .checked_add_signed(chrono::Duration::days(7))
        .ok_or(format_err!("Cannot add expiry time"))?
        .timestamp();

    let claims = json!({
        "iss": "fluxauth",
        "sub": username,
        "nbf": now_unix,
        "iat": now_unix,
        "exp": expiry_unix,
    });

    Ok(claims.sign_with_key(&key)?)
}

type JwtToken = jwt::Token<jwt::Header, BTreeMap<String, String>, jwt::Verified>;
pub fn decode_jwt(public_key: &[u8], jwt: &str) -> Result<JwtToken> {
    let public_key = openssl::ec::EcKey::public_key_from_pem(public_key)?;
    let key = PKeyWithDigest {
        key: PKey::from_ec_key(public_key)?,
        digest: MessageDigest::sha384(),
    };

    Ok(VerifyWithKey::verify_with_key(jwt, &key)?)
}

pub fn jwt_duration_is_valid(token: &JwtToken) -> bool {
    let claims = token.claims();
    let now_unix = chrono::Utc::now().timestamp();

    if let Some(expiry_unix_str) = claims.get("exp") {
        let expiry_unix = match expiry_unix_str.parse::<i64>() {
            Ok(value) => value,
            Err(_) => {
                return false;
            }
        };

        if now_unix > expiry_unix {
            // Expired
            return false;
        }
    }

    if let Some(not_before_unix_str) = claims.get("nbf") {
        let not_before_unix = match not_before_unix_str.parse::<i64>() {
            Ok(value) => value,
            Err(_) => {
                return false;
            }
        };

        if now_unix < not_before_unix {
            // Not valid yet
            return false;
        }
    }

    true
}
