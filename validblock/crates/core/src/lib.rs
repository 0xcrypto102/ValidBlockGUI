#![forbid(unsafe_code)]
pub mod proto;
pub mod services;

pub use validblock_types::*;
use validblock_storage::AnchorRepo;
use validblock_wallet::WalletAdapter;
use validblock_hasher::hash_file;
pub struct AnchorEngine<W: WalletAdapter> {
  pub repo: AnchorRepo,
  pub wallet: W,
}

impl<W: WalletAdapter> AnchorEngine<W> {
  pub fn new(repo: AnchorRepo, wallet: W) -> Self {
    Self { repo, wallet }
  }

  /// Anchor a file, store record, call wallet (stub)
  pub fn anchor_file<P: AsRef<std::path::Path>>(
    &self,
    path: P,
    memo_policy: MemoPolicy,
  ) -> Result<AnchorRecord, VBError> {
    let digest = hash_file(&path)?;
    let ts = chrono::Utc::now().timestamp();
    let memo = match memo_policy {
      MemoPolicy::Disabled => None,
      _ => Some(vec![]), // stub: no memo content
    };
    
    let txid = None;
    let rec = AnchorRecord {
      digest: digest.clone(),
      ts,
      memo,
      txid,
    };
    self.repo.insert(&rec)?;
    Ok(rec)
  }

  /// Verify a file, return anchor record if present
  pub fn verify_file<P: AsRef<std::path::Path>>(
    &self,
    path: P,
  ) -> Result<Option<AnchorRecord>, VBError> {
    let digest = hash_file(&path)?;
    self.repo.get(&digest)
  }

  /// check whether a digest exists
  pub fn exist_digest(&self, digest: &Digest256) -> Result<bool, VBError> {
    Ok(self.repo.get(digest)?.is_some())
  }

}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
  use super::*;
  use validblock_types::MemoPolicy;
  use std::fs::File;
  use std::io::Write;
  use tempfile::tempdir;

  struct MockWallet;
  impl WalletAdapter for MockWallet {}

  #[test]
  fn test_anchor_and_verify_happy_path() {
    let repo: AnchorRepo = AnchorRepo::memory().unwrap();
    let wallet: MockWallet = MockWallet;
    let engine: AnchorEngine<MockWallet> = AnchorEngine::new(repo, wallet);
    let dir = tempdir().unwrap();
    let file_path: std::path::PathBuf = dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"hello world").unwrap();
    file.sync_all().unwrap();
    let rec: AnchorRecord = engine.anchor_file(&file_path, MemoPolicy::LocalOnly).unwrap();
    println!("rec: {:?}", rec);
    let found: AnchorRecord = engine.verify_file(&file_path).unwrap().unwrap();
    println!("found: {:?}", found);
    assert_eq!(rec, found);
  }
} 