use jwt::PKeyWithDigest;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private, Public};
use serde::Deserialize;
use std::ops::Deref;

#[derive(Debug)]
pub struct Url(url::Url);

impl Deref for Url {
    type Target = url::Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Ok(Url(url::Url::parse(&string).unwrap()))
    }
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub allowed_urls: Vec<Url>,
    #[serde(deserialize_with = "decode_jwt_public_key")]
    pub jwt_public_key: PKeyWithDigest<Public>,
    #[serde(deserialize_with = "decode_jwt_private_key")]
    pub jwt_private_key: PKeyWithDigest<Private>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME: implement proper debug for config
        write!(f, "config")
    }
}

fn default_host() -> String {
    "[::]".into()
}

fn default_port() -> u16 {
    9090
}

fn decode_jwt_public_key<'de, D>(deserializer: D) -> Result<PKeyWithDigest<Public>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let pem = String::deserialize(deserializer)?.replace("\\n", "\n");
    let eckey =
        openssl::ec::EcKey::public_key_from_pem(pem.as_bytes()).expect("valid jwt public key pem");
    let pkey = PKey::from_ec_key(eckey).expect("created public pkey");

    Ok(PKeyWithDigest {
        key: pkey,
        digest: MessageDigest::sha256(),
    })
}

fn decode_jwt_private_key<'de, D>(deserializer: D) -> Result<PKeyWithDigest<Private>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let pem = String::deserialize(deserializer)?.replace("\\n", "\n");
    let eckey = openssl::ec::EcKey::private_key_from_pem(pem.as_bytes())
        .expect("valid jwt private key pem");
    let pkey = PKey::from_ec_key(eckey).expect("created private pkey");

    Ok(PKeyWithDigest {
        key: pkey,
        digest: MessageDigest::sha256(),
    })
}
