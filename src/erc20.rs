use alloy::{
    network::Ethereum,
    primitives::U256,
    providers::{RootProvider},
    sol,
    sol_types::SolInterface
};

use std::sync::Arc;

use crate::{
    error::Error,
    ethereum_address::EthereumAddress,
    make_runtime
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
        function transfer(address to, uint256 value) external returns (bool);
        function transferFrom(address from, address to, uint256 value) external returns (bool);
    }
}

pub struct ERC20 {
    contract_address: EthereumAddress,
    own_address: EthereumAddress,
    provider: RootProvider<Ethereum>,
    runtime: tokio::runtime::Runtime
}

impl ERC20 {
    pub fn new(
        contract_address: EthereumAddress,
        own_address: EthereumAddress,
        provider: RootProvider<Ethereum>,
    ) -> Self {
        Self { contract_address, own_address, provider, runtime: make_runtime() }
    }

    pub fn contract_address(&self) -> Arc<EthereumAddress> {
        Arc::new(self.contract_address.clone())
    }

    pub async fn balance_of(
        &self,
        owner: Arc<EthereumAddress>
    ) -> Result<String, Error> {
        let contract = IERC20::new(self.contract_address.clone().into(), self.provider.clone());

        let balance = self.runtime.block_on(async {
            contract.balanceOf(owner.as_ref().into())
                .call()
                .await
                .map_err(|e| Error::AlloyError { description: e.to_string() })
        })?;

        Ok(format!("{}", balance))
    }

    pub async fn decimals(&self) -> Result<u8, Error> {
        let contract = IERC20::new(self.contract_address.clone().into(), self.provider.clone());

        let decimals = self.runtime.block_on(async {
            contract.decimals().call().await
                .map_err(|e| Error::AlloyError { description: e.to_string() })
        })?;

        Ok(decimals)
    }

    pub fn make_transfer_tx(
        &self,
        recipient: Arc<EthereumAddress>,
        amount: String
    ) -> Result<Vec<u8>, Error> {
        println!("Making transfer tx: self={}, recipient={}, amount={}", self.own_address, recipient, amount);

        let amount = amount.parse::<U256>()
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let call = IERC20::IERC20Calls::transfer(IERC20::transferCall {
            to: recipient.as_ref().into(),
            value: amount,
        });

        let tx_request = alloy::rpc::types::eth::TransactionRequest::default()
            .from(self.own_address.clone().into())
            .to(self.contract_address.clone().into())
            .input(call.abi_encode().into());

        let data = serde_json::to_string(&tx_request)
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        Ok(data.into_bytes())
    }

    pub fn make_transfer_from_tx(
        &self,
        sender: Arc<EthereumAddress>,
        recipient: Arc<EthereumAddress>,
        amount: String
    ) -> Result<Vec<u8>, Error> {
        println!(
            "Making transfer from tx: self={}, sender={}, recipient={}, amount={}",
            self.own_address.clone(),
            sender,
            recipient,
            amount
        );

        let amount = amount.parse::<U256>()
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let call = IERC20::IERC20Calls::transferFrom(IERC20::transferFromCall {
            from: sender.as_ref().into(),
            to: recipient.as_ref().into(),
            value: amount,
        });

        let tx_request = alloy::rpc::types::eth::TransactionRequest::default()
            .from(self.own_address.clone().into())
            .input(call.abi_encode().into());

        let data = serde_json::to_string(&tx_request)
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        Ok(data.into_bytes())
    }

    pub async fn symbol(&self) -> Result<String, Error> {
        let contract = IERC20::new(self.contract_address.clone().into(), self.provider.clone());

        let symbol = self.runtime.block_on(async {
            contract.symbol().call().await
                .map_err(|e| Error::AlloyError { description: e.to_string() })
        })?;

        Ok(symbol)
    }
}