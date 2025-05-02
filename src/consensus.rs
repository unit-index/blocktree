use crate::block::Block;
use crate::error::BlocktreeError;
use chrono::Utc;

pub trait Consensus {
    fn mine_block(&self, block: Block) -> Result<Block, BlocktreeError>;
    fn adjust_difficulty(&self, block: &Block, previous_block: &Block) -> usize;
}

pub struct ProofOfWork {
    base_difficulty: usize,
    target_block_time: f64, // Seconds
}

impl ProofOfWork {
    pub fn new(base_difficulty: usize, target_block_time: f64) -> Self {
        ProofOfWork {
            base_difficulty,
            target_block_time,
        }
    }
}

impl Consensus for ProofOfWork {
    fn mine_block(&self, mut block: Block) -> Result<Block, BlocktreeError> {
        let target = "0".repeat(self.base_difficulty);
        let start_time = Utc::now().timestamp_millis() as f64 / 1000.0;
        loop {
            block.update_hash()?;
            if block.hash.starts_with(&target) {
                println!("Block mined on branch {}: {}", block.branch_id, block.hash);
                return Ok(block);
            }
            block.nonce += 1;
            let elapsed = (Utc::now().timestamp_millis() as f64 / 1000.0) - start_time;
            if elapsed > self.target_block_time {
                return Err(BlocktreeError::MiningTimeout(format!(
                    "Failed to mine block within {} seconds",
                    self.target_block_time
                )));
            }
        }
    }

    fn adjust_difficulty(&self, block: &Block, previous_block: &Block) -> usize {
        let time_taken = (block.timestamp - previous_block.timestamp) as f64 / 1000.0;
        let expected_time = self.target_block_time;
        if time_taken < expected_time / 2.0 {
            self.base_difficulty + 1
        } else if time_taken > expected_time * 2.0 {
            self.base_difficulty.saturating_sub(1).max(1)
        } else {
            self.base_difficulty
        }
    }
}
