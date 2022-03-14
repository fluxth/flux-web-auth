use rocket::Request;
use rocket_dyn_templates::Template;
use serde_json::json;

#[catch(404)]
pub fn not_found(request: &Request) -> Template {
    Template::render(
        "errors/404",
        json!({
            "request_uri": request.uri()
        }),
    )
}
