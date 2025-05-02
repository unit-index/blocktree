use crate::block::Block;
use crate::error::BlocktreeError;
use tokio::sync::mpsc;

pub trait Network {
    fn broadcast_block(&self, block: Block) -> Result<(), BlocktreeError>;
    fn get_latency(&self, node1: u32, node2: u32) -> f64;
}

pub struct MockNetwork {
    sender: mpsc::Sender<Block>,
    receiver: mpsc::Receiver<Block>,
}

impl MockNetwork {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(100);
        MockNetwork { sender, receiver }
    }
}

impl Network for MockNetwork {
    fn broadcast_block(&self, block: Block) -> Result<(), BlocktreeError> {
        self.sender
            .try_send(block)
            .map_err(|e| BlocktreeError::NetworkError(e.to_string()))?;
        Ok(())
    }

    fn get_latency(&self, _node1: u32, _node2: u32) -> f64 {
        rand::thread_rng().gen_range(10.0..100.0) // Mocked latency
    }
}
