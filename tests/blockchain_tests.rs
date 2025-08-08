use tracechain::blockchain::Blockchain;

#[test]
fn test_blockchain_add_and_validate() {
    let mut bc = Blockchain::new();
    bc.add_block("test data".into());
    bc.add_block("more test data".into());

    assert!(bc.is_valid(), "Blockchain should be valid after adding blocks");
}

#[test]
fn test_blockchain_detect_tampering() {
    let mut bc = Blockchain::new();
    bc.add_block("original".into());

    // Tamper with a block
    bc.blocks[0].data = "tampered".into();

    assert!(!bc.is_valid(), "Blockchain should be invalid after tampering");
}
