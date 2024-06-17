use crate::db::LabelDatabase;
use crate::heuristics::Heuristic;

use reqwest::Client;
use reth::primitives::TransactionSigned;
use reth_exex::ExExNotification;
use serde::Serialize;

#[derive(Serialize)]
struct TransactionEvent {
    network: Network,
    block_number: u64,
    from: String,
    to: String,
    // To add other fields
}

#[derive(Serialize)]
struct Network {
    name: String,
    // To add other fields
}

pub struct AirdropFarming {
    client: Client,
    url: String,
}

impl AirdropFarming {
    pub fn new(url: String) -> Self {
        AirdropFarming {
            client: Client::new(),
            url,
        }
    }
}

impl Heuristic for AirdropFarming {
    fn apply_transaction(
        &self,
        tx_signed: &TransactionSigned,
        db: &mut LabelDatabase,
        notification: &ExExNotification,
    ) {
        // Extract the transaction from TransactionSigned
        let tx = &tx_signed.transaction;

        // get this from the context?
        // For now, using a placeholder since block_number is not directly part of the transaction
        let block_number = 0; // TODO: Update this

        // Extract the `from` address by recovering the signer
        let from = tx_signed.recover_signer().unwrap_or_default();

        // Extract the `to` address
        let to = tx.to().unwrap_or_default();

        let transaction_event = TransactionEvent {
            network: Network {
                name: "your_network".to_string(),
            },
            block_number: block_number,
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

        db.insert("airdrop_farming_key", "airdrop_farming_value");
    }
}
