use std::fmt;

#[derive(Debug)]
pub enum BlocktreeError {
    BranchNotFound(String),
    InvalidHash(String),
    InvalidPreviousHash(String),
    SerializationError(String),
    MiningTimeout(String),
    ClusteringError(String),
    TransactionError(String),
    NetworkError(String),
    StorageError(String),
}

impl fmt::Display for BlocktreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlocktreeError::BranchNotFound(id) => write!(f, "Branch not found: {}", id),
            BlocktreeError::InvalidHash(msg) => write!(f, "Invalid hash: {}", msg),
            BlocktreeError::InvalidPreviousHash(msg) => write!(f, "Invalid previous hash: {}", msg),
            BlocktreeError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            BlocktreeError::MiningTimeout(msg) => write!(f, "Mining timeout: {}", msg),
            BlocktreeError::ClusteringError(msg) => write!(f, "Clustering error: {}", msg),
            BlocktreeError::TransactionError(msg) => write!(f, "Transaction error: {}", msg),
            BlocktreeError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            BlocktreeError::StorageError(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl std::error::Error for BlocktreeError {}
