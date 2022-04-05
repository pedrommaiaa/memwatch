use ethers_providers::*;

use dotenv::dotenv;
use dotenv::var;

use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load files
    dotenv().ok();
    let node_url = var("NODE_URL").unwrap();

    // 2. Connect to WebSocket endpoint
    let provider = Provider::connect(node_url).await?.interval(Duration::from_millis(2000));

    let mut stream = provider.subscribe_pending_txs().await?.take(5);
    println!("Stream created! Entering loop...");

    while let Some(tx) = stream.next().await {
        let newTx = provider.get_transaction(tx).await?;
        
        if newTx.is_none() {
            continue
        }
        dbg!(newTx);
        
        // Use Serde Json
    }

    Ok(())
}
