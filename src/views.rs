use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[get("/")]
pub fn root() -> Redirect {
    Redirect::to(uri!("/login"))
}

#[derive(Serialize)]
struct LoginContext {}

#[get("/login")]
pub fn login() -> Template {
    Template::render("login", LoginContext {})
}
