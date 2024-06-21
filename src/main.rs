mod db;
mod heuristics;

use crate::heuristics::sybil::Sybil;
use crate::heuristics::Heuristic;
use futures::Future;
use reth::api::FullNodeComponents;
use reth_exex::{ExExContext, ExExNotification};
use reth_node_ethereum::EthereumNode;
use reth_primitives::{address, Address, TransactionSigned};
use reth_provider::Chain;

const OP_BRIDGES: [Address; 6] = [
    address!("3154Cf16ccdb4C6d922629664174b904d80F2C35"),
    address!("3a05E5d33d7Ab3864D53aaEc93c8301C1Fa49115"),
    address!("697402166Fbf2F22E970df8a6486Ef171dbfc524"),
    address!("99C9fc46f92E8a1c0deC1b1747d010903E884bE1"),
    address!("735aDBbE72226BD52e818E7181953f42E3b0FF21"),
    address!("3B95bC951EE0f553ba487327278cAc44f29715E5"),
];

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
                        heuristic.apply_transaction(tx);
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

// I'll show you how great I am - Muhammad Ali
