use crate::db::LabelDatabase;
use crate::heuristics::Heuristic;
use reth_exex::ExExNotification;

pub struct FlowThrough;

impl Heuristic for FlowThrough {
    fn apply(&self, notification: &ExExNotification, db: &mut LabelDatabase) {
        // Implement logic for detecting flow through and update db
        db.insert("flow_through_key", "flow_through_value");
    }
}
