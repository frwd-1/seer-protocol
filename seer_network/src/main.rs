extern crate reth_network;

use reth_network::{config::rng_secret_key, NetworkConfig, NetworkManager};
use reth_primitives::mainnet_nodes;
use reth_provider::test_utils::NoopProvider;
use tokio::main;

#[main]
async fn main() {
    // Set up the block provider (for testing purposes, using a NoopProvider)
    let client = NoopProvider::default();

    // Generate a local key for encrypting sessions and identifying our node
    let local_key = rng_secret_key();

    // Configure the network
    let config = NetworkConfig::builder(local_key)
        .boot_nodes(mainnet_nodes())  // Use Ethereum mainnet nodes for bootstrapping
        .build(client);

    // Create the network manager instance
    let network = NetworkManager::new(config).await.unwrap();

    // Clone a handle to the network for later interaction
    let handle = network.handle().clone();

    // Spawn the network manager to run asynchronously
    tokio::task::spawn(network);

    // Example interaction with the network
    // e.g., handle.send(...) or handle.request(...)

    // Keep the main task alive to keep the network running.
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
