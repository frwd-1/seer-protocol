use eyre::Result;
// use futures::StreamExt; // Ensure to import StreamExt for handling streams
use reth_exex::{ExExContext, ExExNotification};
use reth_node_api::FullNodeComponents;
use reth_node_ethereum::EthereumNode;
use std::future::Future;

async fn exex<Node: FullNodeComponents>(mut ctx: ExExContext<Node>) -> Result<()> {
    while let Some(notification) = ctx.notifications.recv().await {
        match &notification {
            ExExNotification::ChainCommitted { new: _ } => {
                // Handle chain committed event, ignoring the `new` variable
            }
            ExExNotification::ChainReorged { old: _, new: _ } => {
                // Handle chain reorganization event, ignoring the `old` and `new` variables
            }
            ExExNotification::ChainReverted { old: _ } => {
                // Handle chain reverted event, ignoring the `old` variable
            }
        };
    }
    Ok(())
}

// Define a wrapper function to ensure correct return types
fn exex_wrapper<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> impl Future<Output = eyre::Result<impl Future<Output = eyre::Result<()>>>> + Send {
    async move { Ok(async move { exex(ctx).await }) }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("Minimal", |ctx| exex_wrapper(ctx))
            .launch()
            .await?;

        handle.wait_for_node_exit().await?;
        Ok(())
    })
}
