use anyhow::format_err;
use std::borrow::Borrow;
use std::borrow::Cow;
use tonic::{Request, Response, Status};
use unwrap_or::unwrap_ok_or;

use crate::proto::authentication_service_server;
use crate::proto::status_response;
use crate::proto::Error;
use crate::proto::{StatusResponse, StatusResult};

pub use crate::proto::authentication_service_server::AuthenticationServiceServer;

use crate::utils;

#[derive(Default)]
pub struct AuthenticationService {}

#[tonic::async_trait]
impl authentication_service_server::AuthenticationService for AuthenticationService {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusResponse>, Status> {
        let token = crate::utils::get_token(&request);

        let response = StatusResponse {
            response: Some(match status_process(token) {
                Ok(result) => status_response::Response::Result(result),
                Err(error) => status_response::Response::Error(error),
            }),
        };

        Ok(Response::new(response))
    }
}

fn status_process(token: Option<Cow<str>>) -> Result<StatusResult, Error> {
    let config = crate::CONFIG
        .get()
        .ok_or(format_err!("Config not initialized"))?;

    let not_logged_in_result = || StatusResult {
        logged_in: false,
        display_name: "".into(),
    };

    if let Some(token) = token {
        let jwt = unwrap_ok_or!(
            utils::verify_jwt(&config.jwt_public_key, token.borrow()),
            err,
            {
                println!("jwt verify failed: {:?}", err);
                return Ok(not_logged_in_result());
            }
        );

        if utils::has_valid_session(&jwt)? {
            let sub = jwt.claims().sub.as_str();

            Ok(StatusResult {
                logged_in: true,
                display_name: sub.into(),
            })
        } else {
            Ok(not_logged_in_result())
        }
    } else {
        // No token found, not logged in
        Ok(not_logged_in_result())
    }
}
