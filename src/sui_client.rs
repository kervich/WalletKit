use std::str::FromStr;

use sui_types::base_types::SuiAddress;
use sui_types::crypto::SuiKeyPair;
use bip32::DerivationPath;

use crate::{
    BlockchainClient,
    error::Error,
    signature_scheme::SignatureScheme,
};

pub struct SuiClient {
    address: SuiAddress,
    derivation_path: Option<DerivationPath>,
    key_pair: Option<SuiKeyPair>,
    rpc_url: String
}

impl SuiClient {
    pub fn new(
        account_index: u64,
        key_scheme: SignatureScheme,
        private_key: Vec<u8>,
        password: Option<String>,
        rpc_url: String
    ) -> Result<Self, Error> {
        let derivation_path = SuiClient::make_derivation_path(account_index, &key_scheme)?
            .parse::<DerivationPath>()
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        let (address, key_pair) =
            sui_keys::key_derive::derive_key_pair_from_path(
                &private_key,
                Some(derivation_path.clone()),
                &key_scheme.into(),
            )
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        Ok(Self { address, key_pair: Some(key_pair), derivation_path: Some(derivation_path), rpc_url })
    }

    pub fn new_watch_only(
        address: String,
        rpc_url: String
    ) -> Result<Self, Error> {
        let sui_address = SuiAddress::from_str(&address)
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        Ok(Self { address: sui_address, key_pair: None, derivation_path: None, rpc_url })
    }
}

impl BlockchainClient for SuiClient {
    fn address(&self) -> String {
        self.address.to_string()
    }

    fn derivation_path(&self) -> Option<String> {
        self.derivation_path.as_ref().map(|dp| dp.to_string())
    }

    fn is_active_address(&self, address: String) -> bool {
        false // TODO: Implement this method by checking if the address has any transactions or balance on the Sui blockchain.
    }

    fn is_valid_address(&self, address: String) -> bool {
        SuiAddress::from_str(&address).is_ok()
    }
}

impl SuiClient {
    fn make_derivation_path(account_index: u64, key_scheme: &SignatureScheme) -> Result<String, Error> {
        match key_scheme {
            SignatureScheme::ED25519 => Ok(format!("m/44'/784'/{account_index}'/0'/0'")),
            SignatureScheme::Secp256k1 => Ok(format!("m/44'/784'/{account_index}'/0'/0'")),
            SignatureScheme::Secp256r1 => Ok(format!("m/44'/784'/{account_index}'/0'/0'")),
            _ => Err(Error::NotImplemented),
        }
    }
}