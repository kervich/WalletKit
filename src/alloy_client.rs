use alloy::{
    network::{ Ethereum, TxSigner },
    primitives::Address,
    providers::{ Provider, RootProvider },
    signers::{
        local::{
            MnemonicBuilder,
            PrivateKeySigner,
            coins_bip39::{ English, Entropy, Mnemonic }
        },
        trezor::{ HDPath, TrezorSigner },
    },
    transports::http::reqwest::Url
};

use std::sync::Arc;

use crate::{
    error::Error,
    ethereum_address::EthereumAddress,
    runtime::Runtime
};

enum SignerKind {
    Local,
    Trezor,
    WatchOnly
}

pub struct AlloyClient {
    chain_id: u64,
    derivation_path: Option<String>,
    kind: SignerKind,
    local_signer: Option<PrivateKeySigner>,
    rpc_url: Url,
    trezor_signer: Option<TrezorSigner>,
    watch_address: Option<Address>
}

impl AlloyClient {
    pub fn new(
        chain_id: u64,
        account_index: u64,
        private_key: Vec<u8>,
        password: Option<String>,
        rpc_url: String
    ) -> Result<Self, Error> {
        let entropy = Entropy::try_from(private_key.as_slice())
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let mnemonic = Mnemonic::<English>::new_from_entropy(entropy);

        let phrase = mnemonic.to_phrase();

        let derivation_path = format!("m/44'/60'/0'/0/{account_index}");

        let builder = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .password(password.unwrap_or_default())
            .derivation_path(&derivation_path)
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let signer = builder.build()
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let rpc_url = rpc_url.parse()
            .map_err(|_| Error::AlloyError { description: "Invalid URL".to_string() })?;

        Ok(Self {
            chain_id,
            derivation_path: Some(derivation_path),
            kind: SignerKind::Local,
            local_signer: Some(signer),
            rpc_url,
            trezor_signer: None,
            watch_address: None
        })
    }

    pub async fn new_trezor(
        chain_id: u64,
        account_index: u64,
        device_id: String,
        rpc_url: String
    ) -> Result<Self, Error> {
        let derivation = HDPath::TrezorLive(account_index.try_into().unwrap());

        let signer = TrezorSigner::new(derivation, Some(chain_id))
            .await
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let derivation_path = format!("m/44'/60'/0'/0/{account_index}'");

        let rpc_url = rpc_url.parse()
            .map_err(|_| Error::AlloyError { description: "Invalid URL".to_string() })?;

        Ok(Self {
            chain_id,
            derivation_path: Some(derivation_path),
            kind: SignerKind::Trezor,
            local_signer: None,
            trezor_signer: Some(signer),
            watch_address: None,
            rpc_url,
         })
    }

    pub fn new_watch_only(
        chain_id: u64,
        address: Arc<EthereumAddress>,
        rpc_url: String
    ) -> Result<Self, Error> {
        let rpc_url = rpc_url.parse()
            .map_err(|_| Error::AlloyError { description: "Invalid URL".to_string() })?;

        let address: Address = address.as_ref().into();

        Ok(Self {
            chain_id,
            derivation_path: None,
            kind: SignerKind::WatchOnly,
            local_signer: None,
            trezor_signer: None,
            watch_address: Some(address),
            rpc_url,
        })
    }

    pub fn address(&self) -> Arc<EthereumAddress> {
        Arc::new(self.get_address().into())
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    pub fn derivation_path(&self) -> Option<String> {
        self.derivation_path.as_ref().map(|dp| dp.to_string())
    }

    pub async fn get_balance(&self) -> Result<Vec<u64>, Error> {
        let provider = RootProvider::<Ethereum>::new_http(self.rpc_url.clone());
        let runtime = Runtime::new();

        let balance = runtime.runtime.block_on(async {
            provider.get_balance(self.get_address())
                .await
                .map_err(|e| Error::AlloyError { description: e.to_string() })
        })?;

        Ok(balance.as_limbs().to_vec())
    }

    pub async fn is_active_address(&self, address: Arc<EthereumAddress>) -> Result<bool, Error> {
        Err(Error::NotImplemented)
    }

    pub fn rpc_url(&self) -> String {
        self.rpc_url.to_string()
    }
}

impl AlloyClient {
    fn get_address(&self) -> Address {
        match self.kind {
            SignerKind::Local => self.local_signer.as_ref().unwrap().address(),
            SignerKind::Trezor => self.trezor_signer.as_ref().unwrap().address(),
            SignerKind::WatchOnly => self.watch_address.unwrap(),
        }
    }
}
