use crate::block::Block;
use crate::error::BlocktreeError;
use std::collections::HashMap;

pub trait Storage {
    fn save_block(&mut self, block: Block, branch_id: &str) -> Result<(), BlocktreeError>;
    fn get_branch(&self, branch_id: &str) -> Option<&Vec<Block>>;
    fn get_branch_keys(&self) -> Vec<String>;
}

pub struct InMemoryStorage {
    branches: HashMap<String, Vec<Block>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        InMemoryStorage {
            branches: HashMap::new(),
        }
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage for InMemoryStorage {
    fn save_block(&mut self, block: Block, branch_id: &str) -> Result<(), BlocktreeError> {
        self.branches
            .entry(branch_id.to_string())
            .or_default()
            .push(block);
        Ok(())
    }

    fn get_branch(&self, branch_id: &str) -> Option<&Vec<Block>> {
        self.branches.get(branch_id)
    }

    fn get_branch_keys(&self) -> Vec<String> {
        self.branches.keys().cloned().collect()
    }
}
