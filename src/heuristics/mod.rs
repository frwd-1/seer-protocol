pub mod airdrop_farming;
pub mod flow_through;
pub mod wash_trading;

use crate::db::LabelDatabase;
use reth_exex::ExExNotification;

pub trait Heuristic {
    fn apply(&self, notification: &ExExNotification, db: &mut LabelDatabase);
}
