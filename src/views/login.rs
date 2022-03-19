use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::Template;
use serde_json::json;

use crate::utils::{
    decode_jwt, generate_jwt, get_csrf_token, jwt_duration_is_valid, validate_next_url, CSRFToken,
};

#[derive(Serialize)]
struct LoginContext<'a> {
    next_host: Option<&'a str>,
    error: Option<&'a str>,
    csrf_token: CSRFToken<'a>,
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
            let authtoken_cookie = cookies.get(config.authtoken_cookie_name.as_str());
            if has_active_session(authtoken_cookie, config.jwt_public_key.as_bytes()) {
                return Ok(LoginResponse::Redirect(Redirect::to(next)));
            }

            // Render login form
            Ok(LoginResponse::Template(Template::render(
                "pages/login",
                LoginContext {
                    next_host: url.host_str(),
                    error: None,
                    csrf_token: get_csrf_token(cookies),
                },
            )))
        }

        // Invalid host
        Err(_) => Err(Template::render("errors/host_denied", json!({}))),
    }
}

#[derive(Responder)]
pub enum LoginProcessResponse {
    Redirect(Redirect),
    Template(Template),
}

#[derive(FromForm)]
pub struct LoginForm<'a> {
    username: &'a str,
    password: &'a str,
    csrf_token: &'a str,
}

#[post("/login?<next>", data = "<form>")]
pub fn post_login(
    config: &State<crate::Config>,
    cookies: &CookieJar<'_>,
    next: String,
    form: Form<LoginForm<'_>>,
) -> Result<LoginProcessResponse, Template> {
    match validate_next_url(&next, &config) {
        Ok(url) => {
            // Check CSRF token
            let token = get_csrf_token(cookies);
            if token.as_str() != form.csrf_token {
                return Ok(LoginProcessResponse::Template(Template::render(
                    "pages/login",
                    LoginContext {
                        next_host: url.host_str(),
                        error: Some("CSRF Token Mismatch"),
                        csrf_token: token,
                    },
                )));
            }

            // Check existing session
            let authtoken_cookie = cookies.get(config.authtoken_cookie_name.as_str());
            if has_active_session(authtoken_cookie, config.jwt_public_key.as_bytes()) {
                return Ok(LoginProcessResponse::Redirect(Redirect::to(next)));
            }

            // TODO: Authenticate

            let jwt_token = match generate_jwt(config.jwt_private_key.as_bytes(), "anon") {
                    Ok(token) => token,
                    Err(_) => {
                        return Ok(LoginProcessResponse::Template(Template::render(
                            "pages/login",
                            LoginContext {
                                next_host: url.host_str(),
                                error: Some("Auth token generation failed"),
                                csrf_token: get_csrf_token(cookies),
                            },
                        )));
                    }
                };

            // Set authtoken cookie with jwt content
            cookies.add(
                Cookie::build(config.authtoken_cookie_name.clone(), jwt_token)
                    .domain(config.authtoken_cookie_domain.clone())
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
        Err(_) => Err(Template::render("errors/host_denied", json!({}))),
    }
}

fn has_active_session(authtoken_cookie: Option<&Cookie>, public_key: &[u8]) -> bool {
    if let Some(token) = authtoken_cookie {
        if let Ok(token) = decode_jwt(public_key, token.value()) {
            if jwt_duration_is_valid(&token) {
                return true;
            }
        }
    }

    false
}
