pub mod ml;
pub mod sybil;
pub mod token;
use async_trait::async_trait;
use reth_primitives::TransactionSigned;

#[async_trait]
pub trait Capabilities: Send + Sync {
    async fn apply_transaction(&self, tx: &TransactionSigned);
}
