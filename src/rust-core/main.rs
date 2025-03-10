mod consensus;
mod network;
mod storage;

#[tokio::main]
async fn main() {
    println!("Distributed Inventory System - Rust Core");

    // Initialize and run the consensus module
    let consensus = consensus::Consensus::new();
    consensus.run().await;

    // Start the network module
    network::start_network().await;
}