use futures::Future;
use reth_exex::{ExExContext, ExExEvent, ExExNotification};
use reth_node_ethereum::EthereumNode;

async fn my_exex<Node: reth_node_api::FullNodeComponents>(
    mut ctx: ExExContext<Node>,
) -> eyre::Result<()> {
    while let Some(notification) = ctx.notifications.recv().await {
        match &notification {
            ExExNotification::ChainCommitted { new } => {
                // Handle chain committed events
            }
            ExExNotification::ChainReorged { old, new } => {
                // Handle chain reorganization events
            }
            ExExNotification::ChainReverted { old } => {
                // Handle chain reverted events
            }
        };
    }
    Ok(())
}

// setup to peer with other nodes

fn main() -> eyre::Result<()> {
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("MyExEx", |ctx| async move { my_exex(ctx) })
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    })
}
