use tonic::{Request, Response, Status};
use crate::{AnchorEngine};
use crate::proto::{
    anchor_service_server::AnchorService,
    AnchorRequest, AnchorResponse,
};
use crate::proto::{
    verify_service_server::VerifyService,
    VerifyRequest, VerifyResponse,
    ExistDigestRequest, ExistDigestResponse, 
};
use validblock_storage::AnchorRepo;
use validblock_wallet::WalletAdapter;
use std::sync::Arc;
use tokio::sync::Mutex;
use validblock_types::Digest256; 

pub struct AnchorServiceImpl<W: WalletAdapter + Send + Sync + 'static> {
    engine: Arc<Mutex<AnchorEngine<W>>>,
}

impl<W: WalletAdapter + Send + Sync + 'static> AnchorServiceImpl<W> {
    pub fn new(engine: Arc<Mutex<AnchorEngine<W>>>) -> Self {
        Self { engine }
    }
}

#[tonic::async_trait]
impl<W: WalletAdapter + Send + Sync + 'static> AnchorService for AnchorServiceImpl<W> {
    async fn anchor(
        &self,
        request: Request<AnchorRequest>,
    ) -> Result<Response<AnchorResponse>, Status> {
        let req = request.into_inner();
        let engine = self.engine.lock().await;

        // Parse file content and memo policy
        let file_bytes = req.file_content;
        let path = std::env::temp_dir().join("tmpfile.dat");
        std::fs::write(&path, &file_bytes).map_err(|e| {
            Status::internal(format!("Failed to write file: {}", e))
        })?;

        let policy = crate::MemoPolicy::LocalOnly; // Use logic to map if needed

        let record = engine
            .anchor_file(&path, policy)
            .map_err(|e| Status::internal(format!("Anchor failed: {}", e)))?;

        Ok(Response::new(AnchorResponse {
            digest: record.digest.to_string(),
            timestamp: record.ts,
            txid: record.txid.unwrap_or_default(),
        }))
    }
}

pub struct VerifyServiceImpl<W: WalletAdapter + Send + Sync + 'static> {
    engine: Arc<Mutex<AnchorEngine<W>>>,
}

impl<W: WalletAdapter + Send + Sync + 'static> VerifyServiceImpl<W> {
    pub fn new(engine: Arc<Mutex<AnchorEngine<W>>>) -> Self {
        Self { engine }
    }
}

#[tonic::async_trait]
impl<W: WalletAdapter + Send + Sync + 'static> VerifyService for VerifyServiceImpl<W> {
    async fn verify(
        &self,
        request: Request<VerifyRequest>,
    ) -> Result<Response<VerifyResponse>, Status> {
        let req = request.into_inner();
        let file_bytes = req.file_content;
        let path = std::env::temp_dir().join("verify_tmp.dat");
        std::fs::write(&path, &file_bytes).map_err(|e| {
            Status::internal(format!("Failed to write file: {}", e))
        })?;

        let engine = self.engine.lock().await;
        let maybe_rec = engine
            .verify_file(&path)
            .map_err(|e| Status::internal(format!("Verify failed: {}", e)))?;

        if let Some(record) = maybe_rec {
            Ok(Response::new(VerifyResponse {
                verified: true,
                digest: record.digest.to_string(),
                timestamp: record.ts,
                txid: record.txid.unwrap_or_default(),
            }))
        } else {
            Err(Status::not_found("Record not found"))
        }
    }

    async fn exist_digest(
        &self,
        request: Request<ExistDigestRequest>,
    ) -> Result<Response<ExistDigestResponse>, Status> {
        let req = request.into_inner();
        let digest_str = req.digest;
    
        // Try to parse digest (base64 or hex as string)
        let digest = digest_str
            .parse::<Digest256>()
            .map_err(|e| Status::invalid_argument(format!("Invalid digest format: {}", e)))?;
    
        let engine = self.engine.lock().await;
        let exists = engine.repo.get(&digest).map(|opt| opt.is_some())
            .map_err(|e| Status::internal(format!("Repo lookup failed: {}", e)))?;
    
        Ok(Response::new(ExistDigestResponse { exists }))
    }
    
}