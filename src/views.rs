use anyhow::{bail, format_err};
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use url::Url;

#[derive(Serialize)]
struct LoginContext {}

fn next_url_is_valid(next: &str) -> anyhow::Result<()> {
    if next.len() == 0 {
        bail!("`next` parameter is required");
    }

    let url = Url::parse(next)?;

    if url.scheme() != "https" {
        bail!("only https scheme is supported");
    }

    let host = url.host_str().ok_or(format_err!("hostname is required"))?;

    if host == "search.flux.ci" {
        Ok(())
    } else {
        Err(format_err!("hostname not allowed"))
    }
}

#[get("/login?<next>")]
pub fn login(next: &str) -> Option<Template> {
    if next_url_is_valid(next).is_err() {
        return None;
    }

    Some(Template::render("login", LoginContext {}))
}

#[post("/login?<next>")]
pub fn login_process(next: String, cookies: &CookieJar<'_>) -> Result<Redirect, String> {
    if let Err(error) = next_url_is_valid(&next) {
        return Err(format!("Redirect URL is sus: {}", error.to_string()));
    }

    cookies.add(
        Cookie::build("authtoken", "1")
            .domain("flux.ci")
            .path("/")
            .secure(true)
            .http_only(true)
            .max_age(time::Duration::days(7))
            .same_site(rocket::http::SameSite::Strict)
            .finish(),
    );
    Ok(Redirect::to(next))
}
