mod db;
mod heuristics;
mod p2p;

use crate::db::LabelDatabase;
use crate::heuristics::{
    airdrop_farming::AirdropFarming, money_laundering::MoneyLaundering, wash_trading::WashTrading,
    Heuristic,
};
use crate::p2p::P2PNetwork;
use eyre::Result;
use futures::StreamExt;
use reth_exex::{ExExContext, ExExNotification};
use reth_node_api::FullNodeComponents;
use reth_node_ethereum::EthereumNode;
use std::future::Future;
use tokio::task;

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

async fn exex<Node: FullNodeComponents>(
    mut ctx: ExExContext<Node>,
    heuristics: Vec<Box<dyn Heuristic + Send + Sync>>,
    mut db: LabelDatabase,
    p2p_network: P2PNetwork,
) -> Result<()> {
    let mut p2p_network = p2p_network;
    task::spawn(async move {
        if let Err(e) = p2p_network.run().await {
            eprintln!("Error running P2P network: {:?}", e);
        }
    });

    while let Some(notification) = ctx.notifications.recv().await {
        for heuristic in &heuristics {
            heuristic.apply(&notification, &mut db);
        }

        // Broadcast the updated state to peers
        p2p_network.broadcast("New state update".to_string());

        match &notification {
            ExExNotification::ChainCommitted { new: _ } => {
                // Handle chain committed event
            }
            ExExNotification::ChainReorged { old: _, new: _ } => {
                // Handle chain reorganization event
            }
            ExExNotification::ChainReverted { old: _ } => {
                // Handle chain reverted event
            }
        };
    }
    Ok(())
}

fn exex_wrapper<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
    heuristics: Vec<Box<dyn Heuristic + Send + Sync>>,
    db: LabelDatabase,
    p2p_network: P2PNetwork,
) -> impl Future<Output = eyre::Result<impl Future<Output = eyre::Result<()>>>> + Send {
    async move { Ok(async move { exex(ctx, heuristics, db, p2p_network).await }) }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Set up custom configuration
    let rpc_url = ""; // Update with RPC URL from the Kurtosis enclave
    let custom_config = CustomNodeConfig::new(rpc_url.to_string());

    // Set the environment variable for the RPC URL
    std::env::set_var("RETH_RPC_URL", custom_config.rpc_url());

    // Initialize heuristics
    let heuristics: Vec<Box<dyn Heuristic + Send + Sync>> = vec![
        Box::new(AirdropFarming),
        Box::new(MoneyLaundering),
        Box::new(WashTrading),
    ];

    // Initialize the label database
    let db = LabelDatabase::new();

    // Initialize P2P network
    let mut p2p_network = P2PNetwork::new()?;

    // Runs the reth node
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("Minimal", |ctx| {
                exex_wrapper(ctx, heuristics.clone(), db, p2p_network)
            })
            .launch()
            .await?;

        handle.wait_for_node_exit().await?;
        Ok(())
    })
}
