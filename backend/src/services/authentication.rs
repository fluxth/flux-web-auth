use tonic::{Request, Response, Status};

use crate::proto::authentication_service_server;
use crate::proto::{LoginRequest, LoginResult};

pub use crate::proto::authentication_service_server::AuthenticationServiceServer;

#[derive(Default)]
pub struct AuthenticationService {}

#[tonic::async_trait]
impl authentication_service_server::AuthenticationService for AuthenticationService {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResult>, Status> {
        println!("got login: {:?}", request.metadata());

        let reply = LoginResult {
            result: "yes".to_owned(),
        };

        Ok(Response::new(reply))
    }
}
