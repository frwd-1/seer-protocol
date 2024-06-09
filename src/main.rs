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
use alloy_primitives::{Address, U256};
use dotenv::dotenv;
use eyre::Result;
use log::{error, info};
use reth::exex::{ExExContext, ExExNotification};
use reth::reth_node_api::FullNodeComponents;
use reth::reth_node_ethereum::EthereumNode;
use std::collections::HashMap;
use std::env;
use std::future::Future;
use std::sync::Arc;
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

// proof of stake?
#[derive(Clone)]
struct Stake {
    amount: U256,
    active: bool,
}

async fn exex<Node: FullNodeComponents>(
    mut ctx: ExExContext<Node>,
    heuristics: Vec<Box<dyn Heuristic + Send + Sync>>,
    mut db: LabelDatabase,
    p2p_network: Arc<P2PNetwork>,
    token: SeerToken,
    stakes: HashMap<Address, Stake>,
) -> Result<()> {
    info!("Starting exex function");

    let p2p_network_clone = Arc::clone(&p2p_network);
    task::spawn(async move {
        if let Err(e) = p2p_network_clone.run().await {
            error!("Error running P2P network: {:?}", e);
        }
    });

    while let Some(notification) = ctx.notifications.recv().await {
        info!("Received notification: {:?}", notification);

        // Call heuristic apply for notification
        for heuristic in &heuristics {
            heuristic.apply(&notification, &mut db);
        }

        // Broadcast the updated state to peers
        p2p_network.broadcast("New state update".to_string());

        match &notification {
            ExExNotification::ChainCommitted { new } => {
                info!("Handling ChainCommitted notification");

                // Handle new committed blocks
                for block in new.blocks_iter() {
                    for tx in &block.body {
                        // Process each transaction with the heuristics
                        for heuristic in &heuristics {
                            heuristic.apply_transaction(tx, &mut db, &notification);
                        }
                    }
                }
            }
            ExExNotification::ChainReorged { old: _, new: _ } => {
                info!("Handling ChainReorged notification");
            }
            ExExNotification::ChainReverted { old: _ } => {
                info!("Handling ChainReverted notification");
            }
        };
    }
    Ok(())
}

fn exex_wrapper<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
    heuristics: Vec<Box<dyn Heuristic + Send + Sync>>,
    db: LabelDatabase,
    p2p_network: Arc<P2PNetwork>,
    token: SeerToken,
    stakes: HashMap<Address, Stake>,
) -> impl Future<Output = eyre::Result<impl Future<Output = eyre::Result<()>>>> + Send {
    async move {
        info!("Starting exex_wrapper");
        Ok(async move { exex(ctx, heuristics, db, p2p_network, token, stakes).await })
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Determine the environment (default to development)
    let environment = env::var("ENV").unwrap_or_else(|_| "development".to_string());
    info!("Running in {} environment", environment);

    let rpc_url = match environment.as_str() {
        "production" => env::var("PROD_RPC_URL").expect("PROD_RPC_URL must be set"),
        _ => env::var("DEV_RPC_URL").expect("DEV_RPC_URL must be set"),
    };
    info!("Using RPC URL: {}", rpc_url);

    let custom_config = CustomNodeConfig::new(rpc_url.clone());

    // Initialize heuristics
    let heuristics: Vec<Box<dyn Heuristic + Send + Sync>> = vec![
        Box::new(AirdropFarming::new(
            "http://127.0.0.1:5000/transaction".to_string(),
        )),
        Box::new(FlowThrough),
        Box::new(WashTrading),
    ];
    info!("Initialized heuristics");

    // Initialize the label database
    let db = LabelDatabase::new();
    info!("Initialized label database");

    // Initialize P2P network
    let p2p_network = Arc::new(P2PNetwork::new()?);
    info!("Initialized P2P network");

    // Initialize Seer token
    let token = SeerToken::new();
    info!("Initialized Seer token");

    // Initialize staking
    let mut stakes: HashMap<Address, Stake> = HashMap::new();

    // Add a stake
    let address: Address = "0xEthereumAddress".parse().unwrap();
    stakes.insert(
        address,
        Stake {
            amount: U256::from(1000),
            active: true,
        },
    );
    info!("Initialized staking");

    if environment == "production" {
        // Runs the reth node
        reth::cli::Cli::parse_args().run(|builder, _| async move {
            let handle = builder
                .node(EthereumNode::default())
                .install_exex("Minimal", |ctx| {
                    exex_wrapper(
                        ctx,
                        heuristics.clone(),
                        db,
                        p2p_network.clone(),
                        token,
                        stakes,
                    )
                })
                .launch()
                .await?;

            handle.wait_for_node_exit().await?;
            Ok(())
        })
    } else {
        // Setup ExEx context with the remote RPC
        let ctx = ExExContext::new(
            custom_config.rpc_url().to_string(),
            heuristics.clone(),
            db.clone(),
            p2p_network.clone(),
            token.clone(),
            stakes.clone(),
        )
        .await?;
        info!("Initialized ExEx context");

        // Call the exex function with the ExEx context
        exex(ctx, heuristics, db, p2p_network, token, stakes).await?;

        Ok(())
    }
}
