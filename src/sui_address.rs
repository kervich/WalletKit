use crate::error::Error;
use sui_types::base_types::SuiAddress as SDKAddress;

pub struct SuiAddress {
    address: SDKAddress
}

impl SuiAddress {
    pub fn new(address: String) -> Result<Self, Error> {
        let address = address.parse::<SDKAddress>()
            .map_err(|e| Error::InvalidAddress { description: e.to_string() })?;
        Ok(Self { address })
    }

    pub fn to_string(&self) -> String {
        self.address.to_string()
    }
}

impl Clone for SuiAddress {
    fn clone(&self) -> Self {
        Self { address: self.address }
    }
}

impl From<SDKAddress> for SuiAddress {
    fn from(address: SDKAddress) -> Self {
        Self { address }
    }
}

impl Into<SDKAddress> for SuiAddress {
    fn into(self) -> SDKAddress {
        self.address
    }
}