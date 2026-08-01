// Declara os módulo a serem roteados do projeto 
mod block;
mod error;
mod blockchain;

// Exportar módulos publicamente e informações internar públicas dele para que possam ser usados externamente
pub use block::Block;
pub use error::BlockchainError;
pub use blockchain::{Blockchain, current_timestamp};
