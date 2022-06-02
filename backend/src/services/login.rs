use tonic::{Request, Response, Status};

use crate::proto::login_service_server;
use crate::proto::{identity_response, password_response};
use crate::proto::{IdentityRequest, IdentityResponse, IdentityResult};
use crate::proto::{PasswordRequest, PasswordResponse, PasswordResult};

pub use crate::proto::login_service_server::LoginServiceServer;

#[derive(Default)]
pub struct LoginService {}

#[tonic::async_trait]
impl login_service_server::LoginService for LoginService {
    async fn identity(
        &self,
        request: Request<IdentityRequest>,
    ) -> Result<Response<IdentityResponse>, Status> {
        let metadata = request.metadata();
        println!("got identity: {:?}", metadata);

        let response = IdentityResponse {
            response: Some(identity_response::Response::Result(IdentityResult {
                identifier: [0x42, 0x42].into(),
                display_name: "user0x42".into(),
            })),
        };

        Ok(Response::new(response))
    }

    async fn password(
        &self,
        request: Request<PasswordRequest>,
    ) -> Result<Response<PasswordResponse>, Status> {
        let metadata = request.metadata();
        println!("got password: {:?}", metadata);

        let response = PasswordResponse {
            response: Some(password_response::Response::Result(PasswordResult {
                authtoken: "test1234".into(),
            })),
        };

        Ok(Response::new(response))
    }
}
