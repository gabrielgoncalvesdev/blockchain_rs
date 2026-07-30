mod block;
mod error;
mod blockchain;

// Exportar módulos publicamente 
pub use block::Block;
pub use error::BlockchainError;
pub use blockchain::{Blockchain, current_timestamp};
