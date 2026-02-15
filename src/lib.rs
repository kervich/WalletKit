uniffi::include_scaffolding!("lib");

mod error;
mod alloy_client;
mod sui_client;
mod mnemonic;
mod signature_scheme;
mod trezor;

use crate::{
    error::Error,
    alloy_client::AlloyClient,
    mnemonic::Mnemonic,
    signature_scheme::SignatureScheme,
    sui_client::SuiClient,
    trezor::Trezor
};

pub trait BlockchainClient: Send + Sync {
    fn address(&self) -> String;
    fn derivation_path(&self) -> Option<String>;
    fn is_active_address(&self, address: String) -> bool;
    fn is_valid_address(&self, address: String) -> bool;
}