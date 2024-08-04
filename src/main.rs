mod alchemy_node;
mod capabilities;
mod config;
mod db;
mod local_node;
mod node_provider;

use crate::node_provider::NodeProvider;
use crate::capabilities::sybil::Sybil;
use crate::capabilities::Capabilities;
use futures::Future;
use reth::api::FullNodeComponents;
use reth_exex::{ExExContext, ExExNotification};
use reth_node_ethereum::EthereumNode;
use reth_primitives::TransactionSigned;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
    node_provider: Arc<dyn NodeProvider>,
) -> eyre::Result<impl Future<Output = eyre::Result<()>>> {
    Ok(exex(ctx, node_provider))
}

async fn exex<Node: FullNodeComponents>(
    mut ctx: ExExContext<Node>,
    node_provider: Arc<dyn NodeProvider>,
) -> eyre::Result<()> {
    let heuristics: Vec<Box<dyn Capabilities>> = vec![Box::new(Sybil {
        client: reqwest::Client::new(),
        url: "http://localhost:8080".to_string(),
    })];

    while let Some(notification) = node_provider.notifications().lock().await.recv().await {
        match &notification {
            ExExNotification::ChainCommitted { new } => {
                let block_number = new.number();
                let transactions = node_provider.get_block_transactions(block_number).await?;
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

fn main() -> eyre::Result<()> {
    let use_local_node = true; // Or false, based on your requirement
    let alchemy_url = Some("https://eth-mainnet.alchemyapi.io/v2/your-api-key".to_string());
    let local_node_url = Some("http://localhost:8545".to_string());

    let config = Config::new(use_local_node, alchemy_url, local_node_url);

    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let node_provider: Arc<dyn NodeProvider> = if config.use_local_node {
            // Initialize local node
            let chain = // Initialize your local chain here
            Arc::new(LocalNode {
                chain: Arc::new(chain),
                notifications: Arc::new(Mutex::new(/* initialize receiver here */)),
            })
        } else {
            // Initialize Alchemy node
            Arc::new(AlchemyNode {
                client: reqwest::Client::new(),
                url: config.alchemy_url.expect("Alchemy URL must be set if use_local_node is false"),
                notifications: Arc::new(Mutex::new(/* initialize receiver here */)),
            })
        };

        let handle = builder
            .node(EthereumNode::default())
            .install_exex("Seer", |ctx| async move { exex_init(ctx, node_provider).await })
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    })
}
