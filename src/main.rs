mod db;
mod heuristics;
mod p2p;
mod token;

use crate::db::LabelDatabase;
use crate::heuristics::{
    airdrop_farming::AirdropFarming, flow_through::FlowThrough, wash_trading::WashTrading,
    Heuristic,
};
use crate::p2p::P2PNetwork;
use crate::token::SeerToken;
use ethers::types::Address;
use eyre::Result;
use reth_exex::{ExExContext, ExExNotification};
use reth_node_api::FullNodeComponents;
use reth_node_ethereum::EthereumNode;
use std::collections::HashMap;
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

#[derive(Clone)]
struct Stake {
    amount: U256,
    active: bool,
}

async fn exex<Node: FullNodeComponents>(
    mut ctx: ExExContext<Node>,
    heuristics: Vec<Box<dyn Heuristic + Send + Sync>>,
    mut db: LabelDatabase,
    p2p_network: P2PNetwork,
    token: SeerToken,
    stakes: HashMap<Address, Stake>,
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
                // Handle new committed blocks
                for block in new {
                    for tx in &block.transactions {
                        // Extract `from` and `to` addresses from the transaction
                        let from = tx.from;
                        let to = tx.to.unwrap_or_default();

                        // Process each transaction with the heuristics
                        for heuristic in &heuristics {
                            heuristic.apply(tx, &mut db);
                        }

                        // Here you can also add any additional processing for the addresses
                        println!("Transaction from: {:?} to: {:?}", from, to);
                    }
                }
            }
            ExExNotification::ChainReorged { old: _, new: _ } => {
                // Not sure if I need this yet... Handle chain reorganization event
            }
            ExExNotification::ChainReverted { old: _ } => {
                // Not sure if I need this yet... Handle chain reverted event
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
    token: SeerToken,
    stakes: HashMap<Address, Stake>,
) -> impl Future<Output = eyre::Result<impl Future<Output = eyre::Result<()>>>> + Send {
    async move { Ok(async move { exex(ctx, heuristics, db, p2p_network, token, stakes).await }) }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Set up custom configuration
    let rpc_url = ""; // Update with the actual RPC URL from the Kurtosis enclave
    let custom_config = CustomNodeConfig::new(rpc_url.to_string());

    // Set the environment variable for the RPC URL if needed
    std::env::set_var("RETH_RPC_URL", custom_config.rpc_url());

    // Initialize heuristics
    let heuristics: Vec<Box<dyn Heuristic + Send + Sync>> = vec![
        Box::new(AirdropFarming::new(
            "http://127.0.0.1:5000/transaction".to_string(),
        )),
        Box::new(FlowThrough),
        Box::new(WashTrading),
    ];

    // Initialize the label database
    let db = LabelDatabase::new();

    // Initialize P2P network
    let mut p2p_network = P2PNetwork::new()?;

    // Initialize Seer token
    let token = SeerToken::new();

    // Initialize staking
    let mut stakes: HashMap<Address, Stake> = HashMap::new();

    // Add a stake
    let address = "0xEthereumAddress".parse().unwrap();
    stakes.insert(
        address,
        Stake {
            amount: U256::from(1000),
            active: true,
        },
    );

    // Runs the reth node
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("Minimal", |ctx| {
                exex_wrapper(ctx, heuristics.clone(), db, p2p_network, token, stakes)
            })
            .launch()
            .await?;

        handle.wait_for_node_exit().await?;
        Ok(())
    })
}
