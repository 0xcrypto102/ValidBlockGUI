#![forbid(unsafe_code)]

use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

// ============================================================================
// Core Data Types
// ============================================================================

/// 32-byte digest type
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Digest256(pub [u8; 32]);

/// Anchor record
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct AnchorRecord {
  pub digest: Digest256,
  pub ts: i64,
  pub memo: Option<Vec<u8>>,
  pub txid: Option<String>,
}

/// Memo policy enum
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum MemoPolicy {
  LocalOnly,
  OnChain,
  Disabled,
}

// ============================================================================
// Error Types
// ============================================================================

/// Error enum shared across crates
#[derive(Debug, Error)]
pub enum VBError {
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),
  #[error("Hash error")]
  Hash,
  #[error("Wallet error")]
  Wallet,
  #[error("Database error: {0}")]
  Db(String),
  #[error("Duplicate record")]
  DbDuplicate,
  #[error("Other error: {0}")]
  Other(String),
}

// ============================================================================
// Traits
// ============================================================================

/// ByteLen trait for memo length helpers
pub trait ByteLen {
  fn byte_len(&self) -> usize;
}

impl ByteLen for Vec<u8> {
  fn byte_len(&self) -> usize {
    self.len()
  }
}

impl ByteLen for Option<Vec<u8>> {
  fn byte_len(&self) -> usize {
    self.as_ref().map_or(0, |v| v.len())
  }
}

// ============================================================================
// Implementations
// ============================================================================

impl fmt::Display for Digest256 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for byte in &self.0 {
      write!(f, "{:02x}", byte)?;
    }
    Ok(())
  }
}

impl FromStr for Digest256 {
  type Err = VBError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = s.trim();
    if s.len() != 64 {
      return Err(VBError::Hash);
    }
    let mut bytes = [0u8; 32];
    for (i, byte) in bytes.iter_mut().enumerate() {
      *byte = u8::from_str_radix(&s[2*i..2*i+2], 16).map_err(|_| VBError::Hash)?;
    }
    Ok(Digest256(bytes))
  }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json;

  #[test]
  fn test_digest256_fromstr_and_display() {
    let hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let digest = Digest256::from_str(hex).unwrap();
    assert_eq!(digest.to_string(), hex);
  }

  #[test]
  fn test_digest256_fromstr_invalid() {
    assert!(Digest256::from_str("deadbeef").is_err());
    assert!(Digest256::from_str(&"0".repeat(63)).is_err());
    assert!(Digest256::from_str(&"g".repeat(64)).is_err());
  }

  #[test]
  fn test_serde_roundtrip_digest256() {
    let hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let digest = Digest256::from_str(hex).unwrap();
    let ser = serde_json::to_string(&digest).unwrap();
    let de: Digest256 = serde_json::from_str(&ser).unwrap();
    assert_eq!(digest, de);
  }

  #[test]
  fn test_serde_roundtrip_anchorrecord() {
    let rec = AnchorRecord {
      digest: Digest256([1; 32]),
      ts: 1234567890,
      memo: Some(vec![1,2,3]),
      txid: Some("txid123".to_string()),
    };
    let ser = serde_json::to_string(&rec).unwrap();
    let de: AnchorRecord = serde_json::from_str(&ser).unwrap();
    assert_eq!(rec, de);
  }

  #[test]
  fn test_bytelen_trait() {
    let v = vec![1,2,3,4];
    assert_eq!(v.byte_len(), 4);
    let o = Some(vec![1,2,3]);
    assert_eq!(o.byte_len(), 3);
    let n: Option<Vec<u8>> = None;
    assert_eq!(n.byte_len(), 0);
  }
} 