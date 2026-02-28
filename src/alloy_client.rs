use alloy::{
    consensus::{EthereumTxEnvelope, TxEip4844Variant},
    network::{ Ethereum, TransactionBuilder },
    primitives::Address,
    providers::{ Provider, RootProvider },
    rpc::types::TransactionRequest,
    transports::http::reqwest::Url
};
use std::sync::Arc;

use crate::{
    erc20::ERC20,
    error::Error,
    ethereum_address::EthereumAddress,
    make_runtime
};

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
                self.chain_id,
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

    pub async fn estimate_fees(&self) -> Result<Vec<u8>, Error> {
        let provider = self.provider.clone();

        let fees = self.runtime
            .spawn(async move { provider.estimate_eip1559_fees().await })
            .await?
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        serde_json::to_vec(&fees)
            .map_err(|e| Error::AlloyError { description: e.to_string() })
    }

    pub async fn send(
        &self,
        tx_envelope: Vec<u8>
    ) -> Result<String, Error> {
        let tx_envelope: EthereumTxEnvelope<TxEip4844Variant> = serde_json::from_slice(&tx_envelope)
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let provider = self.provider.clone();

        let receipt = self.runtime
            .spawn(async move { provider.send_tx_envelope(tx_envelope).await?.get_receipt().await })
            .await?
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        Ok(format!("{:?}", receipt.transaction_hash))
    }

    pub async fn tx_request(
        &self,
        to: Arc<EthereumAddress>,
        amount: String,
        nonce: Option<u64>,
        gas_limit: Option<u64>,
        fees: Option<Vec<u8>>
    ) -> Result<Vec<u8>, Error> {
        let amount = amount.parse::<alloy_primitives::U256>()
            .map_err(|e| Error::AlloyError { description: e.to_string() })?;

        let mut tx_request = TransactionRequest::default()
            .from(self.address.clone().into())
            .to(to.as_ref().into())
            .value(amount)
            .with_chain_id(self.chain_id);

        if let Some(gas_limit) = gas_limit {
            tx_request = tx_request.gas_limit(gas_limit);
        }

        if let Some(fees) = fees {
            let fees: alloy::eips::eip1559::Eip1559Estimation = serde_json::from_slice(&fees)
                .map_err(|e| Error::AlloyError { description: e.to_string() })?;

            tx_request = tx_request
                .max_fee_per_gas(fees.max_fee_per_gas)
                .max_priority_fee_per_gas(fees.max_priority_fee_per_gas);
        }

        if let Some(nonce) = nonce {
            tx_request = tx_request.nonce(nonce);
        }

        serde_json::to_vec(&tx_request)
            .map_err(|e| Error::AlloyError { description: e.to_string() })
    }

    pub fn rpc_url(&self) -> String {
        self.rpc_url.to_string()
    }
}