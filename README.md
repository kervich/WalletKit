# WalletKit

Crypto wallet utilities for Swift, using Rust implementations.

## Features

- BIP39 key mnemonics
- BIP32/SLIP-0010 keys from [Sui Rust SDK](https://docs.sui.io/references/rust-sdk) (ECDSA Secp256k1 and Ed25519 signing schemes are supported)
- EVM and Sui address generation from HDR key
- EVM and Sui address validation

Rust compiler produces very large binary, that is stored with git-lfs. Tuist and SPM don't fully support this, so you may have to check out manually and import local SPM package.
