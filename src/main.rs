mod capabilities;
mod db;
mod exex;
mod utils;

use exex::init::exex_init;
use eyre::Result;
use reth_node_ethereum::EthereumNode;

fn main() -> Result<()> {
    println!("Using Reth node");
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("Seer", exex_init)
            .launch()
            .await?;

        handle.wait_for_node_exit().await?;

        Ok::<(), eyre::Report>(())
    })?;

    Ok(())
}
