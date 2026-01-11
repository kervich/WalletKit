uniffi::include_scaffolding!("lib");

mod error;
mod evm_keys;
mod keystore;
mod mnemonic;
mod signature_scheme;

use crate::{
    error::Error,
    keystore::KeyStore,
    mnemonic::Mnemonic,
    signature_scheme::SignatureScheme,
};