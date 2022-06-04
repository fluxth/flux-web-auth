use tonic::{Request, Response, Status};

use crate::error::Error;
use crate::url::validate_redirect_url;

use crate::proto::login_service_server;
use crate::proto::{identity_response, initiate_response, initiate_result, password_response};
use crate::proto::{IdentityRequest, IdentityResponse, IdentityResult};
use crate::proto::{InitiateData, InitiateRequest, InitiateResponse, InitiateResult};
use crate::proto::{PasswordRequest, PasswordResponse, PasswordResult};

pub use crate::proto::login_service_server::LoginServiceServer;

#[derive(Default)]
pub struct LoginService {}

#[tonic::async_trait]
impl login_service_server::LoginService for LoginService {
    async fn initiate(
        &self,
        request: Request<InitiateRequest>,
    ) -> Result<Response<InitiateResponse>, Status> {
        let metadata = request.metadata();
        println!("got initiate: {:?}", metadata);

        let response = InitiateResponse {
            response: match initiate_process(request).await {
                Ok(result) => Some(initiate_response::Response::Result(InitiateResult {
                    result: Some(result),
                })),
                Err(error) => Some(initiate_response::Response::Error(error)),
            },
        };

        Ok(Response::new(response))
    }

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

async fn initiate_process(
    request: Request<InitiateRequest>,
) -> Result<initiate_result::Result, Error> {
    let token = crate::utils::get_token(&request);

    // Validate redirect urls
    let request_data = request.into_inner();
    let continue_url = request_data.continue_url;
    let back_url = {
        if request_data.back_url == "" {
            None
        } else {
            Some(request_data.back_url)
        }
    };

    validate_redirect_url(&continue_url)?;
    if let Some(back_url) = back_url {
        validate_redirect_url(&back_url)?;
    }

    // Has session?
    if token.is_some() {
        return Ok(initiate_result::Result::RedirectUrl(continue_url));
    }

    Ok(initiate_result::Result::Data(InitiateData {
        next_service_name: "fluxauth".into(),
    }))
}
