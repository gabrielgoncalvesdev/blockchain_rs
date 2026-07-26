use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BlockchainError {
    // The hash of the block does not match the recalculated hash 
    InvalidHash { index: u64 },
    // The block hash does not meet the difficulty requirement (does not have a valid proof-of-work)
    InsufficientWork {index: u64, difficulty: usize},
    // The previous_hash of the block does not match with the previous block's hash
    BrokenChain { index: u64, previous_index: u64 },
}

impl fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockchainError::InvalidHash {index} => {
                write!(f, "Block at index {} has an invalid hash.", index)
            }
            BlockchainError::InsufficientWork {index, difficulty} => {
                write!(f, "Block at index {} does not meet the difficulty requirement at {}.", index, difficulty)
            }
            BlockchainError::BrokenChain {index, previous_index} => {
                write!(f, "Block at index {} has a previous hash that does not match the hash of the previous block at previous index {}.", index, previous_index)
            }
        }
    }
}

impl std::error::Error for BlockchainError {}