use alloy::{
    network::{ Ethereum, TransactionBuilder },
    primitives::Address,
    providers::{ Provider, RootProvider },
    rpc::types::TransactionRequest,
    transports::http::reqwest::Url
};

use std::sync::Arc;

use crate::{
    alloy_wallet::AlloyWallet,
    erc20::ERC20,
    error::Error,
    ethereum_address::EthereumAddress,
    make_runtime
};

pub struct FeeData {
    pub max_fee_per_gas: String,
    pub priority_fee_per_gas: String
}

pub struct AlloyClient {
    address: Address,
    chain_id: u64,
    provider: RootProvider<Ethereum>,
    rpc_url: Url,
    runtime: tokio::runtime::Runtime
}

impl AlloyClient {
    pub fn new(
        address: Arc<EthereumAddress>,
        chain_id: u64,
        rpc_url: String
    ) -> Result<Self, Error> {
        let rpc_url: Url = rpc_url.parse()
            .map_err(|_| Error::AlloyError { description: "Invalid URL".to_string() })?;

        Ok(Self {
            address: address.as_ref().into(),
            chain_id,
            provider: RootProvider::<Ethereum>::new_http(rpc_url.clone()),
            rpc_url,
            runtime: make_runtime()
        })
    }
}

impl AlloyClient {
    pub fn address(&self) -> Arc<EthereumAddress> {
        Arc::new(self.address.into())
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    pub fn erc20(
        &self,
        contract_address: Arc<EthereumAddress>
    ) -> Arc<ERC20> {
        Arc::new(
            ERC20::new(
                contract_address.as_ref().clone(),
                self.address.into(),
                self.provider.clone()
            )
        )
    }

    pub async fn get_balance(&self) -> Result<String, Error> {
        let provider = self.provider.clone();
        let address = self.address.clone();

        let balance = self.runtime
            .spawn(async move { provider.get_balance(address).await })
            .await?
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        Ok(format!("{}", balance))
    }

    pub async fn get_nonce(&self) -> Result<u64, Error> {
        let provider = self.provider.clone();
        let address = self.address.clone();

        let nonce = self.runtime
            .spawn(async move { provider.get_transaction_count(address.into()).await })
            .await?
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        Ok(nonce)
    }

    pub async fn estimate_gas(
        &self,
        tx: Vec<u8>
    ) -> Result<u64, Error> {
        let provider = self.provider.clone();
        let tx: TransactionRequest = serde_json::from_slice(&tx)
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        self.runtime
            .spawn(async move { provider.estimate_gas(tx).await })
            .await?
            .map_err(|e| Error::AlloyError { description: e.to_string() })
    }

    pub async fn estimate_fees(&self) -> Result<FeeData, Error> {
        let provider = self.provider.clone();

        let fee_data = self.runtime
            .spawn(async move { provider.estimate_eip1559_fees().await })
            .await?
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        Ok(FeeData {
            max_fee_per_gas: format!("{}", fee_data.max_fee_per_gas),
            priority_fee_per_gas: format!("{}", fee_data.max_priority_fee_per_gas)
        })
    }

    pub fn make_transfer_tx(
        &self,
        to: Arc<EthereumAddress>,
        amount: String
    ) -> Result<Vec<u8>, Error> {
        let amount = amount.parse::<alloy_primitives::U256>()
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let tx_request = TransactionRequest::default()
            .from(self.address.clone().into())
            .to(to.as_ref().into())
            .value(amount);

        serde_json::to_vec(&tx_request)
            .map_err(|e| Error::AlloyError { description: e.to_string() })
    }

    pub fn rpc_url(&self) -> String {
        self.rpc_url.to_string()
    }

    pub async fn send_transaction(
        &self,
        tx: Vec<u8>,
        wallet: Arc<AlloyWallet>,
        gas_limit: u64,
        fee_data: FeeData
    ) -> Result<String, Error> {
        let max_fee_per_gas = fee_data.max_fee_per_gas.parse::<u128>()
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let max_priority_fee_per_gas = fee_data.priority_fee_per_gas.parse::<u128>()
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let mut tx_request: TransactionRequest = serde_json::from_slice(&tx)
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let nonce = self.get_nonce().await?;

        tx_request = tx_request
            .gas_limit(gas_limit)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .nonce(nonce)
            .with_chain_id(self.chain_id);

        println!("Final tx request: {:?}", tx_request);

        let tx_envelope = tx_request.build(&wallet.wallet)
            .await
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        println!("Built tx envelope: {:?}", tx_envelope);

        let provider = self.provider.clone();

        let receipt = self.runtime
            .spawn(async move { provider.send_tx_envelope(tx_envelope).await?.get_receipt().await })
            .await?
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        Ok(format!("{:?}", receipt.transaction_hash))
    }
}