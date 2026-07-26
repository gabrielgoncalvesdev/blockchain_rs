use blockchain_rs::Blockchain;


fn main() {
    let mut blockchain = Blockchain::new(3);
    blockchain.add_block("Alice pays Bob 10 BTC".into());
    blockchain.add_block("Bob pays Charlie 5 BTC".into());

    for block in &blockchain.blocks {
        println!("{block:?}");
    }

    match blockchain.validate() {
        Ok(()) => println!("chain válida! ✅"),
        Err(e) => println!("chain inválida: {e}"),
    }
}

