// src/block_provider.rs
use async_trait::async_trait;
use reth_primitives::TransactionSigned;

#[async_trait]
pub trait BlockProvider {
    async fn get_block_transactions(
        &self,
        block_number: u64,
    ) -> eyre::Result<Vec<TransactionSigned>>;
}
