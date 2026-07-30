use blockchain_rs::{Blockchain, current_timestamp};


fn main() {
    let mut blockchain = Blockchain::new(3, current_timestamp());
    blockchain.add_block("Alice pays Bob 10 BTC".into(), current_timestamp());
    blockchain.add_block("bob pays gabriel 100 BTC".into(), current_timestamp());

    for block in &blockchain.blocks {
        println!("{block:?}");
    }

    match blockchain.validate() {
        Ok(()) => println!("chain válida! ✅"),
        Err(e) => println!("chain inválida: {e}"),
    }
}

