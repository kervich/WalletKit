uniffi::include_scaffolding!("lib");

mod address_validator;
mod error;
mod evm_keys;
mod keystore;
mod mnemonic;
mod signature_scheme;

use crate::{
    address_validator::AddressValidator,
    error::Error,
    keystore::KeyStore,
    mnemonic::Mnemonic,
    signature_scheme::SignatureScheme,
};