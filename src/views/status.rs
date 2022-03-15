use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::Template;
use serde_json::Value;
use std::collections::BTreeMap;

use crate::utils;
use crate::views;

#[derive(Serialize)]
struct StatusContext {
    username: Option<String>,
    button_url: String,
}

#[derive(Debug)]
pub struct StatusHeaders {
    host: String,
    uri: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for StatusHeaders {
    type Error = &'static str;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let host = match request.headers().get_one("Host") {
            Some(value) => value.to_string(),
            None => return Outcome::Failure((Status::BadRequest, "Host header required")),
        };

        let uri = request.uri().to_string();
        Outcome::Success(Self { host, uri })
    }
}

#[get("/status")]
pub fn get_status(
    headers: StatusHeaders,
    config: &State<crate::Config>,
    cookies: &CookieJar<'_>,
) -> Template {
    #[cfg(debug_assertions)]
    let host = &headers.host;
    #[cfg(not(debug_assertions))]
    let host = crate::SITE_HOST;

    let return_url = format!("{}://{}{}", crate::DEFAULT_SCHEME, host, headers.uri);
    let mut button_url: Option<String> = None;
    let mut username: Option<String> = None;

    if let Some(token) = cookies.get(crate::AUTHTOKEN_COOKIE_NAME) {
        if let Ok(token_data) = utils::decode_jwt(config.jwt_public_key.as_bytes(), token.value()) {
            if utils::jwt_duration_is_valid(&token_data) {
                username = get_username_from_claims(token_data.claims());
                if username.is_some() {
                    button_url = Some(uri!(views::get_logout(Some(&return_url))).to_string());
                }
            }
        }
    }

    Template::render(
        "pages/status",
        StatusContext {
            username,
            button_url: button_url.unwrap_or(uri!(views::get_login(return_url)).to_string()),
        },
    )
}

fn get_username_from_claims(claims: &BTreeMap<String, Value>) -> Option<String> {
    Some(claims.get("sub")?.as_str()?.to_string())
}
