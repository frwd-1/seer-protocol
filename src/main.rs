use eyre::Result;
// use futures::StreamExt; // Ensure to import StreamExt for handling streams
use reth_exex::{ExExContext, ExExNotification};
use reth_node_api::FullNodeComponents;
use reth_node_ethereum::EthereumNode;
use std::future::Future;

#[derive(Debug)]
struct CustomNodeConfig {
    rpc_url: String,
}

impl CustomNodeConfig {
    pub fn new(rpc_url: String) -> Self {
        CustomNodeConfig { rpc_url }
    }

    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }
}

#[derive(Debug)]
struct CustomNodeConfig {
    rpc_url: String,
}

impl CustomNodeConfig {
    pub fn new(rpc_url: String) -> Self {
        CustomNodeConfig { rpc_url }
    }

    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }
}

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

// Defines a wrapper function to ensure correct return types
fn exex_wrapper<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> impl Future<Output = eyre::Result<impl Future<Output = eyre::Result<()>>>> + Send {
    async move { Ok(async move { exex(ctx).await }) }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Set up custom configuration
    let rpc_url = ""; // Update with the actual RPC URL from the Kurtosis enclave
    let custom_config = CustomNodeConfig::new(rpc_url.to_string());

    // Set the environment variable for the RPC URL if needed
    std::env::set_var("RETH_RPC_URL", custom_config.rpc_url());

    // Runs the reth node
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
