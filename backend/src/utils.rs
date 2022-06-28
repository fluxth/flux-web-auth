use anyhow::Result;
use jwt::{PKeyWithDigest, VerifyWithKey};
use openssl::pkey::Public;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use tonic::Request;

use crate::TOKEN_KEY;

pub fn get_token<T>(request: &Request<T>) -> Option<Cow<str>> {
    if let Some(token) = request.metadata().get(TOKEN_KEY) {
        if let Ok(token_str) = token.to_str() {
            Some(token_str.into())
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub nbf: u64,
}

type JwtToken = jwt::Token<jwt::Header, Claims, jwt::Verified>;
pub fn verify_jwt(key: &PKeyWithDigest<Public>, jwt: &str) -> Result<JwtToken> {
    Ok(VerifyWithKey::verify_with_key(jwt, key)?)
}

pub fn has_valid_session(_jwt: &JwtToken) -> Result<bool> {
    // FIXME: validate session
    Ok(true)
}
