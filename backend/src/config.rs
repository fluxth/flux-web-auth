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

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub allowed_urls: Vec<Url>,
}

fn default_host() -> String {
    "[::]".into()
}

fn default_port() -> u16 {
    9090
}
