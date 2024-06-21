use crate::heuristics::Heuristic;

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
impl Heuristic for Sybil {
    async fn apply_transaction(&self, tx_signed: &TransactionSigned) {
        // Extract the transaction from TransactionSigned
        let tx = &tx_signed.transaction;

        // Extract the `from` address by recovering the signer
        let from = tx_signed.recover_signer().unwrap_or_default();

        // Extract the `to` address
        let to = tx.to().unwrap_or_default();

        let transaction_event = TransactionEvent {
            network: Network {
                name: "your_network".to_string(),
            },
            from: format!("{:?}", from),
            to: format!("{:?}", to),
            // Populate other fields
        };

        let url = self.url.clone();
        let client = self.client.clone();
        tokio::spawn(async move {
            let response = client.post(&url).json(&transaction_event).send().await;

            match response {
                Ok(res) => println!("Response: {:?}", res),
                Err(err) => eprintln!("Error: {:?}", err),
            }
        });

        // db.insert("airdrop_farming_key", "airdrop_farming_value");
    }
}
