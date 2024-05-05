use log::{debug, info};
use reth_network::{config::rng_secret_key, NetworkConfig, NetworkManager};
use reth_primitives::mainnet_nodes;
use reth_provider::test_utils::NoopProvider;
use tokio::main;

#[tokio::main]
async fn main() {
    env_logger::init();
    debug!("Starting network setup...");
    // Set up the block provider (for testing purposes, using a NoopProvider)
    let client = NoopProvider::default();

    // Generate a local key for encrypting sessions and identifying our node
    let local_key = rng_secret_key();

    // Configure the network
    let config = NetworkConfig::builder(local_key)
        .boot_nodes(mainnet_nodes()) // Use Ethereum mainnet nodes for bootstrapping
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_connection() {
        // Setup your network configuration similarly to main
        let client = NoopProvider::default();
        let local_key = rng_secret_key();
        let config = NetworkConfig::builder(local_key)
            .boot_nodes(mainnet_nodes())
            .build(client);

        let network = NetworkManager::new(config)
            .await
            .expect("Failed to create network manager");

        // Implement your test
        // For example, you might check if the network connects to a node or how many nodes are connected
        assert!(
            network.peers().await.is_empty(),
            "Should initially connect to no peers"
        );
    }
}
