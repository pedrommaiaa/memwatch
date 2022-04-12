use ethers_providers::*;
use dotenv::dotenv;
use dotenv::var;

use ethers_core::types::*;

#[derive(Debug)]
struct Transaction {
    from: Address,
    to: Option<Address>,
    value: U256,
}

#[tokio::main]
pub async fn listen() -> Result<(), Box<dyn std::error::Error>> {
    
    // 1. Load files
    dotenv().ok();
    let node_url = var("NODE_URL").unwrap();

    // 2. Connect to WebSocket endpoint
    let provider = Provider::connect(node_url).await?;

    let mut stream = provider.subscribe_pending_txs().await?;

    while let Some(tx) = stream.next().await {  
        let new_tx = provider.get_transaction(tx).await?;

        if !new_tx.is_none() {
            let pending_tx = new_tx.unwrap();

            let txs = Transaction {
                from: pending_tx.from,
                to: pending_tx.to,
                value: pending_tx.value,
            };

            println!("TXS FOUND");
            println!("{:?}", txs.from);
            println!("{:?}", txs.to);
            println!("{:?}\n", txs.value);
        }
    }

    Ok(())
}
