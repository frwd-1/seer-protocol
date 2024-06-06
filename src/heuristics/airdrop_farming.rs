use crate::db::LabelDatabase;
use crate::heuristics::Heuristic;
use reqwest::Client;
use reth_exex::ExExNotification;
use serde::Serialize;

#[derive(Serialize)]
struct TransactionEvent {
    network: Network,
    block_number: u64,
    // Add other fields needed for processing
}

#[derive(Serialize)]
struct Network {
    name: String,
    // Add other fields if necessary
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
    fn apply(&self, notification: &ExExNotification, db: &mut LabelDatabase) {
        let transaction_event = TransactionEvent {
            network: Network {
                name: "your_network".to_string(),
            },
            block_number: notification.block_number(), // Or extract appropriately
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
