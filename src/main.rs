mod catchers;
mod fairings;
mod utils;
mod views;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use serde::Deserialize;

const APP_NAME: &'static str = env!("CARGO_CRATE_NAME");
const APP_VERSION: &'static str = git_version::git_describe!();

#[derive(Debug, Deserialize)]
pub struct Config {
    site_host: String,
    jwt_private_key: String,
    jwt_public_key: String,
    authtoken_cookie_name: String,
    authtoken_cookie_domain: String,
    allowed_next_hosts: Vec<String>,
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    let figment = rocket.figment();
    let config: Config = {
        let mut env: Config = figment.extract().expect("config must be valid");
        // Parse as multi-line string
        env.jwt_private_key = env.jwt_private_key.replace("\\n", "\n");
        env.jwt_public_key = env.jwt_public_key.replace("\\n", "\n");
        env
    };

    rocket
        .attach(Template::fairing())
        .attach(fairings::CSRFCookieInjector::new())
        .manage(config)
        .register("/", catchers![catchers::not_found])
        .mount(
            "/",
            routes![
                views::get_root,
                views::get_status,
                views::get_login,
                views::post_login,
                views::get_logout,
            ],
        )
        .register("/api", catchers![catchers::api_default_catcher])
        .mount("/api", routes![views::api::get_info])
        .mount("/static", FileServer::from("./static"))
}
