use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::Template;
use serde_json::json;

use crate::utils::validate_next_url;

#[derive(Serialize)]
struct LoginContext<'a> {
    next_host: Option<&'a str>,
    error: Option<&'a str>,
}

#[derive(Responder)]
pub enum LoginResponse {
    Redirect(Redirect),
    Template(Template),
}

#[get("/login?<next>")]
pub fn get_login(
    config: &State<crate::Config>,
    cookies: &CookieJar<'_>,
    next: String,
) -> Result<LoginResponse, Template> {
    match validate_next_url(&next, &config) {
        Ok(url) => {
            // Check existing session
            if has_active_session(cookies, config.jwt_public_key.as_bytes()) {
                return Ok(LoginResponse::Redirect(Redirect::to(next)));
            }

            // Render login form
            Ok(LoginResponse::Template(Template::render(
                "login",
                LoginContext {
                    next_host: url.host_str(),
                    error: None,
                },
            )))
        }

        // Invalid host
        Err(_) => Err(Template::render("errors/denied", json!({}))),
    }
}

#[derive(Responder)]
pub enum LoginProcessResponse {
    Redirect(Redirect),
    Template(Template),
}

#[post("/login?<next>")]
pub fn post_login(
    config: &State<crate::Config>,
    cookies: &CookieJar<'_>,
    next: String,
) -> Result<LoginProcessResponse, Template> {
    match validate_next_url(&next, &config) {
        Ok(url) => {
            // Check existing session
            if has_active_session(cookies, config.jwt_public_key.as_bytes()) {
                return Ok(LoginProcessResponse::Redirect(Redirect::to(next)));
            }

            // TODO: Authenticate

            let jwt_token =
                match crate::utils::generate_jwt(config.jwt_private_key.as_bytes(), "anon") {
                    Ok(token) => token,
                    Err(_) => {
                        return Ok(LoginProcessResponse::Template(Template::render(
                            "login",
                            LoginContext {
                                next_host: url.host_str(),
                                error: Some("Auth token generation failed"),
                            },
                        )));
                    }
                };

            // Set authtoken cookie with jwt content
            cookies.add(
                Cookie::build(crate::AUTHTOKEN_COOKIE_NAME, jwt_token)
                    .domain(crate::AUTHTOKEN_COOKIE_DOMAIN)
                    .path("/")
                    .secure(true)
                    .http_only(true)
                    .max_age(time::Duration::days(7))
                    .same_site(rocket::http::SameSite::Strict)
                    .finish(),
            );

            // Redirect to next url
            Ok(LoginProcessResponse::Redirect(Redirect::to(next)))
        }

        // Invalid host
        Err(_) => Err(Template::render("errors/denied", json!({}))),
    }
}

fn has_active_session(cookies: &CookieJar<'_>, public_key: &[u8]) -> bool {
    if let Some(token) = cookies.get(crate::AUTHTOKEN_COOKIE_NAME) {
        if let Ok(token) = crate::utils::decode_jwt(public_key, token.value()) {
            if crate::utils::jwt_duration_is_valid(&token) {
                return true;
            }
        }
    }

    false
}
