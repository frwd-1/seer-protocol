use crate::db::LabelDatabase;
use crate::heuristics::Heuristic;
use reth_exex::ExExNotification;

pub struct AirdropFarming;

impl Heuristic for AirdropFarming {
    fn apply(&self, notification: &ExExNotification, db: &mut LabelDatabase) {
        // Implement logic for detecting airdrop farming and update db
        db.insert("airdrop_farming_key", "airdrop_farming_value");
    }
}
