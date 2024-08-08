mod capabilities;
mod db;
mod heuristics;

use crate::capabilities::sybil::Sybil;
use crate::capabilities::Capabilities;
use futures::Future;
use reth::api::FullNodeComponents;
use reth_exex::{ExExContext, ExExNotification};
use reth_node_ethereum::EthereumNode;
use reth_primitives::TransactionSigned;
use reth_provider::Chain;
use std::env;

async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> Result<impl Future<Output = Result<(), eyre::Report>>> {
    Ok(exex(ctx))
}

async fn exex<Node: FullNodeComponents>(mut ctx: ExExContext<Node>) -> eyre::Result<()> {
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

#[tokio::main]
async fn main() -> Result<(), eyre::Report> {
    let node_type = env::var("NODE_TYPE").unwrap_or_else(|_| "reth".to_string());

    if node_type == "alchemy" {
        println!("Using Alchemy node");
        AlchemyNode::run().await?;
    } else {
        println!("Using Reth node");
        reth::cli::Cli::parse_args().run(|builder, _| async move {
            let handle = builder
                .node(EthereumNode::default())
                .install_exex("Seer", |ctx| async move { exex_init(ctx).await })
                .launch()
                .await?;

            handle.wait_for_node_exit().await?;
            Ok((exex(ctx).await, handle))
        });
    }

    Ok(())
}
