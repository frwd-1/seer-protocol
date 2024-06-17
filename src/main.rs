use futures::Future;
use reth::api::FullNodeComponents;
use reth_exex::{ExExContext, ExExEvent, ExExNotification};
use reth_node_ethereum::EthereumNode;

async fn exex<Node: FullNodeComponents>(mut ctx: ExExContext<Node>) -> eyre::Result<()> {
    while let Some(notification) = ctx.notifications.recv().await {
        match &notification {
            ExExNotification::ChainCommitted { new } => {
                // Process the new block
                for block in new.blocks_iter() {
                    let block_number = block.number();
                    let transactions = &block.body;
                    // Process each transaction with the heuristics
                    for tx in transactions {
                        for heuristic in ctx.heuristics.iter() {
                            heuristic.apply_transaction(tx, &mut ctx.db, &notification);
                        }
                    }
                }
            }
            ExExNotification::ChainReorged { old, new } => {
                // Handle ChainReorged notification
            }
            ExExNotification::ChainReverted { old } => {
                // Handle ChainReverted notification
            }
        }
    }
    Ok(())
}

fn main() -> eyre::Result<()> {
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("Minimal", |ctx| async move { exex(ctx).await })
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    })
}
