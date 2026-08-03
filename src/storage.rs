use crate::blockchain::Blockchain;
use crate::error::StorageError;
use std::fs;
use std::path::Path;


pub fn save(chain: &Blockchain, path: &Path) -> Result<(), StorageError> {
    let json = serde_json::to_string(chain)?;
    fs::write(path, json)?;
    Ok(())  
}

pub fn load(path: &Path) -> Result<Blockchain, StorageError> {
    let json = fs::read_to_string(path)?;
    let chain: Blockchain = serde_json::from_str(&json)?;
    chain.validate()?;
    Ok(chain)
}