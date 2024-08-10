#![allow(dead_code)]

use discv5::{enr::secp256k1::rand, Enr, Event, ListenConfig};
use reth::network::config::SecretKey;
use reth_discv5::{enr::EnrCombinedKeyWrapper, Config, Discv5};
use reth_network_peers::NodeRecord;
use reth_tracing::tracing::info;
use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{ready, Context, Poll},
};
use tokio::sync::mpsc;

pub(crate) mod cli_ext;

pub(crate) struct DiscV5ExEx {
    inner: Discv5,
    node_record: NodeRecord,
    events: mpsc::Receiver<discv5::Event>,
}

impl DiscV5ExEx {
    /// Starts a new discv5 node.
    pub async fn new(udp_port: u16, tcp_port: u16) -> eyre::Result<DiscV5ExEx> {
        let secret_key = SecretKey::new(&mut rand::thread_rng());

        let discv5_addr: SocketAddr = format!("127.0.0.1:{udp_port}").parse()?;
        let rlpx_addr: SocketAddr = format!("127.0.0.1:{tcp_port}").parse()?;

        let discv5_listen_config = ListenConfig::from(discv5_addr);
        let discv5_config = Config::builder(rlpx_addr)
            .discv5_config(discv5::ConfigBuilder::new(discv5_listen_config).build())
            .build();

        let (discv5, events, node_record) = Discv5::start(&secret_key, discv5_config).await?;
        Ok(Self {
            inner: discv5,
            events,
            node_record,
        })
    }

    /// Adds a node to the table if its not already present.
    pub fn add_node(&mut self, enr: Enr) -> eyre::Result<()> {
        let reth_enr: enr::Enr<SecretKey> = EnrCombinedKeyWrapper(enr.clone()).into();
        self.inner.add_node(reth_enr)?;
        Ok(())
    }

    /// Returns the local ENR of the discv5 node.
    pub fn local_enr(&self) -> Enr {
        self.inner.with_discv5(|discv5| discv5.local_enr())
    }
}

impl Future for DiscV5ExEx {
    type Output = eyre::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut();
        loop {
            match ready!(this.events.poll_recv(cx)) {
                Some(evt) => {
                    if let Event::SessionEstablished(enr, socket_addr) = evt {
                        info!(?enr, ?socket_addr, "Session established with a new peer.");
                    }
                }
                None => return Poll::Ready(Ok(())),
            }
        }
    }
}
