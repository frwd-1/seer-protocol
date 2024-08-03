// src/local_node_client.rs
use crate::block_provider::BlockProvider;
use async_trait::async_trait;
use reth_primitives::TransactionSigned;
use reth_provider::Chain;

pub struct LocalNodeClient {
    pub chain: Chain,
}

#[async_trait]
impl BlockProvider for LocalNodeClient {
    async fn get_block_transactions(
        &self,
        block_number: u64,
    ) -> eyre::Result<Vec<TransactionSigned>> {
        // Simulate fetching transactions from the local node's chain
        Ok(self
            .chain
            .blocks_iter()
            .flat_map(|block_with_senders| block_with_senders.body.iter())
            .filter(|tx| tx.block_number() == block_number)
            .cloned()
            .collect())
    }
}
