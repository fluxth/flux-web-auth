use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Cookie, SameSite, Status};
use rocket::{Request, Response};

pub struct CSRFCookieInjector {
    cookie_name: &'static str,
}

impl CSRFCookieInjector {
    pub fn new() -> Self {
        Self {
            cookie_name: "csrftoken",
        }
    }
}

#[rocket::async_trait]
impl Fairing for CSRFCookieInjector {
    fn info(&self) -> Info {
        Info {
            name: "CSRF Cookie Injector",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Only enable on 200 response pages
        if response.status() != Status::Ok {
            return;
        }

        // Skip if there is already a csrf cookie
        if request.cookies().get(self.cookie_name).is_some() {
            return;
        }

        let csrf_token: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let csrf_cookie = Cookie::build(self.cookie_name, csrf_token)
            .path("/")
            .secure(true)
            .same_site(SameSite::Strict)
            .finish();

        response.set_header(csrf_cookie);
    }
}
