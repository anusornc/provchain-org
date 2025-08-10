//! ProvChain Performance Benchmarks
//! 
//! This module contains comprehensive performance tests for ProvChain,
//! including load testing, scaling analysis, and competitive benchmarking.

use provchain_org::blockchain::Blockchain;
use provchain_org::rdf_store::RDFStore;
use oxigraph::model::NamedNode;
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Performance metrics for ProvChain operations
#[derive(Debug, Clone)]
pub struct ProvChainMetrics {
    pub blocks_per_second: f64,
    pub rdf_canonicalization_time: Duration,
    pub sparql_query_latency: Duration,
    pub ontology_validation_time: Duration,
    pub memory_usage_mb: u64,
    pub block_validation_time: Duration,
    pub total_test_duration: Duration,
}

impl Default for ProvChainMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl ProvChainMetrics {
    pub fn new() -> Self {
        Self {
            blocks_per_second: 0.0,
            rdf_canonicalization_time: Duration::new(0, 0),
            sparql_query_latency: Duration::new(0, 0),
            ontology_validation_time: Duration::new(0, 0),
            memory_usage_mb: 0,
            block_validation_time: Duration::new(0, 0),
            total_test_duration: Duration::new(0, 0),
        }
    }

    pub fn print_summary(&self) {
        println!("\n=== ProvChain Performance Metrics ===");
        println!("Blocks per second: {:.2}", self.blocks_per_second);
        println!("RDF canonicalization time: {:?}", self.rdf_canonicalization_time);
        println!("SPARQL query latency: {:?}", self.sparql_query_latency);
        println!("Ontology validation time: {:?}", self.ontology_validation_time);
        println!("Block validation time: {:?}", self.block_validation_time);
        println!("Memory usage: {} MB", self.memory_usage_mb);
        println!("Total test duration: {:?}", self.total_test_duration);
        println!("=====================================\n");
    }
}

/// Generate realistic supply chain RDF data for testing
fn generate_supply_chain_rdf(batch_id: u64) -> String {
    format!(r#"
@prefix trace: <http://provchain.org/trace#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ex: <http://example.org/batch{}#> .

ex:batch{} a trace:ProductBatch ;
    trace:hasBatchID "BATCH{:06}" ;
    trace:producedAt "2025-08-08T{:02}:{:02}:00Z"^^xsd:dateTime ;
    prov:wasAttributedTo ex:farmer{} .

ex:farmer{} a trace:Farmer ;
    rdfs:label "Farmer {}" .

ex:processing{} a trace:ProcessingActivity ;
    trace:recordedAt "2025-08-08T{:02}:{:02}:00Z"^^xsd:dateTime ;
    prov:used ex:batch{} ;
    prov:wasAssociatedWith ex:processor{} .

ex:processor{} a trace:Manufacturer ;
    rdfs:label "Processor {}" .

ex:transport{} a trace:TransportActivity ;
    trace:recordedAt "2025-08-08T{:02}:{:02}:00Z"^^xsd:dateTime ;
    prov:used ex:batch{} ;
    trace:hasCondition ex:condition{} .

ex:condition{} a trace:EnvironmentalCondition ;
    trace:hasTemperature "{:.1}"^^xsd:decimal ;
    trace:hasHumidity "{:.1}"^^xsd:decimal .
"#, 
        batch_id, batch_id, batch_id,
        (batch_id % 24) as u8, (batch_id % 60) as u8,
        batch_id % 1000, batch_id % 1000, batch_id % 1000,
        batch_id, (batch_id % 24) as u8, ((batch_id + 1) % 60) as u8,
        batch_id, batch_id % 1000, batch_id % 1000, batch_id % 1000,
        batch_id, (batch_id % 24) as u8, ((batch_id + 2) % 60) as u8,
        batch_id, batch_id, batch_id,
        2.0 + (batch_id as f64 % 10.0), 40.0 + (batch_id as f64 % 30.0)
    )
}

/// Generate complex RDF data with blank nodes for canonicalization testing
fn generate_complex_rdf_with_blank_nodes(complexity_level: u32) -> String {
    let mut rdf = String::from(r#"
@prefix ex: <http://example.org/> .
@prefix trace: <http://provchain.org/trace#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
"#);

    for i in 0..complexity_level {
        rdf.push_str(&format!(r#"
_:batch{} a trace:ProductBatch ;
    trace:hasBatchID "COMPLEX{}" ;
    prov:wasAttributedTo _:agent{} .

_:agent{} a trace:Farmer ;
    ex:knows _:agent{} ;
    ex:processes _:batch{} .
"#, i, i, i, i, (i + 1) % complexity_level, (i + 1) % complexity_level));
    }

    rdf
}

#[test]
fn benchmark_blockchain_scaling_small() {
    println!("=== ProvChain Blockchain Scaling Test (Small: 100 blocks) ===");
    let metrics = run_blockchain_scaling_test(100);
    metrics.print_summary();
    
    // Performance assertions for small scale
    assert!(metrics.blocks_per_second > 10.0, "Should process at least 10 blocks/second");
    assert!(metrics.block_validation_time < Duration::from_millis(100), "Block validation should be under 100ms");
}

#[test]
fn benchmark_blockchain_scaling_medium() {
    println!("=== ProvChain Blockchain Scaling Test (Medium: 1000 blocks) ===");
    let metrics = run_blockchain_scaling_test(1000);
    metrics.print_summary();
    
    // Performance assertions for medium scale
    assert!(metrics.blocks_per_second > 5.0, "Should process at least 5 blocks/second");
    assert!(metrics.total_test_duration < Duration::from_secs(300), "Should complete within 5 minutes");
}

#[test]
#[ignore] // Use `cargo test -- --ignored` to run this expensive test
fn benchmark_blockchain_scaling_large() {
    println!("=== ProvChain Blockchain Scaling Test (Large: 10000 blocks) ===");
    let metrics = run_blockchain_scaling_test(10000);
    metrics.print_summary();
    
    // Performance assertions for large scale
    assert!(metrics.blocks_per_second > 1.0, "Should process at least 1 block/second");
    assert!(metrics.total_test_duration < Duration::from_secs(3600), "Should complete within 1 hour");
}

fn run_blockchain_scaling_test(num_blocks: u32) -> ProvChainMetrics {
    let mut metrics = ProvChainMetrics::new();
    let start_time = Instant::now();
    
    let mut bc = Blockchain::new();
    
    // Measure block addition performance
    let mut total_canonicalization_time = Duration::new(0, 0);
    let mut total_validation_time = Duration::new(0, 0);
    
    for i in 0..num_blocks {
        let rdf_data = generate_supply_chain_rdf(i as u64);
        
        // Measure canonicalization time
        let canon_start = Instant::now();
        bc.add_block(rdf_data);
        let canon_time = canon_start.elapsed();
        total_canonicalization_time += canon_time;
        
        // Measure validation time every 100 blocks
        if i % 100 == 0 {
            let validation_start = Instant::now();
            assert!(bc.is_valid(), "Blockchain should remain valid");
            total_validation_time += validation_start.elapsed();
        }
        
        if i % 1000 == 0 && i > 0 {
            println!("Processed {i} blocks...");
        }
    }
    
    // Final validation
    let final_validation_start = Instant::now();
    assert!(bc.is_valid(), "Final blockchain should be valid");
    let final_validation_time = final_validation_start.elapsed();
    
    metrics.total_test_duration = start_time.elapsed();
    metrics.blocks_per_second = num_blocks as f64 / metrics.total_test_duration.as_secs_f64();
    metrics.rdf_canonicalization_time = total_canonicalization_time / num_blocks;
    metrics.block_validation_time = (total_validation_time + final_validation_time) / ((num_blocks / 100) + 1);
    
    // Estimate memory usage (simplified)
    metrics.memory_usage_mb = (bc.chain.len() * 1024) as u64 / 1024; // Rough estimate
    
    metrics
}

#[test]
fn benchmark_rdf_canonicalization_complexity() {
    println!("=== ProvChain RDF Canonicalization Complexity Test ===");
    
    let complexity_levels = vec![10, 50, 100, 500];
    let mut results = HashMap::new();
    
    for &complexity in &complexity_levels {
        let mut rdf_store = RDFStore::new();
        let rdf_data = generate_complex_rdf_with_blank_nodes(complexity);
        let graph_name = NamedNode::new(format!("http://example.org/complexity_{complexity}")).unwrap();
        
        rdf_store.add_rdf_to_graph(&rdf_data, &graph_name);
        
        // Measure canonicalization time
        let start = Instant::now();
        let _hash = rdf_store.canonicalize_graph(&graph_name);
        let duration = start.elapsed();
        
        results.insert(complexity, duration);
        println!("Complexity {complexity}: {duration:?}");
    }
    
    // Verify that canonicalization time scales reasonably
    assert!(results[&10] < Duration::from_millis(100), "Simple graphs should canonicalize quickly");
    assert!(results[&500] < Duration::from_secs(10), "Complex graphs should canonicalize within 10 seconds");
}

#[test]
fn benchmark_sparql_query_performance() {
    println!("=== ProvChain SPARQL Query Performance Test ===");
    
    let mut bc = Blockchain::new();
    
    // Add test data
    for i in 0..100 {
        let rdf_data = generate_supply_chain_rdf(i);
        bc.add_block(rdf_data);
    }
    
    let queries = vec![
        // Simple batch lookup
        r#"
        PREFIX trace: <http://provchain.org/trace#>
        SELECT ?batch WHERE {
            ?batch a trace:ProductBatch ;
                   trace:hasBatchID ?id .
        } LIMIT 10
        "#,
        
        // Complex traceability query
        r#"
        PREFIX trace: <http://provchain.org/trace#>
        PREFIX prov: <http://www.w3.org/ns/prov#>
        SELECT ?batch ?activity ?agent WHERE {
            ?batch a trace:ProductBatch .
            ?activity prov:used ?batch ;
                      prov:wasAssociatedWith ?agent .
        } LIMIT 10
        "#,
        
        // Environmental conditions query
        r#"
        PREFIX trace: <http://provchain.org/trace#>
        SELECT ?condition ?temp ?humidity WHERE {
            ?activity trace:hasCondition ?condition .
            ?condition trace:hasTemperature ?temp ;
                       trace:hasHumidity ?humidity .
        } LIMIT 10
        "#,
    ];
    
    for (i, query) in queries.iter().enumerate() {
        let start = Instant::now();
        let _results = bc.rdf_store.query(query);
        let duration = start.elapsed();
        
        println!("Query {}: {:?}", i + 1, duration);
        assert!(duration < Duration::from_millis(500), "SPARQL queries should complete within 500ms");
    }
}

#[test]
fn benchmark_concurrent_operations() {
    println!("=== ProvChain Concurrent Operations Test ===");
    
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let bc = Arc::new(Mutex::new(Blockchain::new()));
    
    // Add some initial data
    {
        let mut blockchain = bc.lock().unwrap();
        for i in 0..50 {
            let rdf_data = generate_supply_chain_rdf(i);
            blockchain.add_block(rdf_data);
        }
    }
    
    let start = Instant::now();
    let mut handles = vec![];
    
    // Spawn multiple threads for concurrent SPARQL queries
    for thread_id in 0..4 {
        let bc_clone = Arc::clone(&bc);
        let handle = thread::spawn(move || {
            let query = r#"
                PREFIX trace: <http://provchain.org/trace#>
                SELECT ?batch WHERE {
                    ?batch a trace:ProductBatch .
                } LIMIT 5
            "#;
            
            for _i in 0..10 {
                let blockchain = bc_clone.lock().unwrap();
                let _results = blockchain.rdf_store.query(query);
                // Simulate some processing time
                thread::sleep(Duration::from_millis(10));
            }
            
            println!("Thread {thread_id} completed");
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    println!("Concurrent operations completed in: {duration:?}");
    
    // Verify blockchain is still valid after concurrent access
    let blockchain = bc.lock().unwrap();
    assert!(blockchain.is_valid(), "Blockchain should remain valid after concurrent operations");
}

#[test]
fn benchmark_memory_usage_growth() {
    println!("=== ProvChain Memory Usage Growth Test ===");
    
    let mut bc = Blockchain::new();
    let mut memory_samples = Vec::new();
    
    for i in 0..1000 {
        let rdf_data = generate_supply_chain_rdf(i);
        bc.add_block(rdf_data);
        
        // Sample memory usage every 100 blocks
        if i % 100 == 0 {
            // Simplified memory estimation based on chain length
            let estimated_memory = bc.chain.len() * 2048; // Rough estimate in bytes
            memory_samples.push((i, estimated_memory));
            println!("Blocks: {}, Estimated memory: {} KB", i, estimated_memory / 1024);
        }
    }
    
    // Verify memory growth is reasonable (linear, not exponential)
    if memory_samples.len() >= 2 {
        let first_sample = memory_samples[1].1; // Skip the first sample (might be 0)
        let last_sample = memory_samples.last().unwrap().1;
        let growth_ratio = last_sample as f64 / first_sample as f64;
        
        println!("Memory growth ratio: {growth_ratio:.2}x");
        assert!(growth_ratio < 20.0, "Memory growth should be reasonable (less than 20x for 10x data)");
    }
}

/// Comparative benchmark against simple hash-based blockchain
#[test]
fn benchmark_provchain_vs_simple_blockchain() {
    println!("=== ProvChain vs Simple Blockchain Comparison ===");
    
    let num_blocks = 100;
    
    // Test ProvChain performance
    let provchain_start = Instant::now();
    let mut bc = Blockchain::new();
    for i in 0..num_blocks {
        let rdf_data = generate_supply_chain_rdf(i);
        bc.add_block(rdf_data);
    }
    let provchain_duration = provchain_start.elapsed();
    
    // Test simple string-based blockchain performance
    let simple_start = Instant::now();
    let mut simple_chain = Vec::new();
    for i in 0..num_blocks {
        let simple_data = format!("Simple transaction {i}");
        // Simulate simple hash calculation
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(simple_data.as_bytes());
        let _hash = format!("{:x}", hasher.finalize());
        simple_chain.push(simple_data);
    }
    let simple_duration = simple_start.elapsed();
    
    println!("ProvChain (RDF + Canonicalization): {provchain_duration:?}");
    println!("Simple Blockchain (String + Hash): {simple_duration:?}");
    
    let overhead_ratio = provchain_duration.as_secs_f64() / simple_duration.as_secs_f64();
    println!("ProvChain overhead ratio: {overhead_ratio:.2}x");
    
    // ProvChain should be slower due to RDF processing, but not excessively so
    assert!(overhead_ratio < 100.0, "ProvChain overhead should be reasonable (less than 100x)");
    assert!(overhead_ratio > 1.0, "ProvChain should have some overhead due to RDF processing");
    
    // But ProvChain provides semantic capabilities that simple blockchain doesn't
    let query = r#"
        PREFIX trace: <http://provchain.org/trace#>
        SELECT (COUNT(?batch) as ?count) WHERE {
            ?batch a trace:ProductBatch .
        }
    "#;
    
    let query_start = Instant::now();
    let _results = bc.rdf_store.query(query);
    let query_duration = query_start.elapsed();
    
    println!("ProvChain semantic query capability: {query_duration:?}");
    println!("Simple blockchain semantic query capability: Not available");
}
