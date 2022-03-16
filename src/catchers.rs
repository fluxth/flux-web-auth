use rocket::{http::Status, serde::json::Json, Request};
use rocket_dyn_templates::Template;
use serde_json::json;

use crate::views::api;

#[catch(404)]
pub fn not_found(request: &Request) -> Template {
    Template::render(
        "errors/404",
        json!({
            "request_uri": request.uri()
        }),
    )
}

#[catch(default)]
pub fn api_default_catcher(status: Status, _request: &Request) -> Json<api::ErrorResponse> {
    api::ErrorResponse::http_error(status)
}
