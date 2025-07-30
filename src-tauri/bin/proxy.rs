use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};
use tower::ServiceBuilder;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use validblock_core::{proto::validblock, AnchorEngine};
use validblock::anchor_service_server::AnchorServiceServer;
use validblock::verify_service_server::VerifyServiceServer;
use validblock_core::services::{AnchorServiceImpl, VerifyServiceImpl};
use validblock_storage::AnchorRepo;
use validblock_wallet::mock::MockWallet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    let repo = AnchorRepo::new(None)?;
    let wallet = MockWallet;
    let engine = Arc::new(Mutex::new(AnchorEngine::new(repo, wallet)));

    let anchor_service = AnchorServiceServer::new(AnchorServiceImpl::new(engine.clone()));
    let verify_service = VerifyServiceServer::new(VerifyServiceImpl::new(engine));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let layer = ServiceBuilder::new()
        .layer(GrpcWebLayer::new()) // 👈 Layer applied here
        .layer(CorsLayer::permissive()); // optional: CORS for browser use

    println!("gRPC-Web proxy running at http://{}", addr);

    Server::builder()
        .accept_http1(true) // 👈 Required for grpc-web
        .layer(layer)
        .add_service(anchor_service)
        .add_service(verify_service)
        .serve(addr)
        .await?;

    Ok(())
}
