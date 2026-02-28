use crate::{
    error::Error,
    signature_scheme::SignatureScheme,
    sui_address::SuiAddress,
};

use alloy::signers::local::coins_bip39::{ English, Entropy, Mnemonic };
use bip32::DerivationPath;
use fastcrypto::hash::HashFunction;
use sui_types::crypto::Signer;
use std::sync::Arc;
use sui_sdk::{
    types::transaction::TransactionData,
    types::crypto::SuiKeyPair
};
use shared_crypto::intent::{Intent, IntentMessage};

pub struct SuiWallet {
    address: SuiAddress,
    key_pair: SuiKeyPair,
    derivation_path: DerivationPath,
}

impl SuiWallet {
    pub fn new(
        account_index: u64,
        key_scheme: SignatureScheme,
        private_key: Vec<u8>
    ) -> Result<Self, Error> {
        let derivation_path = SuiWallet::make_derivation_path(account_index, &key_scheme)?
            .parse::<DerivationPath>()
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

                let entropy = Entropy::try_from(private_key.as_slice())
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let mnemonic: Mnemonic<English> = Mnemonic::new_from_entropy(entropy);

        let seed = mnemonic.to_seed(None)
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        let (address, key_pair) = sui_keys::key_derive::derive_key_pair_from_path(
                &seed,
                Some(derivation_path.clone()),
                &key_scheme.into(),
            )
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        Ok(Self { address: address.into(), key_pair, derivation_path })
    }
}

impl SuiWallet {
    pub fn address(&self) -> Arc<SuiAddress> {
        Arc::new(self.address.clone())
    }

    pub fn derivation_path(&self) -> String {
        self.derivation_path.to_string()
    }

    pub fn sign(&self, tx_data: Vec<u8>) -> Result<Vec<u8>, Error> {
        let tx_data: TransactionData = serde_json::from_slice(&tx_data)
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data);
        let raw_tx = bcs::to_bytes(&intent_msg).expect("bcs should not fail");

        let mut hasher = sui_types::crypto::DefaultHash::default();
        hasher.update(raw_tx.clone());

        let digest = hasher.finalize().digest;
        let signature = self.key_pair.sign(&digest);

        /* let res = sui_sig.verify_secure(
            &intent_msg,
            sender.into(),
            sui_types::crypto::SignatureScheme::ED25519,
        );
        assert!(res.is_ok()); */

        serde_json::to_vec(&signature)
            .map_err(|e| Error::AlloyError { description: e.to_string() })
    }
}

impl SuiWallet {
    fn make_derivation_path(
        account_index: u64,
        key_scheme: &SignatureScheme
    ) -> Result<String, Error> {
        match key_scheme {
            SignatureScheme::ED25519 => Ok(format!("m/44'/784'/{account_index}'/0'/0'")),
            SignatureScheme::Secp256k1 => Ok(format!("m/54'/784'/{account_index}'/0/0")),
            SignatureScheme::Secp256r1 => Ok(format!("m/74'/784'/{account_index}'/0/0")),
            _ => Err(Error::NotImplemented),
        }
    }
}