use std::sync::Arc;
use sui_sdk::{SuiClientBuilder};

use crate::{
    error::Error,
    make_runtime,
    sui_address::SuiAddress,
    sui_coin_metadata::SuiCoinMetadata
};

pub struct SuiClient {
    address: SuiAddress,
    client: sui_sdk::SuiClient,
    rpc_url: String,
    runtime: tokio::runtime::Runtime
}

pub struct SuiBalance {
    pub coin_type: String,
    pub amount: String,
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

        Ok(Self {
            address: (*address).clone(),
            client,
            rpc_url,
            runtime: runtime
        })
    }

    pub fn address(&self) -> Arc<SuiAddress> {
        self.address.clone().into()
    }

    pub async fn get_all_balances(
        &self,
        owner: Arc<SuiAddress>
    ) -> Result<Vec<SuiBalance>, Error> {
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

        let result = balances.into_iter()
            .map(|balance| {
                SuiBalance {
                    coin_type: balance.coin_type,
                    amount: format!("{}", balance.total_balance),
                }
            })
            .collect();

        Ok(result)
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

    pub async fn is_active_address(
        &self,
        address: Arc<SuiAddress>
    ) -> Result<bool, Error> {
        self.runtime.block_on(async {
            let client = SuiClientBuilder::default()
                .build(self.rpc_url.clone())
                .await
                .map_err(|e| Error::SuiError { description: e.to_string() })?;

            let balance = client.coin_read_api()
                .get_balance((*address).clone().into(), None)
                .await
                .map_err(|e| Error::SuiError { description: e.to_string() })?;

            Ok(balance.total_balance > 0)
        })
    }
}