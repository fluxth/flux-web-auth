pub use crate::proto::Error;
use std::convert::From;

impl Error {
    pub fn new(message: String) -> Self {
        Self {
            error_code: 9898,
            message,
            http_response_code: 0,
        }
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Self::new(error.to_string())
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Self::new(error.to_string())
    }
}
