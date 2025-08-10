use provchain_org::blockchain::Blockchain;
use std::fs;

#[test]
fn test_blockchain_with_simple_supply_chain_data() {
    let mut blockchain = Blockchain::new();
    
    // Read and add simple supply chain test data to blockchain
    let turtle_data = fs::read_to_string("test_data/simple_supply_chain_test.ttl")
        .expect("Failed to read simple_supply_chain_test.ttl");
    
    blockchain.add_block(turtle_data);
    
    // Verify blockchain is valid
    assert!(blockchain.is_valid(), "Blockchain should be valid after adding simple supply chain data");
    assert_eq!(blockchain.chain.len(), 2, "Should have genesis block + 1 data block");
    
    // Query the RDF data in the blockchain
    let query = r#"PREFIX trace: <http://example.org/trace#>
        SELECT ?batch ?batchId
        FROM <http://provchain.org/block/1>
        WHERE {
            ?batch a trace:ProductBatch ;
                   trace:hasBatchID ?batchId .
        }
    "#;
    
    if let oxigraph::sparql::QueryResults::Solutions(solutions) = blockchain.rdf_store.query(query) {
        let results: Vec<_> = solutions.collect();
        assert!(!results.is_empty(), "Should find product batches in the blockchain");
        
        // Check if we can find the specific milk batch
        let mut found_milk_batch = false;
        for result in results {
            if let Ok(solution) = result {
                if let Some(batch_id) = solution.get("batchId") {
                    if batch_id.to_string().contains("MB-2025-001") {
                        found_milk_batch = true;
                        break;
                    }
                }
            }
        }
        assert!(found_milk_batch, "Should find the milk batch MB-2025-001");
    } else {
        panic!("SPARQL query failed");
    }
}
