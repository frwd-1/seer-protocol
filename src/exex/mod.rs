pub mod init;

use crate::capabilities::sybil::Sybil;
use crate::capabilities::Capabilities;
use crate::utils::chain_utils::decode_chain_into_transactions;
use eyre::Result;
use reth::api::FullNodeComponents;
use reth_exex::{ExExContext, ExExNotification};

pub async fn exex<Node: FullNodeComponents>(mut ctx: ExExContext<Node>) -> Result<()> {
    let heuristics: Vec<Box<dyn Capabilities>> = vec![Box::new(Sybil {
        client: reqwest::Client::new(),
        url: "http://localhost:8080".to_string(),
    })];

    while let Some(notification) = ctx.notifications.recv().await {
        match &notification {
            ExExNotification::ChainCommitted { new } => {
                let transactions = decode_chain_into_transactions(&**new);
                for tx in transactions {
                    for heuristic in &heuristics {
                        println!("Applying heuristic to transaction");
                        heuristic.apply_transaction(tx).await;
                    }
                }
            }
            ExExNotification::ChainReorged { old: _, new: _ } => {
                // Handle ChainReorged notification
            }
            ExExNotification::ChainReverted { old: _ } => {
                // Handle ChainReverted notification
            }
        };

        // if let Some(committed_chain) = notification.committed_chain() {
        //     ctx.events
        //         .send(ExExEvent::FinishedHeight(committed_chain.tip().number))?;
        // }
    }

    Ok(())
}
