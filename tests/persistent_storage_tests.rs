//! Persistent Storage Tests
//! 
//! Tests for blockchain persistence, backup/restore functionality,
//! and storage integrity validation.

use provchain_org::blockchain::Blockchain;
use tempfile::TempDir;
use std::fs;
use anyhow::Result;

#[test]
fn test_persistent_blockchain_creation() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let data_path = temp_dir.path().join("blockchain_data");
    
    // Create a new persistent blockchain
    let mut blockchain = Blockchain::new_persistent(&data_path)?;
    
    // Add some test data
    let test_data = r#"
    @prefix : <http://example.org/> .
    @prefix tc: <http://provchain.org/trace#> .
    
    :batch001 tc:product "Test Product" ;
              tc:origin "Test Farm" ;
              tc:status "Created" .
    "#;
    
    blockchain.add_block(test_data.to_string());
    blockchain.flush()?;
    
    // Verify the blockchain is valid
    assert!(blockchain.is_valid());
    assert_eq!(blockchain.chain.len(), 2); // Genesis + 1 data block
    
    // Verify data directory was created
    assert!(data_path.exists());
    
    Ok(())
}

#[test]
fn test_blockchain_persistence_across_restarts() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let data_path = temp_dir.path().join("blockchain_data");
    
    let test_data = r#"
    @prefix : <http://example.org/> .
    @prefix tc: <http://provchain.org/trace#> .
    
    :batch001 tc:product "Persistent Product" ;
              tc:origin "Persistent Farm" ;
              tc:batchId "PERSIST001" .
    "#;
    
    // Create blockchain and add data
    {
        let mut blockchain = Blockchain::new_persistent(&data_path)?;
        blockchain.add_block(test_data.to_string());
        blockchain.flush()?;
        assert_eq!(blockchain.chain.len(), 2);
    }
    
    // Recreate blockchain from same directory
    {
        let blockchain = Blockchain::new_persistent(&data_path)?;
        assert_eq!(blockchain.chain.len(), 2);
        assert!(blockchain.is_valid());
        
        // Verify the data is still there
        let query = r#"
        PREFIX tc: <http://provchain.org/trace#>
        SELECT ?product WHERE {
            ?batch tc:product ?product .
            FILTER(?product = "Persistent Product")
        }
        "#;
        
        let results = blockchain.rdf_store.query(query);
        // Should find the persistent product
        if let oxigraph::sparql::QueryResults::Solutions(solutions) = results {
            let results: Vec<_> = solutions.collect();
            assert!(!results.is_empty(), "Should find persistent product data");
        }
    }
    
    Ok(())
}

#[test]
fn test_blockchain_backup_and_restore() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let data_path = temp_dir.path().join("blockchain_data");
    let backup_path = temp_dir.path().join("backup");
    let restore_path = temp_dir.path().join("restored_data");
    
    let test_data = r#"
    @prefix : <http://example.org/> .
    @prefix tc: <http://provchain.org/trace#> .
    
    :batch001 tc:product "Backup Test Product" ;
              tc:origin "Backup Test Farm" ;
              tc:batchId "BACKUP001" .
    
    :batch002 tc:product "Another Product" ;
              tc:origin "Another Farm" ;
              tc:batchId "BACKUP002" .
    "#;
    
    // Create blockchain with test data
    let original_blockchain = {
        let mut blockchain = Blockchain::new_persistent(&data_path)?;
        blockchain.add_block(test_data.to_string());
        blockchain.flush()?;
        
        // Create backup
        let backup_info = blockchain.create_backup()?;
        println!("Created backup: {:?}", backup_info);
        
        blockchain
    };
    
    // Restore from backup to new location
    let restored_blockchain = Blockchain::restore_from_backup(&backup_path, &restore_path)?;
    
    // Verify restored blockchain matches original
    assert_eq!(restored_blockchain.chain.len(), original_blockchain.chain.len());
    assert!(restored_blockchain.is_valid());
    
    // Verify data integrity
    for (original, restored) in original_blockchain.chain.iter().zip(restored_blockchain.chain.iter()) {
        assert_eq!(original.index, restored.index);
        assert_eq!(original.hash, restored.hash);
        assert_eq!(original.previous_hash, restored.previous_hash);
        assert_eq!(original.data, restored.data);
    }
    
    Ok(())
}

#[test]
fn test_storage_statistics() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let data_path = temp_dir.path().join("blockchain_data");
    
    let mut blockchain = Blockchain::new_persistent(&data_path)?;
    
    // Add multiple blocks
    for i in 0..10 {
        let test_data = format!(r#"
        @prefix : <http://example.org/> .
        @prefix tc: <http://provchain.org/trace#> .
        
        :batch{:03} tc:product "Product {}" ;
                   tc:origin "Farm {}" ;
                   tc:batchId "BATCH{:03}" .
        "#, i, i, i, i);
        
        blockchain.add_block(test_data);
    }
    
    blockchain.flush()?;
    
    // Get storage statistics
    let stats = blockchain.get_storage_stats()?;
    
    // Verify statistics make sense
    assert!(stats.disk_usage_bytes.unwrap_or(0) > 0);
    assert!(stats.quad_count > 0);
    
    println!("Storage stats: {:?}", stats);
    
    Ok(())
}

#[test]
fn test_database_integrity_check() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let data_path = temp_dir.path().join("blockchain_data");
    
    let mut blockchain = Blockchain::new_persistent(&data_path)?;
    
    // Add test data
    let test_data = r#"
    @prefix : <http://example.org/> .
    @prefix tc: <http://provchain.org/trace#> .
    
    :batch001 tc:product "Integrity Test Product" ;
              tc:origin "Integrity Test Farm" .
    "#;
    
    blockchain.add_block(test_data.to_string());
    blockchain.flush()?;
    
    // Check integrity
    let integrity_report = blockchain.check_integrity()?;
    
    assert_eq!(integrity_report.errors.len(), 0);
    
    println!("Integrity report: {:?}", integrity_report);
    
    Ok(())
}

#[test]
fn test_database_optimization() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let data_path = temp_dir.path().join("blockchain_data");
    
    let mut blockchain = Blockchain::new_persistent(&data_path)?;
    
    // Add multiple blocks to create data to optimize
    for i in 0..20 {
        let test_data = format!(r#"
        @prefix : <http://example.org/> .
        @prefix tc: <http://provchain.org/trace#> .
        
        :batch{:03} tc:product "Optimization Product {}" ;
                   tc:origin "Optimization Farm {}" .
        "#, i, i, i);
        
        blockchain.add_block(test_data);
    }
    
    blockchain.flush()?;
    
    // Get stats before optimization
    let stats_before = blockchain.get_storage_stats()?;
    
    // Optimize database
    blockchain.optimize()?;
    
    // Get stats after optimization
    let stats_after = blockchain.get_storage_stats()?;
    
    // Verify blockchain is still valid after optimization
    assert!(blockchain.is_valid());
    
    // Stats should be available (optimization might not change size significantly in small test)
    assert!(stats_after.disk_usage_bytes.is_some());
    
    println!("Stats before optimization: {:?}", stats_before);
    println!("Stats after optimization: {:?}", stats_after);
    
    Ok(())
}

#[test]
fn test_concurrent_access_safety() -> Result<()> {
    use std::sync::Arc;
    use std::thread;
    
    let temp_dir = TempDir::new()?;
    let data_path = temp_dir.path().join("blockchain_data");
    
    // Create initial blockchain
    {
        let mut blockchain = Blockchain::new_persistent(&data_path)?;
        blockchain.add_block("Initial data".to_string());
        blockchain.flush()?;
    }
    
    // Test concurrent read access
    let data_path = Arc::new(data_path);
    let mut handles = vec![];
    
    for i in 0..5 {
        let path = Arc::clone(&data_path);
        let handle = thread::spawn(move || -> Result<()> {
            let blockchain = Blockchain::new_persistent(&*path)?;
            
            // Verify blockchain is valid
            assert!(blockchain.is_valid());
            assert!(blockchain.chain.len() >= 2); // Genesis + initial data
            
            // Perform a query
            let query = "SELECT * WHERE { ?s ?p ?o } LIMIT 10";
            let _results = blockchain.rdf_store.query(query);
            
            println!("Thread {} completed successfully", i);
            Ok(())
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap()?;
    }
    
    Ok(())
}

#[test]
fn test_corrupted_data_handling() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let data_path = temp_dir.path().join("blockchain_data");
    
    // Create blockchain with data
    {
        let mut blockchain = Blockchain::new_persistent(&data_path)?;
        blockchain.add_block("Test data for corruption test".to_string());
        blockchain.flush()?;
    }
    
    // Simulate corruption by writing invalid data to a database file
    if let Ok(entries) = fs::read_dir(&data_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "db") {
                    // Write some invalid data to corrupt the file
                    fs::write(&path, b"CORRUPTED_DATA")?;
                    break;
                }
            }
        }
    }
    
    // Try to open the corrupted blockchain
    let result = Blockchain::new_persistent(&data_path);
    
    // Should either handle gracefully or return an error
    match result {
        Ok(blockchain) => {
            // If it opens, it should at least have a genesis block
            assert!(blockchain.chain.len() >= 1);
            println!("Blockchain handled corruption gracefully");
        }
        Err(e) => {
            println!("Blockchain correctly detected corruption: {}", e);
            // This is also acceptable behavior
        }
    }
    
    Ok(())
}

#[test]
fn test_large_dataset_persistence() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let data_path = temp_dir.path().join("blockchain_data");
    
    let mut blockchain = Blockchain::new_persistent(&data_path)?;
    
    // Add a large number of blocks with substantial data
    for i in 0..100 {
        let large_data = format!(r#"
        @prefix : <http://example.org/> .
        @prefix tc: <http://provchain.org/trace#> .
        
        :batch{:03} tc:product "Large Dataset Product {}" ;
                   tc:origin "Large Dataset Farm {}" ;
                   tc:description "This is a large description for batch {} with lots of text to make the data substantial and test the storage capabilities of the persistent blockchain implementation." ;
                   tc:metadata "Additional metadata for batch {} including various properties and attributes that would be typical in a real-world supply chain scenario." ;
                   tc:timestamp "2024-01-{:02}T10:00:00Z" .
        "#, i, i, i, i, i, (i % 28) + 1);
        
        blockchain.add_block(large_data);
        
        // Flush every 10 blocks
        if i % 10 == 0 {
            blockchain.flush()?;
        }
    }
    
    blockchain.flush()?;
    
    // Verify all blocks are present and valid
    assert_eq!(blockchain.chain.len(), 101); // Genesis + 100 data blocks
    assert!(blockchain.is_valid());
    
    // Test querying the large dataset
    let query = r#"
    PREFIX tc: <http://provchain.org/trace#>
    SELECT (COUNT(?batch) as ?count) WHERE {
        ?batch tc:product ?product .
        FILTER(CONTAINS(?product, "Large Dataset Product"))
    }
    "#;
    
    let results = blockchain.rdf_store.query(query);
    if let oxigraph::sparql::QueryResults::Solutions(solutions) = results {
        let results: Vec<_> = solutions.collect();
        assert!(!results.is_empty(), "Should find large dataset products");
    }
    
    // Get final storage statistics
    let stats = blockchain.get_storage_stats()?;
    println!("Large dataset storage stats: {:?}", stats);
    
    // Verify substantial data was stored
    assert!(stats.disk_usage_bytes.unwrap_or(0) > 1000); // Should be substantial
    assert!(stats.quad_count > 100); // Should have many quads
    
    Ok(())
}
