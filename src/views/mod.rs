use rocket::response::Redirect;

mod login;
pub use login::*;

mod logout;
pub use logout::*;

mod status;
pub use status::*;

#[get("/")]
pub fn get_root() -> Redirect {
    // Redirect to status page
    Redirect::to(uri!(get_status()))
}
