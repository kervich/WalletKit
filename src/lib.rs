uniffi::include_scaffolding!("lib");

mod error;
mod keystore;
mod mnemonic;
mod signature_scheme;

use crate::{
    error::Error,
    keystore::KeyStore,
    mnemonic::Mnemonic,
    signature_scheme::SignatureScheme,
};