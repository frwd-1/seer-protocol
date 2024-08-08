// src/local_node.rs
use crate::node_provider::NodeProvider;
use async_trait::async_trait;
use reth_exex::ExExNotification;
use reth_primitives::{Chain, TransactionSigned};
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;

pub struct LocalNode {
    chain: Arc<Chain>,
    notifications: Arc<Mutex<Receiver<ExExNotification>>>,
}

#[async_trait]
impl NodeProvider for LocalNode {
    async fn get_block_transactions(
        &self,
        block_number: u64,
    ) -> eyre::Result<Vec<TransactionSigned>> {
        // Fetch transactions from the local node's chain
        let transactions = self
            .chain
            .blocks_iter()
            .flat_map(|block_with_senders| block_with_senders.body.iter())
            .filter(|tx| tx.block_number() == block_number)
            .cloned()
            .collect();
        Ok(transactions)
    }

    fn notifications(&self) -> &Arc<Mutex<Receiver<ExExNotification>>> {
        &self.notifications
    }
}
