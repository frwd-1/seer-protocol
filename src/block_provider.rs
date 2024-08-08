// src/node_provider.rs
use async_trait::async_trait;
use reth_exex::ExExNotification;
use reth_primitives::TransactionSigned;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;

#[async_trait]
pub trait NodeProvider {
    async fn get_block_transactions(
        &self,
        block_number: u64,
    ) -> eyre::Result<Vec<TransactionSigned>>;
    fn notifications(&self) -> &Arc<Mutex<Receiver<ExExNotification>>>;
}
