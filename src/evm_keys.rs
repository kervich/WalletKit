use bip32::{ DerivationPath, XPrv };
use ethers::{core::k256::SecretKey, signers::{ LocalWallet, Signer}};

use crate::{
    error::Error,
};

pub type EvmAddress = String;

pub struct EvmSigner {
    wallet: LocalWallet
}

pub fn derive_key_pair_from_path(
    seed: &[u8],
    derivation_path: DerivationPath,
) -> Result<(EvmAddress, EvmSigner), Error> {
    let child_xprv = XPrv::derive_from_path(
        seed,
         &derivation_path
    )
        .map_err(|e| Error::KeyDerivationError { description: e.to_string() })?;

    let secret_key = SecretKey::from_slice(child_xprv.private_key().to_bytes().as_slice())
        .map_err(|e| Error::KeyDerivationError { description: e.to_string() })?;

    let wallet = LocalWallet::from(secret_key);
    let address = wallet.address().to_string();

    Ok((address, EvmSigner { wallet }))
}