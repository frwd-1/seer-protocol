use eyre::Result;
use futures::{Future, FutureExt};
use reth_exex::{ExExContext, ExExEvent, ExExNotification};
use reth_node_api::FullNodeComponents;
use reth_tracing::tracing::{error, info};
use std::{
    pin::Pin,
    task::{ready, Context, Poll},
};

use crate::capabilities::{ml::MoneyLaundering, sybil::Sybil, Capabilities};
use crate::network::DiscV5ExEx;
use crate::utils::chain_utils::decode_chain_into_transactions;

/// The ExEx struct, representing the initialization and execution of the ExEx.
pub struct ExEx<Node: FullNodeComponents> {
    exex: ExExContext<Node>,
    disc_v5: DiscV5ExEx,
    capabilities: Vec<Box<dyn Capabilities>>,
}

impl<Node: FullNodeComponents> ExEx<Node> {
    pub fn new(exex: ExExContext<Node>, disc_v5: DiscV5ExEx) -> Self {
        let capabilities: Vec<Box<dyn Capabilities>> = vec![
            Box::new(Sybil {
                client: reqwest::Client::new(),
                url: "http://localhost:8080".to_string(),
            }),
            Box::new(MoneyLaundering {
                client: reqwest::Client::new(),
                url: "http://localhost:8080".to_string(),
            }),
        ];

        Self {
            exex,
            disc_v5,
            capabilities,
        }
    }
}

impl<Node: FullNodeComponents> Future for ExEx<Node> {
    type Output = Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll the Discv5 future until its drained
        loop {
            match self.disc_v5.poll_unpin(cx) {
                Poll::Ready(Ok(())) => {
                    info!("Discv5 task completed successfully");
                }
                Poll::Ready(Err(e)) => {
                    error!(?e, "Discv5 task encountered an error");
                    return Poll::Ready(Err(e));
                }
                Poll::Pending => {
                    // Exit match and continue to poll notifications
                    break;
                }
            }
        }

        // Continuously poll the ExExContext notifications
        loop {
            if let Some(notification) = ready!(self.exex.notifications.poll_recv(cx)) {
                match &notification {
                    ExExNotification::ChainCommitted { new } => {
                        info!(committed_chain = ?new.range(), "Received commit");
                        let transactions = decode_chain_into_transactions(&**new);
                        for tx in transactions {
                            for capability in &self.capabilities {
                                let fut = async {
                                    println!("Applying capability to transaction");
                                    capability.apply_transaction(tx).await;
                                };

                                futures::pin_mut!(fut);
                                match fut.poll(cx) {
                                    Poll::Ready(_) => {}
                                    Poll::Pending => {
                                        return Poll::Pending;
                                    }
                                }
                            }
                        }
                    }
                    ExExNotification::ChainReorged { old, new } => {
                        info!(from_chain = ?old.range(), to_chain = ?new.range(), "Received reorg");
                    }
                    ExExNotification::ChainReverted { old } => {
                        info!(reverted_chain = ?old.range(), "Received revert");
                    }
                }

                if let Some(committed_chain) = notification.committed_chain() {
                    self.exex
                        .events
                        .send(ExExEvent::FinishedHeight(committed_chain.tip().number))?;
                }
            }
        }
    }
}
