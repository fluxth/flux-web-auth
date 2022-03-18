use rocket::serde::json::Json;
use serde::Serialize;

use super::SuccessResponse;

#[derive(Serialize)]
pub struct ServerInfo {
    server: &'static str,
    version: &'static str,
}

#[get("/info")]
pub fn get_info() -> Json<SuccessResponse<ServerInfo>> {
    SuccessResponse::new(ServerInfo {
        server: crate::APP_NAME,
        version: crate::APP_VERSION,
    })
}
