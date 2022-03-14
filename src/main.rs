mod catchers;
mod utils;
mod views;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    jwt_private_key: String,
    jwt_public_key: String,
    allowed_next_hosts: Vec<String>,
}

const AUTHTOKEN_COOKIE_DOMAIN: &'static str = "flux.ci";
const AUTHTOKEN_COOKIE_NAME: &'static str = "authtoken";

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
        .manage(config)
        .register("/", catchers![catchers::not_found])
        .mount(
            "/",
            routes![views::get_login, views::post_login, views::get_logout],
        )
        .mount("/static", FileServer::from("./static"))
}
