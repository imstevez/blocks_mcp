use anyhow::{Result, anyhow};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct API {
    pub cached_chains: Arc<RwLock<HashMap<i32, Chain>>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chain {
    name: String,
    description: String,
    #[serde(rename = "isTestnet")]
    is_test_net: bool,
    explorers: Vec<ChainExplorer>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChainExplorer {
    url: String,
}

impl Chain {
    pub fn get_url(self: &Self) -> Result<String> {
        if self.explorers.len() > 0 {
            return Ok(self.explorers[0].url.clone());
        }
        Err(anyhow!("no explorers"))
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SearchParams {
    pub q: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetTransactionsParams {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub filter: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub typ: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub method: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetBlocksParams {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub typ: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetTransactionTokenTransfersParams {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub typ: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetAddressTransactionsParams {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub filter: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetAddressTokenTransfersParams {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub filter: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub token: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetAddressInternalTransactionsParams {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub filter: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetAddressTokensParams {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetAddressNftsParams {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetTokensParams {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "type")]
    pub typ: String,
}

impl API {
    pub fn new() -> Self {
        API {
            cached_chains: Arc::new(RwLock::new(HashMap::<i32, Chain>::new())),
        }
    }

    pub async fn get_chain(self: &Self, chain_id: i32) -> Result<Chain> {
        {
            let read_lock = self.cached_chains.read().await;
            let chain = read_lock.get(&chain_id);
            if chain.is_some() {
                let chain = chain.unwrap().clone();
                return Ok(chain);
            }
        }
        {
            let mut write_lock = self.cached_chains.write().await;

            let res = reqwest::Client::new()
                .get(format!(
                    "https://chains.blockscout.com/api/chains/{}",
                    chain_id
                ))
                .send()
                .await?;

            if res.status() != StatusCode::OK {
                return Err(anyhow!("request failed: {}", res.status()));
            }

            let chain: Chain = res.json().await?;
            write_lock.insert(chain_id, chain.clone());

            Ok(chain)
        }
    }

    pub async fn get_chain_explorer_url(self: &Self, chain_id: i32) -> Result<String> {
        if chain_id == 4200 {
            return Ok("https://scan.merlinverify.com/".into());
        }
        let chain = self.get_chain(chain_id).await?;
        chain.get_url()
    }

    pub async fn request<T: Serialize + ?Sized>(
        self: &Self,
        chain_id: i32,
        path: impl Into<String>,
        query: &T,
    ) -> Result<Value> {
        let url = self.get_chain_explorer_url(chain_id).await?;
        let res = reqwest::Client::new()
            .get(format!("{}api/v2/{}", url, path.into()))
            .query(query)
            .send()
            .await?;

        if res.status() != StatusCode::OK {
            return Err(anyhow!("request failed: {}", res.status()));
        }

        let data: Value = res.json().await?;

        Ok(data)
    }

    pub async fn search(self: &Self, chain_id: i32, params: SearchParams) -> Result<Value> {
        self.request(chain_id, "search", &params).await
    }

    pub async fn get_transactions(
        self: &Self,
        chain_id: i32,
        params: GetTransactionsParams,
    ) -> Result<Value> {
        self.request(chain_id, "transactions", &params).await
    }

    pub async fn get_blocks(self: &Self, chain_id: i32, params: GetBlocksParams) -> Result<Value> {
        self.request(chain_id, "blocks", &params).await
    }

    pub async fn get_transfers(self: &Self, chain_id: i32) -> Result<Value> {
        self.request(chain_id, "token-transfers", &()).await
    }

    pub async fn get_internal_transactions(self: &Self, chain_id: i32) -> Result<Value> {
        self.request(chain_id, "internal-transactions", &()).await
    }

    pub async fn get_withdrawals(self: &Self, chain_id: i32) -> Result<Value> {
        self.request(chain_id, "withdrawals", &()).await
    }

    pub async fn get_stats(self: &Self, chain_id: i32) -> Result<Value> {
        self.request(chain_id, "stats", &()).await
    }

    pub async fn get_transaction_info(self: &Self, chain_id: i32, hash: String) -> Result<Value> {
        self.request(chain_id, format!("transactions/{}", hash), &())
            .await
    }

    pub async fn get_transaction_token_transfers(
        self: &Self,
        chain_id: i32,
        hash: String,
        params: GetTransactionTokenTransfersParams,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("transactions/{}/token-transfers", hash),
            &params,
        )
        .await
    }

    pub async fn get_transaction_internal_transactions(
        self: &Self,
        chain_id: i32,
        hash: String,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("transactions/{}/internal-transactions", hash),
            &(),
        )
        .await
    }

    pub async fn get_transaction_logs(self: &Self, chain_id: i32, hash: String) -> Result<Value> {
        self.request(chain_id, format!("transactions/{}/logs", hash), &())
            .await
    }

    pub async fn get_transaction_summary(
        self: &Self,
        chain_id: i32,
        hash: String,
    ) -> Result<Value> {
        self.request(chain_id, format!("transactions/{}/summary", hash), &())
            .await
    }

    pub async fn get_block_info(
        self: &Self,
        chain_id: i32,
        number_or_hash: String,
    ) -> Result<Value> {
        self.request(chain_id, format!("blocks/{}", number_or_hash), &())
            .await
    }

    pub async fn get_block_transactions(
        self: &Self,
        chain_id: i32,
        number_or_hash: String,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("blocks/{}/transactions", number_or_hash),
            &(),
        )
        .await
    }

    pub async fn get_block_withdrawals(
        self: &Self,
        chain_id: i32,
        number_or_hash: String,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("blocks/{}/withdrawals", number_or_hash),
            &(),
        )
        .await
    }

    pub async fn get_addresses(self: &Self, chain_id: i32) -> Result<Value> {
        self.request(chain_id, "addresses", &()).await
    }

    pub async fn get_address_info(self: &Self, chain_id: i32, hash: String) -> Result<Value> {
        self.request(chain_id, format!("addresses/{}", hash), &())
            .await
    }

    pub async fn get_address_counters(self: &Self, chain_id: i32, hash: String) -> Result<Value> {
        self.request(chain_id, format!("addresses/{}/counters", hash), &())
            .await
    }

    pub async fn get_address_transactions(
        self: &Self,
        chain_id: i32,
        hash: String,
        params: GetAddressTransactionsParams,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("addresses/{}/transactions", hash),
            &params,
        )
        .await
    }

    pub async fn get_address_token_transfers(
        self: &Self,
        chain_id: i32,
        hash: String,
        params: GetAddressTokenTransfersParams,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("addresses/{}/token-transfers", hash),
            &params,
        )
        .await
    }

    pub async fn get_address_internal_transactions(
        self: &Self,
        chain_id: i32,
        hash: String,
        params: GetAddressInternalTransactionsParams,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("addresses/{}/internal-transactions", hash),
            &params,
        )
        .await
    }

    pub async fn get_address_logs(self: &Self, chain_id: i32, hash: String) -> Result<Value> {
        self.request(chain_id, format!("addresses/{}/logs", hash), &())
            .await
    }

    pub async fn get_address_tokens(
        self: &Self,
        chain_id: i32,
        hash: String,
        params: GetAddressTokensParams,
    ) -> Result<Value> {
        self.request(chain_id, format!("addresses/{}/tokens", hash), &params)
            .await
    }

    pub async fn get_address_coin_balance_history(
        self: &Self,
        chain_id: i32,
        hash: String,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("addresses/{}/coin-balance-history", hash),
            &(),
        )
        .await
    }

    pub async fn get_address_coin_balance_history_by_day(
        self: &Self,
        chain_id: i32,
        hash: String,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("addresses/{}/coin-balance-history-by-day", hash),
            &(),
        )
        .await
    }

    pub async fn get_address_withdrawals(
        self: &Self,
        chain_id: i32,
        hash: String,
    ) -> Result<Value> {
        self.request(chain_id, format!("addresses/{}/withdrawals", hash), &())
            .await
    }

    pub async fn get_address_nfts(
        self: &Self,
        chain_id: i32,
        hash: String,
        params: GetAddressNftsParams,
    ) -> Result<Value> {
        self.request(chain_id, format!("addresses/{}/nft", hash), &params)
            .await
    }

    pub async fn get_address_nft_collections(
        self: &Self,
        chain_id: i32,
        hash: String,
        params: GetAddressNftsParams,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("addresses/{}/nft/collections", hash),
            &params,
        )
        .await
    }

    pub async fn get_tokens(self: &Self, chain_id: i32, params: GetTokensParams) -> Result<Value> {
        self.request(chain_id, "tokens", &params).await
    }

    pub async fn get_token_info(self: &Self, chain_id: i32, hash: String) -> Result<Value> {
        self.request(chain_id, format!("tokens/{}", hash), &())
            .await
    }

    pub async fn get_token_transfers(self: &Self, chain_id: i32, hash: String) -> Result<Value> {
        self.request(chain_id, format!("tokens/{}/transfers", hash), &())
            .await
    }

    pub async fn get_token_holders(self: &Self, chain_id: i32, hash: String) -> Result<Value> {
        self.request(chain_id, format!("tokens/{}/holders", hash), &())
            .await
    }

    pub async fn get_token_counters(self: &Self, chain_id: i32, hash: String) -> Result<Value> {
        self.request(chain_id, format!("tokens/{}/counters", hash), &())
            .await
    }

    pub async fn get_token_instances(self: &Self, chain_id: i32, hash: String) -> Result<Value> {
        self.request(chain_id, format!("tokens/{}/instances", hash), &())
            .await
    }

    pub async fn get_token_instance_info(
        self: &Self,
        chain_id: i32,
        hash: String,
        id: u64,
    ) -> Result<Value> {
        self.request(chain_id, format!("tokens/{}/instances/{}", hash, id), &())
            .await
    }

    pub async fn get_token_instance_transfers(
        self: &Self,
        chain_id: i32,
        hash: String,
        id: u64,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("tokens/{}/instances/{}/transfers", hash, id),
            &(),
        )
        .await
    }

    pub async fn get_token_instance_holders(
        self: &Self,
        chain_id: i32,
        hash: String,
        id: u64,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("tokens/{}/instances/{}/holders", hash, id),
            &(),
        )
        .await
    }

    pub async fn get_token_instance_transfers_count(
        self: &Self,
        chain_id: i32,
        hash: String,
        id: u64,
    ) -> Result<Value> {
        self.request(
            chain_id,
            format!("tokens/{}/instances/{}/transfers-count", hash, id),
            &(),
        )
        .await
    }
}

#[tokio::test]
async fn test_search() {
    let api = API::new();
    let r = api
        .search(
            1,
            SearchParams {
                q: "WETH".to_string(),
            },
        )
        .await
        .unwrap();
    let raw = serde_json::to_string_pretty(&r).unwrap();
    println!("{}", raw)
}
