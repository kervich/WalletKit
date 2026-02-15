use bip32::DerivationPath;
use std::sync::Arc;
use sui_sdk::SuiClientBuilder;
use sui_types::crypto::SuiKeyPair;

use crate::{
    error::Error,
    runtime::Runtime,
    signature_scheme::SignatureScheme,
    sui_address::SuiAddress,
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

        Ok(Self { address: address.into(), key_pair: Some(key_pair), derivation_path: Some(derivation_path), rpc_url })
    }

    pub fn new_watch_only(
        address: Arc<SuiAddress>,
        rpc_url: String
    ) -> Result<Self, Error> {
        Ok(Self { address: (*address).clone(), key_pair: None, derivation_path: None, rpc_url })
    }

    pub fn address(&self) -> Arc<SuiAddress> {
        self.address.clone().into()
    }

    pub fn derivation_path(&self) -> Option<String> {
        self.derivation_path.as_ref().map(|dp| dp.to_string())
    }

    pub async fn get_balance(&self, coin_type: String) -> Result<Vec<u64>, Error> {
        let runtime = Runtime::new();

        let balance = runtime.runtime.block_on(async {
            let client = SuiClientBuilder::default()
                .build(self.rpc_url.clone())
                .await
                .map_err(|e| Error::SuiError { description: e.to_string() })?;

            let coin_type: Option<String> = Some(coin_type);

            client.coin_read_api()
                .get_balance(self.address.clone().into(), coin_type)
                .await
                .map_err(|e| Error::SuiError { description: e.to_string() })
        })?;

        let amount = alloy_primitives::U256::from(balance.total_balance);
        Ok(amount.as_limbs().to_vec())
    }

    pub async fn is_active_address(&self, address: Arc<SuiAddress>) -> Result<bool, Error> {
        Ok(false) // TODO: Implement this method by checking if the address has any transactions or balance on the Sui blockchain.
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