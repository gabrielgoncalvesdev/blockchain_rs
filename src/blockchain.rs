use crate::hash::Hash;
use crate::block::Block;
use crate::error::BlockchainError;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const MIN_DIFFICULTY: usize = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    target_block_time: u64,
}

impl Blockchain {
    pub fn new(initial_difficulty: usize, target_block_time: u64, timestamp: u64, ) -> Self {
        let genesis = Block::new(0, timestamp, Vec::new(),  Hash::ZERO, initial_difficulty, 1);
        Blockchain {
            blocks: vec![genesis],
            target_block_time,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>, timestamp: u64, threads: usize) {
        let difficulty = self.next_difficulty(timestamp);
        let previous = self.blocks.last().expect("Blockchain should have at least one block");
        let new_index = previous.index + 1;
        let previous_hash = previous.hash;
        let block = Block::new(new_index, timestamp, transactions, previous_hash, difficulty, threads);
        self.blocks.push(block);
    }

    pub fn next_difficulty(&self, new_timestamp: u64) -> usize {
        let last  = self.blocks.last().expect("Blockchain should have at leat one block");
        let elapsed = new_timestamp.saturating_sub(last.timestamp);

        if elapsed < self.target_block_time {
            last.difficulty + 1
        } else if elapsed > self.target_block_time {
            last.difficulty.saturating_sub(1).max(MIN_DIFFICULTY)
        } else {
            last.difficulty
        }
    }

    pub fn validate(&self) -> Result<(), BlockchainError> {
        let genesis = &self.blocks[0];
        if !genesis.has_valid_hash() {
            return Err(BlockchainError::InvalidHash { index: genesis.index });
        }
        if !genesis.meets_difficulty() {
            return Err(BlockchainError::InsufficientWork { index: genesis.index, difficulty: genesis.difficulty });
        }

        for pair in self.blocks.windows(2) {
            let (previous, current) = (&pair[0], &pair[1]);

            if !current.has_valid_hash() {
                return Err(BlockchainError::InvalidHash { index: current.index });
            }
            if !current.meets_difficulty() {
                return Err(BlockchainError::InsufficientWork { index: current.index, difficulty: current.difficulty })
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
            .as_millis() as u64
    }