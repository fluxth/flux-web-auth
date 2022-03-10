mod views;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![views::login, views::login_process])
        .mount("/static", FileServer::from("./static"))
}
