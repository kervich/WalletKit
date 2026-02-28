uniffi::include_scaffolding!("lib");

mod alloy_client;
mod alloy_wallet;
mod erc20;
mod error;
mod ethereum_address;
mod mnemonic;
mod signature_scheme;
mod sui_address;
mod sui_client;
mod sui_coin_metadata;
mod sui_wallet;
mod trezor;

use crate::{
    alloy_wallet::AlloyWallet,
    alloy_client::AlloyClient,
    erc20::ERC20,
    error::Error,
    ethereum_address::EthereumAddress,
    mnemonic::Mnemonic,
    signature_scheme::SignatureScheme,
    sui_address::SuiAddress,
    sui_client::SuiClient,
    sui_coin_metadata::SuiCoinMetadata,
    sui_wallet::SuiWallet,
    trezor::Trezor
};

pub fn make_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("walletkit-runtime")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_time()
        .enable_io()
        .build()
        .unwrap()
}
