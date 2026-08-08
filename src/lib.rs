// Declara os módulo a serem roteados do projeto 
mod block;
mod error;
mod blockchain;
pub mod storage;
mod transaction;
mod hash;


// Exportar módulos publicamente e informações internar públicas dele para que possam ser usados externamente
pub use block::Block;
pub use error::{BlockchainError, StorageError};
pub use blockchain::{Blockchain, current_timestamp};
pub use transaction::Transaction;
pub use hash::{Hash, HashParseError};

