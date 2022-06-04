use crate::error::Error;
use anyhow::format_err;
use unwrap_or::unwrap_some_or;
use url::Url;

pub fn validate_redirect_url(redirect_url: &str) -> Result<Url, Error> {
    let url = Url::parse(redirect_url)?;
    let config = crate::CONFIG
        .get()
        .ok_or(format_err!("Config not initialized"))?;

    for allowed_url in &config.allowed_urls {
        if url.is_valid_of(&allowed_url) == true {
            return Ok(url);
        }
    }

    Err(Error {
        error_code: 4242,
        message: "Redirect URL not allowed".into(),
        http_response_code: 400,
    })
}

trait IsValidOf {
    fn is_valid_of(&self, url: &Url) -> bool;
}

impl IsValidOf for Url {
    fn is_valid_of(&self, url: &Url) -> bool {
        if self.host() != url.host() {
            return false;
        }

        if self.scheme() != url.scheme() {
            return false;
        }

        if self.port() != url.port() {
            return false;
        }

        if url.path() != "/" {
            // Also validate path
            let needle = unwrap_some_or!(url.path_segments(), return false);
            let mut haystack = unwrap_some_or!(self.path_segments(), return false);

            for item in needle {
                if Some(item) != haystack.next() {
                    return false;
                }
            }
        }

        true
    }
}
