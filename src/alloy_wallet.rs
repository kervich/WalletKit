use alloy::{
    network::{EthereumWallet, TxSigner},
    signers::{
        local::{
            MnemonicBuilder,
            coins_bip39::{ English, Entropy, Mnemonic }
        },
        trezor::{ HDPath, TrezorSigner },
    },
};

use std::sync::Arc;

use crate::{
    error::Error,
    ethereum_address::EthereumAddress,
};

pub struct AlloyWallet {
    derivation_path: String,
    pub wallet: EthereumWallet
}

impl AlloyWallet {
    pub fn new(
        account_index: u64,
        private_key: Vec<u8>,
    ) -> Result<Self, Error> {
        let entropy = Entropy::try_from(private_key.as_slice())
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let mnemonic = Mnemonic::<English>::new_from_entropy(entropy);

        let phrase = mnemonic.to_phrase();

        let derivation_path = format!("m/44'/60'/0'/0/{account_index}");

        let builder = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .derivation_path(&derivation_path)
            .map_err(|e| Error::MnemonicError { description: e.to_string() })?;

        let signer = builder.build()
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let wallet = EthereumWallet::from(signer.clone());

        Ok(Self {
            derivation_path,
            wallet
        })
    }

    pub async fn new_trezor(
        account_index: u64,
        chain_id: Option<u64>
    ) -> Result<Self, Error> {
        let derivation = HDPath::TrezorLive(account_index.try_into().unwrap());

        let signer = TrezorSigner::new(derivation, chain_id)
            .await
            .map_err(|e| Error::TrezorError { description: e.to_string() })?;

        let derivation_path = format!("m/44'/60'/0'/0/{account_index}'");

        let wallet = EthereumWallet::from(signer);

        Ok(Self {
            derivation_path: derivation_path,
            wallet,
        })
    }
}

impl AlloyWallet {
    pub fn address(&self) -> Arc<EthereumAddress> {
        Arc::new(self.wallet.default_signer().address().into())
    }

    pub fn derivation_path(&self) -> String {
        self.derivation_path.clone()
    }
}