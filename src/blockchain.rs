use crate::block::Block;
use crate::error::BlockchainError;
use std::time::{SystemTime, UNIX_EPOCH};


pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize, timestamp: u64) -> Self {
        let genesis = Block::new(0, timestamp, "Genesis Block".into(), "0".into(), difficulty);
        Blockchain {
            blocks: vec![genesis],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String, timestamp: u64) {
        let difficulty = self.difficulty;
        let previous = self.blocks.last().expect("Blockchain should have at least one block");
        let new_index = previous.index + 1;
        let block = Block::new(new_index, timestamp, data, previous.hash.clone(), difficulty);
        self.blocks.push(block);
    }

    pub fn validate(&self) -> Result<(), BlockchainError> {
        let genesis = &self.blocks[0];
        if !genesis.has_valid_hash() {
            return Err(BlockchainError::InvalidHash { index: genesis.index });
        }
        if !genesis.meets_difficulty(self.difficulty) {
            return Err(BlockchainError::InsufficientWork { index: genesis.index, difficulty: self.difficulty });
        }

        for pair in self.blocks.windows(2) {
            let (previous, current) = (&pair[0], &pair[1]);

            if !current.has_valid_hash() {
                return Err(BlockchainError::InvalidHash { index: current.index });
            }
            if !current.meets_difficulty(self.difficulty) {
                return Err(BlockchainError::InsufficientWork { index: current.index, difficulty: self.difficulty })
            }
            if current.previous_hash != previous.hash {
                return Err(BlockchainError::BrokenChain { index: current.index, previous_index: previous.index });
            }
        }
        Ok(())
    }
}

/// Retornar o Unix time atual em segundos
    pub fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Not possible to get the timestamp before the UNIX_EPOCH (1970-01-01 00:00:00 UTC)")
            .as_secs()
    }