mod catchers;
mod models;
mod schema;
mod utils;
mod views;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use serde::Deserialize;

const APP_NAME: &'static str = "flux-web-auth";
const APP_VERSION: &'static str = git_version::git_describe!();

const CSRF_TOKEN_COOKIE_NAME: &'static str = "csrftoken";
const CSRF_TOKEN_LENGTH: usize = 32;

#[derive(Debug, Deserialize)]
pub struct Config {
    site_host: String,
    jwt_private_key: String,
    jwt_public_key: String,
    authtoken_cookie_name: String,
    authtoken_cookie_domain: String,
    allowed_next_hosts: Vec<String>,
}

#[database("auth_db")]
pub struct AuthDatabase(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    println!("Starting {} v{}...", APP_NAME, APP_VERSION);

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
        .attach(AuthDatabase::fairing())
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
