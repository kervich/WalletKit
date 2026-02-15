use alloy::primitives::Address;
use crate::error::Error;

pub struct EthereumAddress {
    address: Address
}

impl EthereumAddress {
    pub fn new(address: String) -> Result<Self, Error> {
        let address = address.parse::<Address>()
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;
        Ok(Self { address })
    }

    pub fn to_string(&self) -> String {
        self.address.to_string()
    }
}

impl Clone for EthereumAddress {
    fn clone(&self) -> Self {
        Self { address: self.address }
    }
}

impl From<Address> for EthereumAddress {
    fn from(address: Address) -> Self {
        Self { address }
    }
}

impl From<EthereumAddress> for Address {
    fn from(eth_address: EthereumAddress) -> Self {
        eth_address.address
    }
}

impl From<&EthereumAddress> for Address {
    fn from(eth_address: &EthereumAddress) -> Self {
        eth_address.address
    }
}