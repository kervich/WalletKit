use sui_keys::{
    key_derive::derive_key_pair_from_path,
    keystore::{
        AccountKeystore,
    },
};

use crate::{
    sui::{
        runtime::Runtime,
        signature_scheme::SignatureScheme,
    },
    types::error::Error,
};

pub struct Keystore {
}

impl Keystore {
    pub fn new() -> Self {
        Keystore {}
    }

    pub async fn add_key(
        &self,
        alias: String,
        seed: Vec<u8>,
        derivation_path: String,
        key_scheme: SignatureScheme,
    ) -> Result<(), Error> {
        let derivation_path = derivation_path
            .parse::<bip32::DerivationPath>()
            .map_err(|e| Error::InvalidDerivationPath { description: e.to_string() })?;

        let (_address, keypair) =
            derive_key_pair_from_path(
                &seed,
                Some(derivation_path),
                &key_scheme.into(),
            )
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        Runtime::shared()
            .keystore
            .import(Some(alias), keypair)
            .await
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        Ok(())
    }
}
