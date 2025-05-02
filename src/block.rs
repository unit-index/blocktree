use crate::error::BlocktreeError;
use crate::transaction::Transaction;
use chrono::Utc;
use merkle_tree::MerkleTree;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub branch_id: String,
    pub nonce: u64,
    pub merkle_root: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        branch_id: String,
    ) -> Result<Self, BlocktreeError> {
        let timestamp = Utc::now().timestamp_millis();
        let merkle_root = Self::calculate_merkle_root(&transactions)?;
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            branch_id,
            nonce: 0,
            merkle_root,
            hash: String::new(),
        };
        block.hash = block.calculate_hash()?;
        Ok(block)
    }

    pub fn calculate_merkle_root(transactions: &[Transaction]) -> Result<String, BlocktreeError> {
        if transactions.is_empty() {
            return Ok("0".to_string());
        }
        let leaves: Vec<String> = transactions
            .iter()
            .map(|tx| tx.tx_id.clone())
            .collect();
        let merkle_tree = MerkleTree::new(&leaves)
            .map_err(|e| BlocktreeError::SerializationError(e.to_string()))?;
        Ok(merkle_tree.root())
    }

    pub fn calculate_hash(&self) -> Result<String, BlocktreeError> {
        let block_json = serde_json::to_string(self)
            .map_err(|e| BlocktreeError::SerializationError(e.to_string()))?;
        let mut hasher = Sha256::new();
        hasher.update(block_json);
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn update_hash(&mut self) -> Result<(), BlocktreeError> {
        self.hash = self.calculate_hash()?;
        Ok(())
    }
}