mod info;
pub use info::*;

use rocket::{http::Status, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<D>
where
    D: Serialize,
{
    status: &'static str,
    data: D,
}

impl<D> SuccessResponse<D>
where
    D: Serialize,
{
    pub fn new(data: D) -> Json<Self> {
        Json(Self {
            status: "success",
            data,
        })
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    status: &'static str,
    r#type: &'static str,
    code: u16,
    message: &'static str,
}

impl ErrorResponse {
    pub fn http_error(status: Status) -> Json<Self> {
        Json(Self {
            status: "error",
            r#type: "http",
            code: status.code,
            message: status.reason().unwrap_or("Unknown error"),
        })
    }
}
