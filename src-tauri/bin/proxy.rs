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

    // gRPC-Web + CORS layers applied
    let svc = tonic::transport::Server::builder()
        .accept_http1(true) // required for grpc-web
        .layer(
            ServiceBuilder::new()
                .layer(GrpcWebLayer::new())
                // .layer(CorsLayer::permissive()), // Or your custom CORS
        )
        .add_service(anchor_service)
        .add_service(verify_service);

    println!("✅ gRPC-Web proxy listening at http://{}", addr);
    svc.serve(addr).await?;

    Ok(())
}
