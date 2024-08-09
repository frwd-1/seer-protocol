use crate::capabilities::Capabilities;

use async_trait::async_trait;
use reqwest::Client;
use reth_primitives::TransactionSigned;
use serde::Serialize;
#[derive(Serialize)]
struct TransactionEvent {
    network: Network,
    from: String,
    to: String,
    // To add other fields
}

#[derive(Serialize)]
struct Network {
    name: String,
    // To add other fields
}

pub struct Sybil {
    pub client: Client,
    pub url: String,
}

// impl Sybil {
//     pub fn new(url: String) -> Self {
//         Sybil {
//             client: Client::new(),
//             url,
//         }
//     }
// }

#[async_trait]
impl Capabilities for Sybil {
    async fn apply_transaction(&self, tx_signed: &TransactionSigned) {
        // Extract the transaction from TransactionSigned
        let tx = &tx_signed.transaction;

        // Extract the `from` address by recovering the signer
        let from = tx_signed.recover_signer().unwrap_or_default();

        // Extract the `to` address
        let to = tx.to().unwrap_or_default();

        // Create the transaction event (even if we're not sending it, this could be useful)
        let transaction_event = TransactionEvent {
            network: Network {
                name: "your_network".to_string(),
            },
            from: format!("{:?}", from),
            to: format!("{:?}", to),
            // Populate other fields if needed
        };

        // Send the event to the external service
        let response = self
            .client
            .post(&self.url)
            .json(&transaction_event)
            .send()
            .await;

        // Handle the response
        match response {
            Ok(_) => {
                // Print the `from` and `to` addresses on successful HTTP response
                println!("Transaction successfully sent!");
                println!("From: {:?}", from);
                println!("To: {:?}", to);
            }
            Err(err) => {
                // Print the error if the request fails
                eprintln!("Error sending transaction event: {:?}", err);
            }
        }
    }
}
