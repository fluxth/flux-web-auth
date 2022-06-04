use tonic::{Request, Response, Status};

use crate::proto::authentication_service_server;
use crate::proto::status_response;
use crate::proto::{StatusResponse, StatusResult};

pub use crate::proto::authentication_service_server::AuthenticationServiceServer;

#[derive(Default)]
pub struct AuthenticationService {}

#[tonic::async_trait]
impl authentication_service_server::AuthenticationService for AuthenticationService {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusResponse>, Status> {
        let token = crate::utils::get_token(&request);

        let result = match token {
            Some(_token) => StatusResult {
                logged_in: true,
                display_name: "flux".into(),
            },
            None => StatusResult {
                logged_in: false,
                display_name: "".into(),
            },
        };

        let response = StatusResponse {
            response: Some(status_response::Response::Result(result)),
        };

        Ok(Response::new(response))
    }
}
