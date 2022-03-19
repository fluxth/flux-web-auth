use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::Template;
use serde_json::json;

use crate::models;
use crate::utils::{
    decode_jwt, generate_jwt, get_csrf_token, jwt_duration_is_valid, validate_next_url, CSRFToken,
};

const ERROR_MSG_LOGIN_FAILED: &'static str = "Invalid username or password";

#[derive(Serialize)]
struct LoginContext<'a> {
    next_host: Option<&'a str>,
    error: Option<&'a str>,
    csrf_token: &'a CSRFToken<'a>,
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
                    csrf_token: &get_csrf_token(cookies),
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
pub async fn post_login(
    database: crate::AuthDatabase,
    config: &State<crate::Config>,
    cookies: &CookieJar<'_>,
    next: String,
    form: Form<LoginForm<'_>>,
) -> Result<LoginProcessResponse, Template> {
    match validate_next_url(&next, &config) {
        Ok(url) => {
            let csrf_token = get_csrf_token(cookies);
            let render_login_page = |error: Option<&str>| {
                let token = &csrf_token;
                Template::render(
                    "pages/login",
                    LoginContext {
                        next_host: url.host_str(),
                        error,
                        csrf_token: token,
                    },
                )
            };

            // Check CSRF token
            if csrf_token.as_str() != form.csrf_token {
                return Ok(LoginProcessResponse::Template(render_login_page(Some(
                    "CSRF Token Mismatch",
                ))));
            }

            // Check existing session
            let authtoken_cookie = cookies.get(config.authtoken_cookie_name.as_str());
            if has_active_session(authtoken_cookie, config.jwt_public_key.as_bytes()) {
                return Ok(LoginProcessResponse::Redirect(Redirect::to(next)));
            }

            // Authenticate user
            println!("User '{}' tried logging in", form.username);

            let username = form.username.to_string();
            let user = match database
                .run(move |conn| models::User::find_username(conn, &username))
                .await
            {
                Ok(user) => user,
                Err(_) => {
                    eprintln!("User '{}' failed to login: No such user", form.username);
                    return Ok(LoginProcessResponse::Template(render_login_page(Some(
                        ERROR_MSG_LOGIN_FAILED,
                    ))));
                }
            };

            let hash = match PasswordHash::new(&user.password) {
                Ok(hash) => hash,
                Err(_) => {
                    return Err(render_login_page(Some(
                        "Password corrupted, please reset your password",
                    )))
                }
            };

            if let Err(error) = Argon2::default().verify_password(form.password.as_bytes(), &hash) {
                eprintln!(
                    "User '{}' failed to login: Verify password failed: {:?}",
                    form.username, &error
                );

                let error_message = match error {
                    argon2::password_hash::Error::Password => "Invalid username or password",
                    _ => "An unknown error occurred",
                };

                // Wrong password
                return Ok(LoginProcessResponse::Template(render_login_page(Some(
                    error_message,
                ))));
            };

            let jwt_token = match generate_jwt(config.jwt_private_key.as_bytes(), form.username) {
                Ok(token) => token,
                Err(error) => {
                    eprintln!(
                        "User '{}' failed to login: AuthToken generation failed: {:?}",
                        form.username, &error
                    );

                    return Ok(LoginProcessResponse::Template(render_login_page(Some(
                        "Auth token generation failed",
                    ))));
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
