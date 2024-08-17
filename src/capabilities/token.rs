// capabilities/tokenID.rs
use crate::capabilities::Capabilities;
use async_trait::async_trait;
use reqwest::Client;
use reth::revm::interpreter::instructions::data;
use reth_primitives::TransactionSigned;
use reth_tracing::tracing_subscriber::registry::Data;
use serde::Serialize;

#[derive(Serialize)]
struct TransactionEvent {
    network: Network,
    from: String,
    to: String,
    data: String,
}

#[derive(Serialize)]
struct Network {
    name: String,
}

pub struct MoneyLaundering {
    pub client: Client,
    pub url: String,
}

#[async_trait]
impl Capabilities for MoneyLaundering {
    async fn apply_transaction(&self, tx_signed: &TransactionSigned) {
        let tx = &tx_signed.transaction;

        let from = tx_signed.recover_signer().unwrap_or_default();
        let to = tx.to().unwrap_or_default();
        let data: Vec<u8> = tx.input().to_vec();

        let transaction_event = TransactionEvent {
            network: Network {
                name: "your_network".to_string(),
            },
            from: format!("{:?}", from),
            to: format!("{:?}", to),
            data: format!("{:?}", data),
        };

        let response = self
            .client
            .post(&self.url)
            .json(&transaction_event)
            .send()
            .await;

        match response {
            Ok(_) => {
                println!("Money laundering heuristic successfully applied!");
                println!("From: {:?}", from);
                println!("To: {:?}", to);
            }
            Err(err) => {
                eprintln!("Error applying money laundering heuristic: {:?}", err);
            }
        }
    }
}
