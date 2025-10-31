use bitcoincore_rpc::{Auth, Client, RpcApi};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a connection to the local Bitcoin node
    let rpc = Client::new(
        "http://127.0.0.1:18443", 
        Auth::UserPass("xoulomon".to_string(), "xoulomon".to_string())
    )?;

    // Fetch basic blockchain info
    let info = rpc.get_blockchain_info()?;
    println!("Chain: {}", info.chain);
    println!("Block height: {}", info.blocks);
    println!("Difficulty: {}", info.difficulty);

    Ok(())
}
