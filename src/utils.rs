use anyhow::{bail, format_err, Result};
use jwt::{PKeyWithDigest, SignWithKey, VerifyWithKey};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use rocket::State;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use url::Url;

pub fn validate_next_url(next: &str, config: &State<crate::Config>) -> anyhow::Result<Url> {
    if next.len() == 0 {
        bail!("`next` parameter is required");
    }

    let url = Url::parse(next)?;

    // Only activate scheme check in release mode
    #[cfg(not(debug_assertions))]
    if url.scheme() != "https" {
        bail!("only https scheme is supported");
    }

    let host = url.host_str().ok_or(format_err!("hostname is required"))?;
    let allowed_hosts = &config.allowed_next_hosts;

    // FIXME: this allows every port coming `allowed_hosts`, tighten this later.
    if allowed_hosts
        .into_iter()
        .find(|entry| *entry == host)
        .is_some()
    {
        Ok(url)
    } else {
        Err(format_err!("hostname not allowed"))
    }
}

pub fn generate_jwt(private_key: &[u8], username: &str) -> Result<String> {
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

type JwtToken = jwt::Token<jwt::Header, BTreeMap<String, Value>, jwt::Verified>;
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
    let now_unix = chrono::Utc::now().timestamp() as u64;

    if let Some(expiry_unix_value) = claims.get("exp") {
        let expiry_unix = match expiry_unix_value.as_u64() {
            Some(value) => value,
            None => {
                return false;
            }
        };

        if now_unix > expiry_unix {
            // Expired
            return false;
        }
    }

    if let Some(not_before_unix_value) = claims.get("nbf") {
        let not_before_unix = match not_before_unix_value.as_u64() {
            Some(value) => value,
            None => {
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
