use provchain_org::blockchain::Blockchain;
use tempfile::tempdir;

#[test]
fn test_simple_blockchain_persistence() {
    // Create a temporary directory for testing
    let temp_dir = tempdir().unwrap();
    let data_dir = temp_dir.path().join("simple_test");
    
    // Create a persistent blockchain
    let mut blockchain = Blockchain::new_persistent(&data_dir).unwrap();
    
    // Add a simple block
    let test_data = r#"
        @prefix ex: <http://example.org/> .
        ex:product ex:name "Test Product" .
    "#.to_string();
    
    blockchain.add_block(test_data);
    
    // Verify blockchain has the expected number of blocks
    assert_eq!(blockchain.chain.len(), 2); // Genesis + 1 added block
    
    // Save to disk
    blockchain.rdf_store.save_to_disk().unwrap();
    
    // Create a new blockchain instance to test loading from disk
    let blockchain2 = Blockchain::new_persistent(&data_dir).unwrap();
    
    // Verify data was loaded correctly
    assert_eq!(blockchain2.chain.len(), 2);
    
    // Verify block hashes match
    for i in 0..2 {
        assert_eq!(blockchain.chain[i].hash, blockchain2.chain[i].hash);
    }
}
