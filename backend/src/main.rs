use tonic::transport::Server;

pub mod proto {
    tonic::include_proto!("flux.web.auth");
}

mod services;
use services::authentication::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:9090".parse().unwrap();

    let authentication_service = AuthenticationService::default();

    println!("flux-web-auth-backend listening on grpc -> {}", addr);

    Server::builder()
        .add_service(AuthenticationServiceServer::new(authentication_service))
        .serve(addr)
        .await?;

    Ok(())
}
