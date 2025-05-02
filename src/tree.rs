use crate::block::Block;
use crate::clustering::Clustering;
use crate::error::BlocktreeError;
use crate::storage::Storage;

pub trait Tree {
    fn add_block<S: Storage>(
        &mut self,
        block: Block,
        branch_id: &str,
        storage: &mut S,
    ) -> Result<(), BlocktreeError>;
    fn split_branch<C: Clustering, S: Storage>(
        &mut self,
        branch_id: &str,
        clustering: &C,
        storage: &mut S,
    ) -> Result<(), BlocktreeError>;
    fn is_branch_valid<S: Storage>(
        &self,
        branch_id: &str,
        storage: &S,
    ) -> Result<bool, BlocktreeError>;
}

pub struct BlocktreeCore {
    split_interval: usize,
}

impl BlocktreeCore {
    pub fn new() -> Self {
        BlocktreeCore { split_interval: 5 }
    }
}

impl Tree for BlocktreeCore {
    fn add_block<S: Storage>(
        &mut self,
        block: Block,
        branch_id: &str,
        storage: &mut S,
    ) -> Result<(), BlocktreeError> {
        storage.save_block(block, branch_id)?;
        Ok(())
    }

    fn split_branch<C: Clustering, S: Storage>(
        &mut self,
        branch_id: &str,
        clustering: &C,
        storage: &mut S,
    ) -> Result<(), BlocktreeError> {
        let branch = storage
            .get_branch(branch_id)
            .ok_or_else(|| BlocktreeError::BranchNotFound(branch_id.to_string()))?;
        let last_block = branch
            .last()
            .cloned()
            .ok_or_else(|| BlocktreeError::BranchNotFound("Empty branch".to_string()))?;
        let fiedler_vector = clustering.compute_fiedler_vector()?;
        let (_cluster1, _cluster2) = clustering.partition_nodes(&fiedler_vector);
        let new_branch1 = format!("{}.1", branch_id);
        let new_branch2 = format!("{}.2", branch_id);
        storage.save_block(last_block.clone(), &new_branch1)?;
        storage.save_block(last_block, &new_branch2)?;
        // Note: In-memory storage doesn't remove old branch; add cleanup for disk storage
        println!(
            "Branch {} split into {} and {}",
            branch_id, new_branch1, new_branch2
        );
        Ok(())
    }

    fn is_branch_valid<S: Storage>(
        &self,
        branch_id: &str,
        storage: &S,
    ) -> Result<bool, BlocktreeError> {
        let chain = storage
            .get_branch(branch_id)
            .ok_or_else(|| BlocktreeError::BranchNotFound(branch_id.to_string()))?;
        for i in 1..chain.len() {
            let current = &chain[i];
            let previous = &chain[i - 1];
            let current_hash = current.calculate_hash()?;
            if current.hash != current_hash {
                return Ok(false);
            }
            if current.previous_hash != previous.hash {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
