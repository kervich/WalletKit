use alloy::{network::TxSigner, signers::local::{
    MnemonicBuilder, PrivateKeySigner, coins_bip39::{English, Entropy, Mnemonic}
}};
use alloy::signers::trezor::{HDPath, TrezorSigner};
use std::str::FromStr;

use crate::{BlockchainClient, error::Error};

enum SignerKind {
    Local,
    Trezor,
    WatchOnly
}

pub struct AlloyClient {
    chain_id: u64,
    derivation_path: Option<String>,
    local_signer: Option<PrivateKeySigner>,
    trezor_signer: Option<TrezorSigner>,
    watch_address: Option<String>,
    kind: SignerKind
}

impl AlloyClient {
    pub fn new(
        chain_id: u64,
        account_index: u64,
        private_key: Vec<u8>,
        password: Option<String>
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

        Ok(Self {
            chain_id,
            derivation_path: Some(derivation_path),
            kind: SignerKind::Local,
            local_signer: Some(signer),
            trezor_signer: None,
            watch_address: None
        })
    }

    pub async fn new_trezor(chain_id: u64, account_index: u64, device_id: String) -> Result<Self, Error> {
        let derivation = HDPath::TrezorLive(account_index.try_into().unwrap());

        let signer = TrezorSigner::new(derivation, Some(chain_id))
            .await
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let derivation_path = format!("m/44'/60'/0'/0/{account_index}'");

        Ok(Self {
            chain_id,
            derivation_path: Some(derivation_path),
            kind: SignerKind::Trezor,
            local_signer: None,
            trezor_signer: Some(signer),
            watch_address: None
         })
    }

    pub fn new_watch_only(chain_id: u64, address: String) -> Result<Self, Error> {
        if alloy_primitives::Address::from_str(&address).is_ok() {
            Ok(Self {
                chain_id,
                derivation_path: None,
                kind: SignerKind::WatchOnly,
                local_signer: None,
                trezor_signer: None,
                watch_address: Some(address)
            })
        } else {
            Err(Error::AlloyError { description: "Invalid address".to_string() })
        }
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }
}

impl BlockchainClient for AlloyClient {
    fn address(&self) -> String {
        match self.kind {
            SignerKind::Local => self.local_signer.as_ref().unwrap().address().to_string(),
            SignerKind::Trezor => self.trezor_signer.as_ref().unwrap().address().to_string(),
            SignerKind::WatchOnly => self.watch_address.as_ref().unwrap().to_string(),
        }
    }

    fn derivation_path(&self) -> Option<String> {
        self.derivation_path.as_ref().map(|dp| dp.to_string())
    }

    fn is_active_address(&self, address: String) -> bool {
        false // TODO: implement this by checking the transaction history of the address
    }

    fn is_valid_address(&self, address: String) -> bool {
        alloy_primitives::Address::from_str(&address).is_ok()
    }
}