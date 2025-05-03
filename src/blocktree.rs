use crate::block::Block;
use crate::clustering::SpectralClustering;
use crate::coin::Coin;
use crate::consensus::{Consensus, ProofOfWork};
use crate::error::BlocktreeError;
use crate::network::Network;
use crate::storage::{InMemoryStorage, Storage};
use crate::transaction::Transaction;
use crate::tree::{BlocktreeCore, Tree};

pub struct Blocktree {
    tree: BlocktreeCore,
    consensus: ProofOfWork,
    clustering: SpectralClustering,
    coin: Coin,
    storage: InMemoryStorage,
    network: Box<dyn Network>,
}

impl Blocktree {
    pub fn new(network: Box<dyn Network>) -> Self {
        let mut storage = InMemoryStorage::new();
        let genesis = Block::new(
            0,
            vec![Transaction::new("genesis".to_string(), "genesis".to_string(), 0).unwrap()],
            "0".to_string(),
            "root".to_string(),
        )
        .unwrap();
        storage
            .save_block(genesis, "root")
            .expect("Failed to save genesis block");
        Blocktree {
            tree: BlocktreeCore::new(),
            consensus: ProofOfWork::new(2, 0.2),
            clustering: SpectralClustering::new(10),
            coin: Coin::new(),
            storage,
            network,
        }
    }

    pub fn add_block(
        &mut self,
        transactions: Vec<Transaction>,
        branch_id: &str,
    ) -> Result<(), BlocktreeError> {
        let branch = self
            .storage
            .get_branch(branch_id)
            .ok_or_else(|| BlocktreeError::BranchNotFound(branch_id.to_string()))?;
        let last_block = branch
            .last()
            .ok_or_else(|| BlocktreeError::BranchNotFound("Empty branch".to_string()))?;
        let new_block = Block::new(
            last_block.index + 1,
            transactions,
            last_block.hash.clone(),
            branch_id.to_string(),
        )?;
        let mined_block = self.consensus.mine_block(new_block)?;
        self.tree
            .add_block(mined_block.clone(), branch_id, &mut self.storage)?;
        self.network.broadcast_block(mined_block)?;
        self.coin.mine_reward();
        if self.storage.get_branch(branch_id).unwrap().len() >= self.tree.get_split_interval() {
            self.tree
                .split_branch(branch_id, &self.clustering, &mut self.storage)?;
        }
        Ok(())
    }

    pub fn is_branch_valid(&self, branch_id: &str) -> Result<bool, BlocktreeError> {
        self.tree.is_branch_valid(branch_id, &self.storage)
    }

    pub fn get_bkt_supply(&self) -> u64 {
        self.coin.get_supply()
    }

    pub fn get_branches(&self) -> Vec<String> {
        self.storage.get_branch_keys()
    }
}
