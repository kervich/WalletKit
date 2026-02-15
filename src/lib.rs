uniffi::include_scaffolding!("lib");

mod alloy_client;
mod erc20;
mod error;
mod ethereum_address;
mod mnemonic;
mod runtime;
mod signature_scheme;
mod sui_address;
mod sui_client;
mod trezor;

use crate::{
    erc20::ERC20,
    error::Error,
    ethereum_address::EthereumAddress,
    alloy_client::AlloyClient,
    mnemonic::Mnemonic,
    signature_scheme::SignatureScheme,
    sui_address::SuiAddress,
    sui_client::SuiClient,
    trezor::Trezor
};