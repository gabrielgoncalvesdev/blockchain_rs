use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum BlockchainError {
    #[error("block at index {index} has a invalid hash (data was tampered)")]
    InvalidHash { index: u64 },

    #[error("block at index {index} does not meet the required difficulty of {difficulty}")]
    InsufficientWork { index: u64, difficulty: usize },

    #[error("block {index}: broken chain, previous hash does not match with the block hash {previous_index}")]
    BrokenChain {index: u64, previous_index: u64 },
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("I/O failure while acessing the storage: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("The loaded chain is invalid: {0}")]
    InvalidChain(#[from] BlockchainError),
}
