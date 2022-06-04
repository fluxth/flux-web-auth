mod config;
mod error;
mod services;
pub mod proto {
    tonic::include_proto!("flux.web.auth");
}

use config::Config;
use services::authentication::*;
use services::login::*;

use once_cell::sync::OnceCell;
use tonic::transport::Server;

pub static CONFIG: OnceCell<Config> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match envy::from_env::<Config>() {
        Ok(config) => {
            CONFIG.set(config).unwrap();
            CONFIG.get().unwrap()
        }
        Err(error) => panic!("fail to parse config: {}", error),
    };

    let addr = format!("{}:{}", config.host, config.port).parse().unwrap();

    let authentication_service = AuthenticationService::default();
    let login_service = LoginService::default();

    println!("flux-web-auth-backend listening on grpc -> {}", addr);

    Server::builder()
        .add_service(AuthenticationServiceServer::new(authentication_service))
        .add_service(LoginServiceServer::new(login_service))
        .serve(addr)
        .await?;

    Ok(())
}
