pub mod sybil;
use reth_primitives::TransactionSigned;

pub trait Heuristic {
    // fn apply(&self, notification: &ExExNotification, db: &mut LabelDatabase);
    fn apply_transaction(&self, tx: &TransactionSigned);
}
