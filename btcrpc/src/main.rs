use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoin::BlockHash;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a connection to the local Bitcoin node
    let rpc = Client::new(
        "http://127.0.0.1:18443", 
        Auth::UserPass("xoulomon".to_string(), "xoulomon".to_string())
    )?;

    // Fetch basic blockchain info
    // let info = rpc.get_blockchain_info()?;
    // println!("Chain: {}", info.chain);
    // println!("Block height: {}", info.blocks);
    // println!("Difficulty: {}", info.difficulty);

        // Get wallet balance
    // let balance = rpc.get_balance(None, None)?;
    // println!("Wallet balance: {:.8} BTC", balance.to_btc());

    // Get the best block hash
    // let best_hash: BlockHash = rpc.get_best_block_hash()?;
    // println!("Best block hash: {}", best_hash);

    // // Fetch block details
    // let block = rpc.get_block(&best_hash)?;
    // println!("Block contains {} transactions", block.txdata.len());

    // Network Info
    let network_info = rpc.get_network_info()?;
    println!("Version: {}", network_info.version);
    println!("Connections: {}", network_info.connections);
    // println!("Relay Fee: {:?}", network_info.relayfee);


    Ok(())
}
