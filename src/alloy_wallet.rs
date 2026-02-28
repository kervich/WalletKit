use crate::{
    error::Error,
    ethereum_address::EthereumAddress,
};

use alloy::{
    network::{ EthereumWallet, TransactionBuilder, TxSigner },
    rpc::types::TransactionRequest,
    signers::{
        local::{
            PrivateKeySigner,
            coins_bip39::{ English, Entropy, Mnemonic }
        },
        trezor::{ HDPath, TrezorSigner },
    }
};
use bip32::DerivationPath;
use k256::ecdsa::SigningKey;
use std::sync::Arc;

pub struct AlloyWallet {
    derivation_path: DerivationPath,
    pub wallet: EthereumWallet
}

impl AlloyWallet {
    pub fn new(
        account_index: u64,
        private_key: Vec<u8>,
    ) -> Result<Self, Error> {
        let entropy = Entropy::try_from(private_key.as_slice())
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let mnemonic: Mnemonic<English> = Mnemonic::new_from_entropy(entropy);

        let derivation_path: DerivationPath = format!("m/44'/60'/0'/0/{account_index}")
            .parse()
            .map_err(|_| Error::AlloyError { description: "Invalid derivation path".to_string() })?;

        let path: Vec<u32> = derivation_path.clone()
            .into_iter()
            .map(|index| index.into())
            .collect();

        let derived_priv_key = mnemonic.derive_key(&path, None)
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let key: &SigningKey = derived_priv_key.as_ref();

        let credential = SigningKey::from_bytes(&key.to_bytes())
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let signer = PrivateKeySigner::from_signing_key(credential);
        let wallet = EthereumWallet::from(signer.clone());

        Ok(Self {
            derivation_path,
            wallet
        })
    }

    pub async fn new_trezor(
        account_index: u64,
        chain_id: Option<u64>
    ) -> Result<Self, Error> {
        let derivation = HDPath::TrezorLive(account_index.try_into().unwrap());

        let signer = TrezorSigner::new(derivation, chain_id)
            .await
            .map_err(|e| Error::TrezorError { description: e.to_string() })?;

        let derivation_path: DerivationPath = format!("m/44'/60'/0'/0/{account_index}")
            .parse()
            .map_err(|_| Error::AlloyError { description: "Invalid derivation path".to_string() })?;

        let wallet = EthereumWallet::from(signer);

        Ok(Self { derivation_path, wallet })
    }
}

impl AlloyWallet {
    pub fn address(&self) -> Arc<EthereumAddress> {
        Arc::new(self.wallet.default_signer().address().into())
    }

    pub fn derivation_path(&self) -> String {
        self.derivation_path.to_string()
    }

    pub async fn sign(&self, tx_request: Vec<u8>) -> Result<Vec<u8>, Error> {
        let tx_request: TransactionRequest = serde_json::from_slice(&tx_request)
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let tx_envelope = tx_request.build(&self.wallet)
            .await
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        serde_json::to_vec(&tx_envelope)
            .map_err(|e| Error::AlloyError { description: e.to_string() })
    }
}