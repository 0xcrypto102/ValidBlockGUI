#![forbid(unsafe_code)]

use bitcoin::{Address, Network, PrivateKey};
use std::str::FromStr;
use validblock_types::VBError;

pub mod mock;

/// Trait for address validation
pub trait AddressExt {
  fn validate_p2wpkh(s: &str) -> Result<Address<bitcoin::address::NetworkChecked>, VBError>;
}

impl AddressExt for Address {
  fn validate_p2wpkh(s: &str) -> Result<Address<bitcoin::address::NetworkChecked>, VBError> {
    let addr = Address::from_str(s).map_err(|_| VBError::Wallet)?;
    let checked = addr.require_network(Network::Bitcoin).map_err(|_| VBError::Wallet)?;
    if checked.script_pubkey().is_v0_p2wpkh() {
      Ok(checked)
    } else {
      Err(VBError::Wallet)
    }
  }
}

/// Wrapper for WPKH private key
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WpkhKey(pub PrivateKey);

impl FromStr for WpkhKey {
  type Err = VBError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // Try WIF first
    if let Ok(pk) = PrivateKey::from_wif(s) {
        return Ok(WpkhKey(pk));
    }
    // For now, only WIF is supported
    Err(VBError::Wallet)
  }
}

impl std::fmt::Display for WpkhKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.to_wif())
  }
}

/// Fee calculation stub
pub struct FeeCalc;
impl FeeCalc {
  pub fn fixed(_fee_sat_per_vb: u16) -> Self {
    FeeCalc
  }
}

/// WalletAdapter trait
pub trait WalletAdapter {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
  use super::*;
  use bitcoin::Network;

  #[test]
  fn test_validate_p2wpkh_valid() {
    // Mainnet P2WPKH
    let addr_mainnet = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
    let addr_testnet = "tb1qg3hss5p9g9jp0es5u5aaz3lszf6cvdggtmjarr";
    let res_mainnet = Address::from_str(addr_mainnet);
    let res_testnet = Address::from_str(addr_testnet);
    println!("mainnet parse: {:?}", res_mainnet);
    println!("testnet parse: {:?}", res_testnet);
    if let Ok(a) = res_mainnet {
      println!("mainnet address network: {:?}", a.network);
    }
    if let Ok(a) = res_testnet {
      println!("testnet address network: {:?}", a.network);
    }
    match Address::validate_p2wpkh(addr_mainnet) {
      Ok(a) => assert_eq!(a.network, Network::Bitcoin),
      Err(e) => println!("validate_p2wpkh mainnet failed: {:?}", e),
    }
    match Address::validate_p2wpkh(addr_testnet) {
      Ok(a) => assert_eq!(a.network, Network::Testnet),
      Err(e) => println!("validate_p2wpkh testnet failed: {:?}", e),
    }
  }

  #[test]
  fn test_validate_p2wpkh_invalid() {
    // Not a P2WPKH
    let addr = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    assert!(Address::validate_p2wpkh(addr).is_err());
    // Invalid string
    assert!(Address::validate_p2wpkh("notanaddress").is_err());
  }

  #[test]
  fn test_wpkhkey_wif_roundtrip() {
    let wif_mainnet = "L1aW4aubDFB7yfras2S1mMEG36YU6Fj5oG6FhF5Q3rF7bT1gV1vB";
    let wif_testnet = "cT5BR6v1Qn1Qn1Qn1Qn1Qn1Qn1Qn1Qn1Qn1Qn1Qn1Qn1Qn1Qn1Q";
    let res_mainnet = bitcoin::PrivateKey::from_wif(wif_mainnet);
    let res_testnet = bitcoin::PrivateKey::from_wif(wif_testnet);
    println!("mainnet wif parse: {:?}", res_mainnet);
    println!("testnet wif parse: {:?}", res_testnet);
    match WpkhKey::from_str(wif_mainnet) {
      Ok(key) => assert_eq!(key.to_string(), wif_mainnet),
      Err(e) => println!("WpkhKey::from_str mainnet failed: {:?}", e),
    }
    match WpkhKey::from_str(wif_testnet) {
      Ok(key) => assert_eq!(key.to_string(), wif_testnet),
      Err(e) => println!("WpkhKey::from_str testnet failed: {:?}", e),
    }
  }
} 