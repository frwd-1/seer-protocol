use clap::Parser;
use exex::ExEx;
use eyre::Result;
use network::{cli_ext::Discv5ArgsExt, DiscV5ExEx};
use reth_node_ethereum::EthereumNode;

mod capabilities;
mod exex;
mod network;
mod utils;

fn main() -> Result<()> {
    reth::cli::Cli::<Discv5ArgsExt>::parse().run(|builder, args| async move {
        let tcp_port = args.tcp_port;
        let udp_port = args.udp_port;

        let handle = builder
            .node(EthereumNode::default())
            .install_exex("exex-discv5", move |ctx| async move {
                let disc_v5 = DiscV5ExEx::new(tcp_port, udp_port).await?;
                Ok(ExEx::new(ctx, disc_v5))
            })
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    })
}
