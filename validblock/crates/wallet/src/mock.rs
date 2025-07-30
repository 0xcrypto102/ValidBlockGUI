use crate::WalletAdapter;

#[derive(Default, Debug, Clone)]
pub struct MockWallet;

impl WalletAdapter for MockWallet {}
