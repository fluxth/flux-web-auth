use tonic::{Request, Response, Status};

use crate::proto::authentication_service_server;
use crate::proto::{LoginRequest, LoginResult, StatusRequest, StatusResult};

pub use crate::proto::authentication_service_server::AuthenticationServiceServer;

#[derive(Default)]
pub struct AuthenticationService {}

#[tonic::async_trait]
impl authentication_service_server::AuthenticationService for AuthenticationService {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResult>, Status> {
        println!("got login: {:?}", request.metadata());

        let result = LoginResult {
            result: "yes".to_owned(),
        };

        Ok(Response::new(result))
    }

    async fn status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResult>, Status> {
        let metadata = request.metadata();
        println!("got status: {:?}", metadata);

        let result = StatusResult {
            logged_in: true,
            display_name: "flux".to_owned(),
        };

        Ok(Response::new(result))
    }
}
