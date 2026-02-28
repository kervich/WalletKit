use crate::{
    error::Error,
    make_runtime,
    sui_address::SuiAddress,
    sui_coin_metadata::SuiCoinMetadata
};

use std::sync::Arc;
use sui_sdk::SuiClientBuilder;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::base_types::SuiAddress as SDKAddress;
use sui_types::transaction::{ Argument, Command, Transaction, TransactionData };
use sui_types::transaction_driver_types::ExecuteTransactionRequestType;

pub struct SuiClient {
    address: SuiAddress,
    client: sui_sdk::SuiClient,
    runtime: tokio::runtime::Runtime
}

impl SuiClient {
    pub async fn new(
        address: Arc<SuiAddress>,
        rpc_url: String
    ) -> Result<Self, Error> {
        let runtime = make_runtime();
        let rpc_url_clone = rpc_url.clone();

        let client = runtime
            .spawn(async move { SuiClientBuilder::default().build(rpc_url_clone).await })
            .await?
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        Ok(Self { address: (*address).clone(), client, runtime })
    }

    pub fn address(&self) -> Arc<SuiAddress> {
        Arc::new(self.address.clone())
    }

    pub async fn get_all_balances(
        &self,
        owner: Arc<SuiAddress>
    ) -> Result<Vec<u8>, Error> {
        let owner = (*owner).clone().into();
        let sui_client = self.client.clone();

        let balances = self.runtime
            .spawn(async move {
                sui_client
                    .coin_read_api()
                    .get_all_balances(owner)
                    .await
            })
            .await?
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        serde_json::to_vec(&balances)
            .map_err(|e| Error::SuiError { description: e.to_string() })
    }

    pub async fn get_balance(
        &self,
        coin_type: Option<String>
    ) -> Result<String, Error> {
        let client = self.client.clone();
        let coin_type = coin_type.clone();
        let owner = (*self.address()).clone().into();

        let balance = self.runtime
            .spawn(async move { client.coin_read_api().get_balance(owner, coin_type).await })
            .await?
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        Ok(format!("{}", balance.total_balance))
    }

    pub async fn get_coin_metadata(
        &self,
        coin_type: String
    ) -> Result<Option<SuiCoinMetadata>, Error> {
        let client = self.client.clone();
        let coin_type = coin_type.clone();

        let metadata = self.runtime
            .spawn(async move { client.coin_read_api().get_coin_metadata(coin_type).await })
            .await?
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        Ok(metadata.map(|meta| meta.into()))
    }

    pub async fn get_reference_gas_price(&self) -> Result<u64, Error> {
        let client = self.client.clone();

        self.runtime
            .spawn(async move { client.read_api().get_reference_gas_price().await })
            .await?
            .map_err(|e| Error::SuiError { description: e.to_string() })
    }

    pub async fn tx_data(
        &self,
        coin_type: String,
        to: Arc<SuiAddress>,
        amount: u64,
        gas_budget: u64,
        gas_price: u64,
    ) -> Result<Vec<u8>, Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        let sender: SDKAddress = self.address.clone().into();
        let client = self.client.clone();

        let coins = self.runtime
            .spawn(async move {
                client.coin_read_api()
                    .get_coins(sender, Some(coin_type), None, None)
                    .await
            })
            .await?
            .map_err(|e| Error::SuiError { description: e.to_string() })?;
        let coin = coins.data.into_iter().next()
            .ok_or_else(|| Error::SuiError { description: "Coin not found".to_string() })?;
        let split_coin_amount = ptb.pure(amount)
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        ptb.command(Command::SplitCoins(Argument::GasCoin, vec![split_coin_amount]));

        let recipient: SDKAddress = (*to).clone().into();
        let argument_address = ptb.pure(recipient)
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        ptb.command(Command::TransferObjects(vec![Argument::Result(0)], argument_address));

        let tx_data = TransactionData::new_programmable(
            sender,
            vec![coin.object_ref()],
            ptb.finish(),
            gas_budget,
            gas_price,
        );

        serde_json::to_vec(&tx_data)
            .map_err(|e| Error::SuiError { description: e.to_string() })
    }

    pub async fn send(
        &self,
        tx_data: Vec<u8>,
        signature: Vec<u8>
    ) -> Result<String, Error> {
        let client = self.client.clone();

        let tx_data: TransactionData = serde_json::from_slice(&tx_data)
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        let signature = serde_json::from_slice(&signature)
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        let response = self.runtime
            .spawn(async move {
                client.quorum_driver_api()
                .execute_transaction_block(
                    Transaction::from_data(tx_data, vec![signature]),
                    SuiTransactionBlockResponseOptions::full_content(),
                    Some(ExecuteTransactionRequestType::WaitForLocalExecution),
                )
                .await
            })
            .await?
            .map_err(|e| Error::SuiError { description: e.to_string() })?;

        Ok(response.digest.to_string())
    }
}