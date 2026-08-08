use blockchain_rs::{Blockchain, current_timestamp, Transaction, storage};
use::std::path::Path;
use std::time::Instant;

fn main() {

    let threads = std::thread::available_parallelism()
    .map(|n| n.get())
    .unwrap_or(1); 
    println!("Running with {threads} threads");

    let mut blockchain = Blockchain::new(5, 10, current_timestamp());

    let start = Instant::now();
    blockchain.add_block(
        vec![
            Transaction::new("Alice", "Bob", 10_000),
            ],
             current_timestamp(),
             threads
            );
    println!("Block mined in {:?}", start.elapsed());

    let path = Path::new("chain.json");

    match storage::save(&blockchain, path) {
        Ok(()) => println!("chain saved into {}", path.display()),
        Err(e) => println!("Failure to save: {e}"),
    }

    match storage::load(path) {
        Ok(loaded) => 
            println!("Chain loaded with {} blocks", loaded.blocks.len()),
            Err(e) => println!("Failed to load: {e}"),
    }

    for block in &blockchain.blocks {
        println!(
            "#{} dif={} txs={} nonce={} hash={} ",
            block.index, block.difficulty, block.transactions.len(), block.nonce, block.hash
        );
    }

    match blockchain.validate() {
        Ok(()) => println!("chain válida! ✅"),
        Err(e) => println!("chain inválida: {e}"),
    }
}

