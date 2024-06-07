pub mod airdrop_farming;
pub mod flow_through;
pub mod wash_trading;

use crate::db::LabelDatabase;
use reth_exex::ExExNotification;
use reth_primitives::TransactionSigned;

pub trait Heuristic {
    // fn apply(&self, notification: &ExExNotification, db: &mut LabelDatabase);
    fn apply_transaction(
        &self,
        tx: &TransactionSigned,
        db: &mut LabelDatabase,
        notification: &ExExNotification,
    );
}
