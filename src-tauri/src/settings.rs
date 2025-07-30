use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SettingsStore {
    pub rpc_endpoint: String,
    pub default_policy: String, // "OnChain" or "LocalOnly"
    pub wallet_id: String,
    pub trinity_mode: bool,
}

impl Default for SettingsStore {
    fn default() -> Self {
        Self {
            rpc_endpoint: "http://127.0.0.1:8080".into(),
            default_policy: "OnChain".into(),
            wallet_id: "".into(),
            trinity_mode: false,
        }
    }
}
