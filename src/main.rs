use blockchain_rs::{Blockchain, current_timestamp, Transaction};


fn main() {
    let mut blockchain = Blockchain::new(3, 10, current_timestamp());
    blockchain.add_block(
        vec![
            Transaction::new("Alice", "Bob", 10_000),
            Transaction::new("Bob", "Gabriel", 1_000),
            ],
             current_timestamp()
            );

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

