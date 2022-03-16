mod info;
pub use info::*;

use rocket::serde::json::Json;
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
