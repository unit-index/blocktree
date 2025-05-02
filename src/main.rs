use blocktree::blocktree::Blocktree;
use blocktree::network::MockNetwork;
use blocktree::transaction::Transaction;

#[tokio::main]
async fn main() {
    let network = Box::new(MockNetwork::new());
    let mut blocktree = Blocktree::new(network);

    // Add blocks to root branch
    for i in 1..=6 {
        println!("\nAdding block {} to root...", i);
        let tx = Transaction::new(format!("sender{}", i), format!("receiver{}", i), 100)
            .expect("Failed to create transaction");
        if let Err(e) = blocktree.add_block(vec![tx], "root") {
            println!("Error: {}", e);
        }
    }

    // Add blocks to new branches
    let branch_ids = blocktree.get_branches();
    for branch_id in branch_ids {
        println!("\nAdding block to {}...", branch_id);
        let tx = Transaction::new(
            format!("sender_{}", branch_id),
            format!("receiver_{}", branch_id),
            100,
        )
        .expect("Failed to create transaction");
        if let Err(e) = blocktree.add_block(vec![tx], &branch_id) {
            println!("Error: {}", e);
        }
    }

    // Validate branches
    println!("\nValidating branches:");
    for branch_id in blocktree.get_branches() {
        match blocktree.is_branch_valid(&branch_id) {
            Ok(valid) => println!("Branch {} valid? {}", branch_id, valid),
            Err(e) => println!("Error validating branch {}: {}", branch_id, e),
        }
    }

    // Print BKT supply
    println!("\nTotal BKT supply: {}", blocktree.get_bkt_supply());
}
