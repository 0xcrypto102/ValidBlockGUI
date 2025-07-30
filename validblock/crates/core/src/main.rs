use tonic::{transport::Server, Request, Response, Status};
use validblock_types::{Digest256, AnchorRecord};
use validblock_hasher::hash_reader;
use validblock_storage::AnchorRepo;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;

// pub mod validblock {
//     tonic::include_proto!("validblock");
// }
use validblock_core::proto::validblock::{
    anchor_service_server::{AnchorService, AnchorServiceServer},
    AnchorRequest, AnchorResponse,
};

// use validblock::anchor_service_server::{AnchorService, AnchorServiceServer};
// use validblock::{AnchorRequest, AnchorResponse};

#[derive(Debug)]
pub struct AnchorServer {
    repo: Arc<Mutex<AnchorRepo>>,
}

#[tonic::async_trait]
impl AnchorService for AnchorServer {
    async fn anchor(
        &self,
        request: Request<AnchorRequest>,
    ) -> Result<Response<AnchorResponse>, Status> {
        let req = request.into_inner();

        // Hash file contents
        let digest = hash_reader(&*req.file_content)
            .map_err(|e| Status::internal(format!("Hash error: {}", e)))?;

        // Insert to DB
        let record = AnchorRecord {
            digest: digest.clone(),
            ts: chrono::Utc::now().timestamp(),
            memo: None,
            txid: None,
        };
        let repo = self.repo.lock().await;
        repo.insert(&record).map_err(|e| Status::internal(format!("DB error: {:?}", e)))?;

        Ok(Response::new(AnchorResponse {
            digest: digest.to_string(),
            timestamp: record.ts,
            txid: "".into(), // On-chain stub for now
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Arc::new(Mutex::new(AnchorRepo::new(None)?));
    let svc = AnchorServiceServer::new(AnchorServer { repo });

    println!("Serving gRPC on 127.0.0.1:50051");
    Server::builder()
        .add_service(svc)
        .serve(([127, 0, 0, 1], 50051).into())
        .await?;

    Ok(())
}
