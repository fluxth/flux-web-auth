use rocket::http::CookieJar;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::Template;
use serde_json::Value;
use std::collections::BTreeMap;

#[cfg(not(debug_assertions))]
use rocket::http::Status;

use crate::utils;
use crate::views;

#[derive(Serialize)]
struct StatusContext {
    username: Option<String>,
    button_url: String,
}

#[derive(Debug)]
pub struct StatusRequestContext {
    uri: String,
    scheme: &'static str,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for StatusRequestContext {
    type Error = &'static str;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let uri = request.uri().to_string();

        #[cfg(debug_assertions)]
        let scheme = "http";

        #[cfg(not(debug_assertions))]
        let scheme = match request.headers().get_one("X-Scheme") {
            Some(scheme) => match scheme {
                "http" => "http",
                "https" => "https",
                _ => return Outcome::Failure((Status::BadRequest, "invalid scheme")),
            },
            None => return Outcome::Failure((Status::BadRequest, "no X-Scheme header")),
        };

        Outcome::Success(Self { uri, scheme })
    }
}

#[get("/status")]
pub fn get_status(
    request_context: StatusRequestContext,
    config: &State<crate::Config>,
    cookies: &CookieJar<'_>,
) -> Template {
    let site_host = config.site_host.as_str();

    let return_url = format!(
        "{}://{}{}",
        request_context.scheme, site_host, request_context.uri
    );
    let mut button_url: Option<String> = None;
    let mut username: Option<String> = None;

    if let Some(token) = cookies.get(config.authtoken_cookie_name.as_str()) {
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
