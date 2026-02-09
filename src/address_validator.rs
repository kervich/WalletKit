use ethers::types::Address as EVMAddress;
use std::str::FromStr;
use sui_types::base_types::SuiAddress;

pub struct AddressValidator {
}

impl AddressValidator {
    pub fn new() -> Self {
        AddressValidator {}
    }

    pub fn validate_evm_address(&self, address: String) -> bool {
        EVMAddress::from_str(&address).is_ok()
    }

    pub fn validate_sui_address(&self, address: String) -> bool {
        SuiAddress::from_str(&address).is_ok()
    }
}