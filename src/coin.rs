pub struct Coin {
    pub supply: u64,
    base_reward: u64,
    decay_factor: f64, // For exponential decay
    blocks_mined: u64,
}

impl Coin {
    pub fn new() -> Self {
        Coin {
            supply: 0,
            base_reward: 50, // Starting reward
            decay_factor: 0.999, // Decay per block
            blocks_mined: 0,
        }
    }

    pub fn mine_reward(&mut self) -> u64 {
        let reward = (self.base_reward as f64 * self.decay_factor.powi(self.blocks_mined as i32)) as u64;
        self.supply += reward;
        self.blocks_mined += 1;
        reward
    }

    pub fn get_supply(&self) -> u64 {
        self.supply
    }
}