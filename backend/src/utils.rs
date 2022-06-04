use crate::TOKEN_KEY;
use tonic::Request;

pub fn get_token<T>(request: &Request<T>) -> Option<String> {
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
