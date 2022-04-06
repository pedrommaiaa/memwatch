use ethers_providers::*;
use dotenv::dotenv;
use dotenv::var;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 1. Load files
    dotenv().ok();
    let node_url = var("NODE_URL").unwrap();

    // 2. Connect to WebSocket endpoint
    let provider = Provider::connect(node_url).await?;

    let mut stream = provider.subscribe_pending_txs().await?;
    println!("Stream created! Entering loop...");

    while let Some(tx) = stream.next().await {
        let new_tx = provider.get_transaction(tx).await?;

        if new_tx.is_none() {
            continue
        }

        let pending_tx = new_tx.unwrap();
        println!("*** NEW TX DETECTED ***");
        println!("TX HASH: {:?}", pending_tx.hash);
        println!("FROM: {:?}", pending_tx.from);
        println!("TO: {:?}\n", pending_tx.to);
    }

    Ok(())
}
