use crate::block_scout_api::{
    API, GetAddressInternalTransactionsParams, GetAddressNftsParams,
    GetAddressTokenTransfersParams, GetAddressTokensParams, GetAddressTransactionsParams,
    GetBlocksParams, GetTokensParams, GetTransactionTokenTransfersParams, GetTransactionsParams,
    SearchParams,
};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
};
use serde_json::{Map, Value};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BaseRequest {
    #[schemars(description = "the chain id to query")]
    pub chain_id: i32,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EmptyRequest {}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchRequest {
    #[schemars(description = "the chain id to query")]
    pub chain_id: i32,
    #[schemars(description = "the query to search, it can be token name, token symbol, address, transaction hash, block number, block hash")]
    pub q: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TransactionRequest {
    #[schemars(description = "the chain id to query")]
    pub chain_id: i32,
    #[schemars(description = "the transaction hash to query")]
    pub transaction_hash: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BlockRequest {
    #[schemars(description = "the chain id to query")]
    pub chain_id: i32,
    #[schemars(description = "the block number or block hash to query")]
    pub number_or_hash: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AddressRequest {
    #[schemars(description = "the chain id to query")]
    pub chain_id: i32,
    #[schemars(description = "the address hash to query")]
    pub address_hash: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TokenRequest {
    #[schemars(description = "the chain id to query")]
    pub chain_id: i32,
    #[schemars(description = "the token address to query")]
    pub token_address: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TokenInstanceRequest {
    #[schemars(description = "the chain id to query")]
    pub chain_id: i32,
    #[schemars(description = "the token address to query")]
    pub token_address: String,
    #[schemars(description = "the token id to query")]
    pub token_id: u64,
}

#[derive(Clone)]
pub struct OnChainData {
    block_scout_api: API,
    tool_router: ToolRouter<OnChainData>,
}

#[tool_router]
impl OnChainData {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            block_scout_api: API::new(),
            tool_router: Self::tool_router(),
        }
    }

    fn convert_result(rst: anyhow::Result<Value>) -> Result<CallToolResult, McpError> {
        match rst {
            Ok(r) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&r).unwrap(),
            )])),
            Err(e) => Err(ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                e.to_string(),
                None,
            )),
        }
    }

    #[tool(
        description = "Search chain data with token name, token symbol, account name, address, transaction hash"
    )]
    async fn search(
        &self,
        Parameters(SearchRequest { chain_id, q }): Parameters<SearchRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .search(chain_id, SearchParams { q })
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get Merlin chain info")]
    async fn get_merlin_chain_info(
        &self,
        _: Parameters<EmptyRequest>,
    ) -> Result<CallToolResult, McpError> {
        let mut data = Map::new();
        data.insert("chain_id".into(), Value::String("4200".into()));
        data.insert("native_token_symbol".into(), Value::String("BTC".into()));
        data.insert("native_token_decimals".into(), Value::String("18".into()));
        data.insert("note".into(), Value::String("The native token on merlin is BTC, but the decimals of merlin BTC is 18, so 1 merlin BTC = 1 * 10^18 wei".into()));
        Self::convert_result(Ok(Value::Object(data)))
    }

    #[tool(description = "List latest 50 transactions")]
    async fn get_transactions(
        &self,
        Parameters(BaseRequest { chain_id }): Parameters<BaseRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_transactions(
                chain_id,
                GetTransactionsParams {
                    ..Default::default()
                },
            )
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List latest 50 blocks")]
    async fn get_blocks(
        &self,
        Parameters(BaseRequest { chain_id }): Parameters<BaseRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_blocks(
                chain_id,
                GetBlocksParams {
                    ..Default::default()
                },
            )
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List latest 50 token transfers")]
    async fn get_transfers(
        &self,
        Parameters(BaseRequest { chain_id }): Parameters<BaseRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self.block_scout_api.get_transfers(chain_id).await;
        Self::convert_result(rst)
    }

    #[tool(description = "List latest 50 internal transactions")]
    async fn get_internal_transactions(
        &self,
        Parameters(BaseRequest { chain_id }): Parameters<BaseRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_internal_transactions(chain_id)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List latest 50 withdrawals")]
    async fn get_withdrawals(
        &self,
        Parameters(BaseRequest { chain_id }): Parameters<BaseRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self.block_scout_api.get_withdrawals(chain_id).await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get chain stats counters")]
    async fn get_chain_stats(
        &self,
        Parameters(BaseRequest { chain_id }): Parameters<BaseRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self.block_scout_api.get_stats(chain_id).await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get transaction info")]
    async fn get_transaction_info(
        &self,
        Parameters(TransactionRequest {
            chain_id,
            transaction_hash,
        }): Parameters<TransactionRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_transaction_info(chain_id, transaction_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get transaction token transfers")]
    async fn get_transaction_token_transfers(
        &self,
        Parameters(TransactionRequest {
            chain_id,
            transaction_hash,
        }): Parameters<TransactionRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_transaction_token_transfers(
                chain_id,
                transaction_hash,
                GetTransactionTokenTransfersParams {
                    ..Default::default()
                },
            )
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get transaction internal transactions")]
    async fn get_transaction_internal_transactions(
        &self,
        Parameters(TransactionRequest {
            chain_id,
            transaction_hash,
        }): Parameters<TransactionRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_transaction_internal_transactions(chain_id, transaction_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get transaction logs")]
    async fn get_transaction_logs(
        &self,
        Parameters(TransactionRequest {
            chain_id,
            transaction_hash,
        }): Parameters<TransactionRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_transaction_logs(chain_id, transaction_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get transaction summary")]
    async fn get_transaction_summary(
        &self,
        Parameters(TransactionRequest {
            chain_id,
            transaction_hash,
        }): Parameters<TransactionRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_transaction_summary(chain_id, transaction_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get block info")]
    async fn get_block_info(
        &self,
        Parameters(BlockRequest {
            chain_id,
            number_or_hash,
        }): Parameters<BlockRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_block_info(chain_id, number_or_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get block transactions")]
    async fn get_block_transactions(
        &self,
        Parameters(BlockRequest {
            chain_id,
            number_or_hash,
        }): Parameters<BlockRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_block_transactions(chain_id, number_or_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get block withdrawals")]
    async fn get_block_withdrawals(
        &self,
        Parameters(BlockRequest {
            chain_id,
            number_or_hash,
        }): Parameters<BlockRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_block_withdrawals(chain_id, number_or_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List top 50 native coin holders")]
    async fn get_addresses(
        &self,
        Parameters(BaseRequest { chain_id }): Parameters<BaseRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self.block_scout_api.get_addresses(chain_id).await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get address info")]
    async fn get_address_info(
        &self,
        Parameters(AddressRequest {
            chain_id,
            address_hash,
        }): Parameters<AddressRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_address_info(chain_id, address_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get address counters")]
    async fn get_address_counters(
        &self,
        Parameters(AddressRequest {
            chain_id,
            address_hash,
        }): Parameters<AddressRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_address_counters(chain_id, address_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List latest 50 transactions of the address")]
    async fn get_address_transactions(
        &self,
        Parameters(AddressRequest {
            chain_id,
            address_hash,
        }): Parameters<AddressRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_address_transactions(
                chain_id,
                address_hash,
                GetAddressTransactionsParams { filter: "".into() },
            )
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List latest 50 token transfers of the address")]
    async fn get_address_token_transfers(
        &self,
        Parameters(AddressRequest {
            chain_id,
            address_hash,
        }): Parameters<AddressRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_address_token_transfers(
                chain_id,
                address_hash,
                GetAddressTokenTransfersParams {
                    ..Default::default()
                },
            )
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List latest 50 internal transactions of the address")]
    async fn get_address_internal_transactions(
        &self,
        Parameters(AddressRequest {
            chain_id,
            address_hash,
        }): Parameters<AddressRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_address_internal_transactions(
                chain_id,
                address_hash,
                GetAddressInternalTransactionsParams {
                    ..Default::default()
                },
            )
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get address tokens")]
    async fn get_address_tokens(
        &self,
        Parameters(AddressRequest {
            chain_id,
            address_hash,
        }): Parameters<AddressRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_address_tokens(
                chain_id,
                address_hash,
                GetAddressTokensParams {
                    ..Default::default()
                },
            )
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get address coin balance history")]
    async fn get_address_coin_balance_history(
        &self,
        Parameters(AddressRequest {
            chain_id,
            address_hash,
        }): Parameters<AddressRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_address_coin_balance_history(chain_id, address_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get address coin balance history by day")]
    async fn get_address_coin_balance_history_by_day(
        &self,
        Parameters(AddressRequest {
            chain_id,
            address_hash,
        }): Parameters<AddressRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_address_coin_balance_history_by_day(chain_id, address_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get address withdrawals")]
    async fn get_address_withdrawals(
        &self,
        Parameters(AddressRequest {
            chain_id,
            address_hash,
        }): Parameters<AddressRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_address_withdrawals(chain_id, address_hash)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get address NFTs")]
    async fn get_address_nfts(
        &self,
        Parameters(AddressRequest {
            chain_id,
            address_hash,
        }): Parameters<AddressRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_address_nfts(
                chain_id,
                address_hash,
                GetAddressNftsParams {
                    ..Default::default()
                },
            )
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get address NFT collections")]
    async fn get_address_nft_collections(
        &self,
        Parameters(AddressRequest {
            chain_id,
            address_hash,
        }): Parameters<AddressRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_address_nft_collections(
                chain_id,
                address_hash,
                GetAddressNftsParams {
                    ..Default::default()
                },
            )
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List top 50 tokens with the most holders")]
    async fn get_tokens(
        &self,
        Parameters(BaseRequest { chain_id }): Parameters<BaseRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_tokens(
                chain_id,
                GetTokensParams {
                    ..Default::default()
                },
            )
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get token info")]
    async fn get_token_info(
        &self,
        Parameters(TokenRequest {
            chain_id,
            token_address,
        }): Parameters<TokenRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_token_info(chain_id, token_address)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List latest 50 transfers of the token")]
    async fn get_token_transfers(
        &self,
        Parameters(TokenRequest {
            chain_id,
            token_address,
        }): Parameters<TokenRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_token_transfers(chain_id, token_address)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List top 50 holders of the token")]
    async fn get_token_holders(
        &self,
        Parameters(TokenRequest {
            chain_id,
            token_address,
        }): Parameters<TokenRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_token_holders(chain_id, token_address)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get token counters")]
    async fn get_token_counters(
        &self,
        Parameters(TokenRequest {
            chain_id,
            token_address,
        }): Parameters<TokenRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_token_counters(chain_id, token_address)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List first 50 instances of the NFT")]
    async fn get_token_instances(
        &self,
        Parameters(TokenRequest {
            chain_id,
            token_address,
        }): Parameters<TokenRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_token_instances(chain_id, token_address)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get NFT instance info")]
    async fn get_token_instance_info(
        &self,
        Parameters(TokenInstanceRequest {
            chain_id,
            token_address,
            token_id,
        }): Parameters<TokenInstanceRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_token_instance_info(chain_id, token_address, token_id)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List latest 50 transfers of the NFT instance")]
    async fn get_token_instance_transfers(
        &self,
        Parameters(TokenInstanceRequest {
            chain_id,
            token_address,
            token_id,
        }): Parameters<TokenInstanceRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_token_instance_transfers(chain_id, token_address, token_id)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "List fist 50 holders of the NFT instance")]
    async fn get_token_instance_holders(
        &self,
        Parameters(TokenInstanceRequest {
            chain_id,
            token_address,
            token_id,
        }): Parameters<TokenInstanceRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_token_instance_holders(chain_id, token_address, token_id)
            .await;
        Self::convert_result(rst)
    }

    #[tool(description = "Get the NFT instance transfers count")]
    async fn get_token_instance_transfers_count(
        &self,
        Parameters(TokenInstanceRequest {
            chain_id,
            token_address,
            token_id,
        }): Parameters<TokenInstanceRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rst = self
            .block_scout_api
            .get_token_instance_transfers_count(chain_id, token_address, token_id)
            .await;
        Self::convert_result(rst)
    }
}

#[tool_handler]
impl ServerHandler for OnChainData {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server provides a tool for query blockchains on-chain data".to_string(),
            ),
        }
    }
}
