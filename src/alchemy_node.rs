// src/alchemy_client.rs
use crate::block_provider::BlockProvider;
use async_trait::async_trait;
use reqwest::Client;
use reth_primitives::TransactionSigned;
use serde_json::Value;

pub struct AlchemyClient {
    pub client: Client,
    pub url: String,
}

#[async_trait]
impl BlockProvider for AlchemyClient {
    async fn get_block_transactions(
        &self,
        block_number: u64,
    ) -> eyre::Result<Vec<TransactionSigned>> {
        let url = format!("{}/v2/your-api-key", self.url);
        let params = vec![Value::from(format!("0x{:x}", block_number))];
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": params,
            "id": 1,
        });

        let response = self.client.post(&url).json(&payload).send().await?;
        let block: Value = response.json().await?;

        // Process the JSON response to extract transactions
        let transactions = block["result"]["transactions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|tx| serde_json::from_value(tx.clone()).unwrap())
            .collect::<Vec<TransactionSigned>>();

        Ok(transactions)
    }
}
