use bip39::Mnemonic;
use sui_keys::key_derive::derive_key_pair_from_path;
use uuid::Uuid;

use crate::{
    signature_scheme::SignatureScheme,
    error::Error,
};

pub struct KeyStore {
    pub id: String,
    seed: [u8; 64],
}

impl KeyStore {
    pub fn new(entropy: Vec<u8>, passphrase: Option<String>) -> Result<Self, Error> {
        let mnemonic = Mnemonic::from_entropy(&entropy)
            .map_err(|e| Error::InvalidData { description: e.to_string() })?;

        let seed = mnemonic
            .to_seed(passphrase.unwrap_or_default());

        let id = Uuid::new_v4().to_string();

        Ok(KeyStore { id, seed: seed })
    }

    pub fn new_from_mnemonic(
        mnemonic: String,
        passphrase: Option<String>,
    ) -> Result<Self, Error> {
        let mnemonic = Mnemonic::parse(&mnemonic)
            .map_err(|e| Error::InvalidMnemonic { description: e.to_string() })?;

        let seed_bytes = mnemonic
            .to_seed(passphrase.as_deref().unwrap_or_default());

        let seed: [u8; 64] = seed_bytes[..64].try_into()
            .map_err(|_| Error::invalid_seed_length(seed_bytes.len()))?;

        let id = Uuid::new_v4().to_string();

        Ok(KeyStore { id, seed })
    }

    pub fn to_storage_string(&self) -> String {
        let seed_hex = hex::encode(self.seed);
        format!("{}:{}", self.id, seed_hex)
    }

    pub fn from_storage_string(storage_string: String) -> Result<Self, Error> {
        let parts: Vec<&str> = storage_string.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::InvalidData { description: "Invalid storage string format".to_string() });
        }

        let id = parts[0].to_string();
        let seed_hex = parts[1];

        let seed_bytes = hex::decode(seed_hex)
            .map_err(|e| Error::InvalidData { description: e.to_string() })?;

        let seed: [u8; 64] = seed_bytes[..64].try_into()
            .map_err(|_| Error::invalid_seed_length(seed_bytes.len()))?;

        Ok(KeyStore { id, seed })
    }

    pub fn preview() -> Self {
        KeyStore {
            id: Uuid::new_v4().to_string(),
            seed: [0u8; 64],
        }
    }

    pub async fn get_evm_address(
        &self,
        derivation_path: String,
    ) -> Result<String, Error> {
        /* let derivation_path = derivation_path
            .parse::<bip32::DerivationPath>()
            .map_err(|e| Error::InvalidDerivationPath { description: e.to_string() })?; */

        Err(Error::NotImplemented)
    }

    pub async fn get_sui_address(
        &self,
        derivation_path: String,
        key_scheme: SignatureScheme
    ) -> Result<String, Error> {
        let derivation_path = derivation_path
            .parse::<bip32::DerivationPath>()
            .map_err(|e| Error::InvalidDerivationPath { description: e.to_string() })?;

        let (address, _keypair) =
            derive_key_pair_from_path(
                &self.seed,
                Some(derivation_path),
                &key_scheme.into(),
            )
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        Ok(address.to_string())
    }
}
