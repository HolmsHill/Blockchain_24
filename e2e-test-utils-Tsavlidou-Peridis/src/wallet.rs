use alloy_signer::Signer;
use alloy_signer_local::{coins_bip39::English, MnemonicBuilder, PrivateKeySigner};

/// Represents a wallet with a private key and related information.
pub struct Wallet {
    pub inner: PrivateKeySigner,
    pub inner_nonce: u64,
    pub chain_id: u64,
    amount: usize,
    derivation_path: Option<String>,
}

impl Wallet {
    /// Creates a new wallet with a specified amount using a predefined mnemonic.
    pub fn new(amount: usize) -> Self {
        let inner = MnemonicBuilder::<English>::default().phrase(TEST_MNEMONIC).build().unwrap();
        Self { inner, chain_id: 1, amount, derivation_path: None, inner_nonce: 0 }
    }

    /// Sets the chain ID for the wallet.
    pub fn with_chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = chain_id;
        self
    }

    /// Returns the derivation path or a default value.
    fn get_derivation_path(&self) -> &str {
        self.derivation_path.as_deref().unwrap_or("m/44'/60'/0'/0/")
    }

    /// Generates a vector of wallets based on the amount.
    pub fn gen(&self) -> Vec<PrivateKeySigner> {
        let builder = MnemonicBuilder::<English>::default().phrase(TEST_MNEMONIC);
        let derivation_path = self.get_derivation_path();

        let mut wallets = Vec::with_capacity(self.amount);
        for idx in 0..self.amount {
            let builder =
                builder.clone().derivation_path(&format!("{derivation_path}{idx}")).unwrap();
            let wallet = builder.build().unwrap().with_chain_id(Some(self.chain_id));
            wallets.push(wallet)
        }
        wallets
    }
}

/// A predefined mnemonic for testing.
const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";

impl Default for Wallet {
    /// Creates a default wallet with one account.
    fn default() -> Self {
        Wallet::new(1)
    }
}