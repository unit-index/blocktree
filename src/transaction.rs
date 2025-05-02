use crate::error::BlocktreeError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub sender: String, // Simplified: public key or address
    pub receiver: String,
    pub amount: u64, // BKT units
    pub timestamp: i64,
    pub tx_id: String, // Hash of transaction
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64) -> Result<Self, BlocktreeError> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let mut tx = Transaction {
            sender,
            receiver,
            amount,
            timestamp,
            tx_id: String::new(),
        };
        tx.tx_id = tx.calculate_hash()?;
        Ok(tx)
    }

    pub fn calculate_hash(&self) -> Result<String, BlocktreeError> {
        let tx_json = serde_json::to_string(&self)
            .map_err(|e| BlocktreeError::SerializationError(e.to_string()))?;
        let mut hasher = Sha256::new();
        hasher.update(tx_json);
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn is_valid(&self) -> bool {
        // Placeholder: Add real validation (e.g., signature, balance)
        !self.sender.is_empty() && !self.receiver.is_empty() && self.amount > 0
    }
}