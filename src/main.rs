mod db;
mod heuristics;

use crate::heuristics::sybil::Sybil;
use crate::heuristics::Heuristic;
use futures::Future;
use reth::api::FullNodeComponents;
use reth_exex::{ExExContext, ExExNotification};
use reth_node_ethereum::EthereumNode;
use reth_primitives::TransactionSigned;
use reth_provider::Chain;

async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> eyre::Result<impl Future<Output = eyre::Result<()>>> {
    Ok(exex(ctx))
}

async fn exex<Node: FullNodeComponents>(mut ctx: ExExContext<Node>) -> eyre::Result<()> {
    let heuristics: Vec<Box<dyn Heuristic>> = vec![Box::new(Sybil {
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
        }
    }
    Ok(())
}

fn decode_chain_into_transactions(chain: &Chain) -> impl Iterator<Item = &TransactionSigned> {
    println!("Decoding chain into transactions");
    chain
        .blocks_iter()
        .flat_map(|block_with_senders| block_with_senders.body.iter())
}

fn main() -> eyre::Result<()> {
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("Seer", |ctx| async move { exex_init(ctx).await })
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    })
}
