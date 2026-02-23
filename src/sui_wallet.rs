use bip32::DerivationPath;
use std::sync::Arc;
use sui_types::crypto::SuiKeyPair;

use crate::{
    error::Error,
    signature_scheme::SignatureScheme,
    sui_address::SuiAddress,
};

pub struct SuiWallet {
    address: SuiAddress,
    _key_pair: SuiKeyPair,
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

        let (address, key_pair) = sui_keys::key_derive::derive_key_pair_from_path(
            &private_key,
            Some(derivation_path.clone()),
            &key_scheme.into(),
        )
        .map_err(|e| Error::SuiError { description: e.to_string() })?;

        Ok(Self {
            address: address.into(),
            _key_pair: key_pair,
            derivation_path: derivation_path,
        })
    }
}

impl SuiWallet {
    pub fn address(&self) -> Arc<SuiAddress> {
        Arc::new(self.address.clone())
    }

    pub fn derivation_path(&self) -> String {
        self.derivation_path.to_string()
    }
}

impl SuiWallet {
    fn make_derivation_path(
        account_index: u64,
        key_scheme: &SignatureScheme
    ) -> Result<String, Error> {
        match key_scheme {
            SignatureScheme::ED25519 => Ok(format!("m/44'/784'/{account_index}'/0'/0'")),
            SignatureScheme::Secp256k1 => Ok(format!("m/44'/784'/{account_index}'/0'/0'")),
            SignatureScheme::Secp256r1 => Ok(format!("m/44'/784'/{account_index}'/0'/0'")),
            _ => Err(Error::NotImplemented),
        }
    }
}