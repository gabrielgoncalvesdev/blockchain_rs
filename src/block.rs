use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, timestamp: u64, data: String, previous_hash: String, difficulty: usize) -> Self {
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.mine(difficulty);
        block
    }
    fn calculate_hash(&self) -> String {
        let input = format!("{}|{}|{}|{}|{}", 
    self.index, self.timestamp, self.data, self.previous_hash, self.nonce); 
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
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

    pub fn meets_difficulty(&self, difficulty: usize) -> bool {
        self.hash.starts_with(&"0".repeat(difficulty))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_hash_e_deterministico() {
        // Só consigo chamar calculate_hash() (privado) porque o teste
        // está NO MESMO módulo. Isso é O motivo de unit tests ficarem no arquivo.
        let block = Block::new(0, 1690000000, "test".into(), "0".into(), 1);
        assert_eq!(block.calculate_hash(), block.calculate_hash());
        assert!(block.meets_difficulty(1));
    }
}