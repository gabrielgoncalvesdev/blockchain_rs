use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: usize,
}

impl Block {
    pub fn new(index: u64, timestamp: u64, transactions: Vec<Transaction>, previous_hash: String, difficulty: usize) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            difficulty,
        };
        block.mine();
        block
    }
    fn calculate_hash(&self) -> String {
        let txs = serde_json::to_string(&self.transactions).
        expect("Failed to serialize transactions");

        let input = format!("{}|{}|{}|{}|{}|{}", 
    self.index, self.timestamp, txs, self.previous_hash, self.nonce, self.difficulty); 
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn mine(&mut self) {
        let target = "0".repeat(self.difficulty);
        loop {
            self.hash = self.calculate_hash();
            if self.hash.starts_with(&target) {
                break;
            }
            self.nonce += 1;
        }
    }

    pub fn has_valid_hash(&self) -> bool {
        self.hash == self.calculate_hash()
    }

    pub fn meets_difficulty(&self) -> bool {
        self.hash.starts_with(&"0".repeat(self.difficulty))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_hash_e_deterministico() {
        // Só consigo chamar calculate_hash() (privado) porque o teste
        // está NO MESMO módulo. Isso é O motivo de unit tests ficarem no arquivo.
        let txs = vec![Transaction::new("Alice", "Bob", 100)];
        let block = Block::new(0, 1690000000, txs, "0".into(), 1);
        assert_eq!(block.calculate_hash(), block.calculate_hash());
        assert!(block.meets_difficulty());
    }
}