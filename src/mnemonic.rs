use bip39::Mnemonic as Bip39Mnemonic;
use crate::error::Error;

pub struct Mnemonic {
    mnemonic: Bip39Mnemonic,
}

impl Mnemonic {
    pub fn from_seed_phrase(seed_phrase: String) -> Result<Self, Error> {
        let mnemonic = Bip39Mnemonic::parse(seed_phrase)
            .map_err(|e| Error::InvalidMnemonic { description: e.to_string() })?;
        Ok(
            Mnemonic { mnemonic }
        )
    }

    pub fn from_entropy(entropy: Vec<u8>) -> Result<Self, Error> {
        let mnemonic = Bip39Mnemonic::from_entropy(&entropy)
            .map_err(|e| Error::InvalidData { description: e.to_string() })?;

        Ok(
            Mnemonic { mnemonic }
        )
    }

    pub fn seed_phrase(&self) -> String {
        self.mnemonic.to_string()
    }

    pub fn entropy(&self) -> Vec<u8> {
        self.mnemonic.to_entropy()
    }

    pub fn entropy_with_passphrase(&self, _passphrase: String) -> Vec<u8> {
        self.mnemonic.to_entropy()
    }
}