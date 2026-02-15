use alloy::{
    network::Ethereum,
    providers::RootProvider,
    // rpc,
    sol,
    transports::http::reqwest::Url
};

use std::sync::Arc;

use crate::{
    error::Error,
    ethereum_address::EthereumAddress,
    runtime::Runtime
};

sol! {
    #[sol(rpc)]
    interface IERC20 {
        // function allowance(address owner, address spender) external view returns (uint256);
        // function approve(address spender, uint256 value) external returns (bool);
        function balanceOf(address owner) external view returns (uint256);
        function decimals() external view returns (uint8);
        function symbol() external view returns (string);
        // function totalSupply() external view returns (uint256);
        // function transfer(address to, uint256 value) external returns (bool);
        // function transferFrom(address from, address to, uint256 value) external returns (bool);
    }
}

pub struct ERC20 {
    address: EthereumAddress,
    rpc_url: Url
}

impl ERC20 {
    pub fn new(address: Arc<EthereumAddress>, rpc_url: String) -> Result<Self, Error> {
        let rpc_url = rpc_url.parse::<Url>()
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        Ok(Self { address: (*address).clone(), rpc_url })
    }

    pub fn address(&self) -> Arc<EthereumAddress> {
        Arc::new(self.address.clone())
    }

    pub async fn balance_of(&self, owner: Arc<EthereumAddress>) -> Result<Vec<u64>, Error> {
        let provider = RootProvider::<Ethereum>::new_http(self.rpc_url.clone());
        let contract = IERC20::new(self.address.clone().into(), provider.clone());

        let runtime = Runtime::new();
        let balance = runtime.runtime.block_on(async {
            contract.balanceOf(owner.as_ref().into()).call().await
                .map_err(|e| Error::AlloyError { description: e.to_string() })
        })?;

        Ok(balance.as_limbs().to_vec())
    }

    pub async fn decimals(&self) -> Result<u8, Error> {
        let provider = RootProvider::<Ethereum>::new_http(self.rpc_url.clone());
        let contract = IERC20::new(self.address.clone().into(), provider.clone());
        let runtime = Runtime::new();

        runtime.runtime.block_on(async {
            contract.decimals().call().await
            .map_err(|e| Error::AlloyError { description: e.to_string() })
        })
    }

    pub async fn symbol(&self) -> Result<String, Error> {
        let provider = RootProvider::<Ethereum>::new_http(self.rpc_url.clone());
        let contract = IERC20::new(self.address.clone().into(), provider.clone());
        let runtime = Runtime::new();

        runtime.runtime.block_on(async {
            contract.symbol().call().await
            .map_err(|e| Error::AlloyError { description: e.to_string() })
        })
    }
}