use crate::utils::validate_next_url;
use rocket::http::{Cookie, CookieJar};
use rocket::{serde::Serialize, State};
use rocket_dyn_templates::Template;
use serde_json::json;

#[derive(Serialize)]
struct LogoutContext<'a> {
    next_url: Option<&'a str>,
}

#[get("/logout?<next>")]
pub fn get_logout(
    config: &State<crate::Config>,
    cookies: &CookieJar<'_>,
    next: Option<&str>,
) -> Result<Template, Template> {
    if let Some(next_url) = next {
        if validate_next_url(&next_url, &config).is_err() {
            return Err(Template::render("errors/host_denied", json!({})));
        }
    };

    cookies.remove(Cookie::named(crate::AUTHTOKEN_COOKIE_NAME));

    Ok(Template::render(
        "pages/logout",
        LogoutContext { next_url: next },
    ))
}
