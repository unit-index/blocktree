#[cfg(test)]
mod tests {
    use crate::blocktree::Blocktree;
    use crate::network::MockNetwork;
    use crate::transaction::Transaction;

    #[test]
    fn test_genesis_block() {
        let network = Box::new(MockNetwork::new());
        let blocktree = Blocktree::new(network);
        let root_branch = blocktree.storage.get_branch("root").unwrap();
        assert_eq!(root_branch.len(), 1);
        assert_eq!(root_branch[0].index, 0);
        assert_eq!(root_branch[0].transactions[0].sender, "genesis");
    }

    #[test]
    fn test_add_block_and_split() {
        let network = Box::new(MockNetwork::new());
        let mut blocktree = Blocktree::new(network);
        for i in 1..=5 {
            let tx = Transaction::new(
                format!("sender{}", i),
                format!("receiver{}", i),
                100,
            )
            .unwrap();
            blocktree
                .add_block(vec![tx], "root")
                .expect("Failed to add block");
        }
        assert!(blocktree.storage.get_branch("root.1").is_some());
        assert!(blocktree.storage.get_branch("root.2").is_some());
    }

    #[test]
    fn test_branch_validation() {
        let network = Box::new(MockNetwork::new());
        let mut blocktree = Blocktree::new(network);
        let tx = Transaction::new("sender".to_string(), "receiver".to_string(), 100).unwrap();
        blocktree
            .add_block(vec![tx], "root")
            .expect("Failed to add block");
        assert!(blocktree.is_branch_valid("root").unwrap());
    }
}