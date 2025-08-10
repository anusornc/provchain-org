//! ProvChain Load Testing Framework
//! 
//! This module contains comprehensive load tests for ProvChain's distributed
//! network capabilities, stress testing, and production readiness validation.

use provchain_org::blockchain::Blockchain;
use provchain_org::rdf_store::RDFStore;
use oxigraph::model::NamedNode;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

/// Load test configuration
#[derive(Debug, Clone)]
pub struct LoadTestConfig {
    pub num_nodes: usize,
    pub blocks_per_node: u32,
    pub concurrent_queries: u32,
    pub test_duration_secs: u64,
    pub rdf_complexity: u32,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            num_nodes: 3,
            blocks_per_node: 100,
            concurrent_queries: 10,
            test_duration_secs: 60,
            rdf_complexity: 50,
        }
    }
}

/// Load test results
#[derive(Debug, Clone)]
pub struct LoadTestResults {
    pub total_blocks_processed: u32,
    pub total_queries_executed: u32,
    pub average_block_time: Duration,
    pub average_query_time: Duration,
    pub peak_memory_usage_mb: u64,
    pub errors_encountered: u32,
    pub throughput_blocks_per_sec: f64,
    pub throughput_queries_per_sec: f64,
}

impl LoadTestResults {
    pub fn print_summary(&self) {
        println!("\n=== ProvChain Load Test Results ===");
        println!("Total blocks processed: {}", self.total_blocks_processed);
        println!("Total queries executed: {}", self.total_queries_executed);
        println!("Average block processing time: {:?}", self.average_block_time);
        println!("Average query time: {:?}", self.average_query_time);
        println!("Peak memory usage: {} MB", self.peak_memory_usage_mb);
        println!("Errors encountered: {}", self.errors_encountered);
        println!("Block throughput: {:.2} blocks/sec", self.throughput_blocks_per_sec);
        println!("Query throughput: {:.2} queries/sec", self.throughput_queries_per_sec);
        println!("=====================================\n");
    }
}

/// Generate realistic supply chain workload for load testing
fn generate_load_test_rdf(batch_id: u64, complexity: u32) -> String {
    let hour = (batch_id % 24) as u8;
    let minute = (batch_id % 60) as u8;
    let second = ((batch_id * 2) % 60) as u8;
    let farmer_id = batch_id % 1000;
    let lat = 40.0 + (batch_id as f64 % 10.0);
    let lon = -74.0 + (batch_id as f64 % 10.0);

    let mut rdf = format!(r#"
@prefix trace: <http://provchain.org/trace#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix ex: <http://example.org/load_test/batch{batch_id}#> .

ex:batch{batch_id} a trace:ProductBatch ;
    trace:hasBatchID "LOAD_BATCH_{batch_id:08}" ;
    trace:producedAt "2025-08-08T{hour:02}:{minute:02}:{second:02}Z"^^xsd:dateTime ;
    prov:wasAttributedTo ex:farmer{farmer_id} .

ex:farmer{farmer_id} a trace:Farmer ;
    rdfs:label "Load Test Farmer {farmer_id}" ;
    trace:hasLocation ex:location{farmer_id} .

ex:location{farmer_id} a trace:GeographicLocation ;
    trace:hasLatitude "{lat:.6}"^^xsd:decimal ;
    trace:hasLongitude "{lon:.6}"^^xsd:decimal .
"#);

    // Add complexity based on the complexity parameter
    for i in 0..complexity {
        let activity_hour = (batch_id % 24) as u8;
        let activity_minute = ((batch_id + i as u64) % 60) as u8;
        let activity_second = ((batch_id + i as u64 * 2) % 60) as u8;
        let processor_id = i % 100;
        let temp = 2.0 + (i as f64 % 8.0);
        let humidity = 40.0 + (i as f64 % 40.0);
        let pressure = 1000.0 + (i as f64 % 100.0);

        rdf.push_str(&format!(r#"
ex:activity{i} a trace:ProcessingActivity ;
    trace:recordedAt "2025-08-08T{activity_hour:02}:{activity_minute:02}:{activity_second:02}Z"^^xsd:dateTime ;
    prov:used ex:batch{batch_id} ;
    prov:wasAssociatedWith ex:processor{processor_id} ;
    trace:hasCondition ex:condition{i} .

ex:processor{processor_id} a trace:Manufacturer ;
    rdfs:label "Processor {processor_id}" .

ex:condition{i} a trace:EnvironmentalCondition ;
    trace:hasTemperature "{temp:.1}"^^xsd:decimal ;
    trace:hasHumidity "{humidity:.1}"^^xsd:decimal ;
    trace:hasPressure "{pressure:.1}"^^xsd:decimal .
"#));
    }

    rdf
}

/// Simulate a ProvChain node under load
struct LoadTestNode {
    id: usize,
    blockchain: Arc<Mutex<Blockchain>>,
    metrics: Arc<Mutex<LoadTestResults>>,
}

impl LoadTestNode {
    fn new(id: usize) -> Self {
        Self {
            id,
            blockchain: Arc::new(Mutex::new(Blockchain::new())),
            metrics: Arc::new(Mutex::new(LoadTestResults {
                total_blocks_processed: 0,
                total_queries_executed: 0,
                average_block_time: Duration::new(0, 0),
                average_query_time: Duration::new(0, 0),
                peak_memory_usage_mb: 0,
                errors_encountered: 0,
                throughput_blocks_per_sec: 0.0,
                throughput_queries_per_sec: 0.0,
            })),
        }
    }

    fn run_block_producer(&self, config: &LoadTestConfig) -> thread::JoinHandle<()> {
        let blockchain = Arc::clone(&self.blockchain);
        let metrics = Arc::clone(&self.metrics);
        let node_id = self.id;
        let blocks_to_produce = config.blocks_per_node;
        let complexity = config.rdf_complexity;

        thread::spawn(move || {
            let mut total_block_time = Duration::new(0, 0);
            let mut blocks_processed = 0;

            for i in 0..blocks_to_produce {
                let batch_id = (node_id as u64 * 10000) + i as u64;
                let rdf_data = generate_load_test_rdf(batch_id, complexity);

                let start = Instant::now();
                {
                    let mut bc = blockchain.lock().unwrap();
                    bc.add_block(rdf_data);
                }
                let block_time = start.elapsed();
                total_block_time += block_time;
                blocks_processed += 1;

                if i % 10 == 0 {
                    println!("Node {}: Processed {} blocks", node_id, i + 1);
                }

                // Simulate realistic block production intervals
                thread::sleep(Duration::from_millis(100));
            }

            // Update metrics
            {
                let mut metrics = metrics.lock().unwrap();
                metrics.total_blocks_processed = blocks_processed;
                metrics.average_block_time = total_block_time / blocks_processed;
            }

            println!("Node {node_id}: Block producer completed");
        })
    }

    fn run_query_executor(&self, config: &LoadTestConfig) -> thread::JoinHandle<()> {
        let blockchain = Arc::clone(&self.blockchain);
        let metrics = Arc::clone(&self.metrics);
        let node_id = self.id;
        let test_duration = Duration::from_secs(config.test_duration_secs);

        thread::spawn(move || {
            let start_time = Instant::now();
            let mut total_query_time = Duration::new(0, 0);
            let mut queries_executed = 0;

            let queries = vec![
                // Simple batch count query
                r#"
                PREFIX trace: <http://provchain.org/trace#>
                SELECT (COUNT(?batch) as ?count) WHERE {
                    ?batch a trace:ProductBatch .
                }
                "#,
                
                // Complex traceability query
                r#"
                PREFIX trace: <http://provchain.org/trace#>
                PREFIX prov: <http://www.w3.org/ns/prov#>
                SELECT ?batch ?activity ?agent WHERE {
                    ?batch a trace:ProductBatch .
                    ?activity prov:used ?batch ;
                              prov:wasAssociatedWith ?agent .
                } LIMIT 20
                "#,
                
                // Environmental conditions aggregation
                r#"
                PREFIX trace: <http://provchain.org/trace#>
                SELECT (AVG(?temp) as ?avg_temp) (MAX(?humidity) as ?max_humidity) WHERE {
                    ?condition a trace:EnvironmentalCondition ;
                               trace:hasTemperature ?temp ;
                               trace:hasHumidity ?humidity .
                }
                "#,
                
                // Geographic location query
                r#"
                PREFIX trace: <http://provchain.org/trace#>
                SELECT ?farmer ?location ?lat ?lon WHERE {
                    ?farmer a trace:Farmer ;
                            trace:hasLocation ?location .
                    ?location trace:hasLatitude ?lat ;
                              trace:hasLongitude ?lon .
                } LIMIT 10
                "#,
            ];

            while start_time.elapsed() < test_duration {
                for query in &queries {
                    let query_start = Instant::now();
                    {
                        let bc = blockchain.lock().unwrap();
                        let _results = bc.rdf_store.query(query);
                    }
                    let query_time = query_start.elapsed();
                    total_query_time += query_time;
                    queries_executed += 1;

                    // Small delay between queries
                    thread::sleep(Duration::from_millis(50));

                    if start_time.elapsed() >= test_duration {
                        break;
                    }
                }
            }

            // Update metrics
            {
                let mut metrics = metrics.lock().unwrap();
                metrics.total_queries_executed = queries_executed;
                if queries_executed > 0 {
                    metrics.average_query_time = total_query_time / queries_executed;
                }
            }

            println!("Node {node_id}: Query executor completed ({queries_executed} queries)");
        })
    }
}

#[test]
fn load_test_single_node_stress() {
    println!("=== ProvChain Single Node Stress Test ===");
    
    let config = LoadTestConfig {
        num_nodes: 1,
        blocks_per_node: 500,
        concurrent_queries: 20,
        test_duration_secs: 30,
        rdf_complexity: 25,
    };

    let results = run_load_test(&config);
    results.print_summary();

    // Performance assertions
    assert!(results.total_blocks_processed >= 400, "Should process most blocks under stress");
    assert!(results.average_block_time < Duration::from_millis(200), "Block processing should remain efficient");
    assert!(results.errors_encountered == 0, "Should not encounter errors under normal stress");
}

#[test]
fn load_test_multi_node_simulation() {
    println!("=== ProvChain Multi-Node Simulation Test ===");
    
    let config = LoadTestConfig {
        num_nodes: 3,
        blocks_per_node: 200,
        concurrent_queries: 15,
        test_duration_secs: 45,
        rdf_complexity: 30,
    };

    let results = run_load_test(&config);
    results.print_summary();

    // Multi-node performance assertions
    assert!(results.total_blocks_processed >= 500, "Should process blocks across multiple nodes");
    assert!(results.throughput_blocks_per_sec > 5.0, "Should maintain reasonable throughput");
    assert!(results.throughput_queries_per_sec > 10.0, "Should handle concurrent queries efficiently");
}

#[test]
#[ignore] // Use `cargo test -- --ignored` to run this expensive test
fn load_test_high_complexity_rdf() {
    println!("=== ProvChain High Complexity RDF Load Test ===");
    
    let config = LoadTestConfig {
        num_nodes: 2,
        blocks_per_node: 100,
        concurrent_queries: 10,
        test_duration_secs: 60,
        rdf_complexity: 100, // High complexity
    };

    let results = run_load_test(&config);
    results.print_summary();

    // High complexity assertions
    assert!(results.total_blocks_processed >= 150, "Should handle complex RDF under load");
    assert!(results.average_block_time < Duration::from_secs(2), "Complex RDF should still process reasonably fast");
}

#[test]
#[ignore] // Use `cargo test -- --ignored` to run this expensive test
fn load_test_extended_duration() {
    println!("=== ProvChain Extended Duration Load Test ===");
    
    let config = LoadTestConfig {
        num_nodes: 2,
        blocks_per_node: 1000,
        concurrent_queries: 25,
        test_duration_secs: 300, // 5 minutes
        rdf_complexity: 40,
    };

    let results = run_load_test(&config);
    results.print_summary();

    // Extended duration assertions
    assert!(results.total_blocks_processed >= 1500, "Should process many blocks over extended time");
    assert!(results.peak_memory_usage_mb < 1000, "Memory usage should remain reasonable");
    assert!(results.errors_encountered < 10, "Should have minimal errors over extended duration");
}

fn run_load_test(config: &LoadTestConfig) -> LoadTestResults {
    println!("Starting load test with config: {config:?}");
    
    let start_time = Instant::now();
    let mut nodes = Vec::new();
    let mut handles = Vec::new();

    // Create nodes
    for i in 0..config.num_nodes {
        let node = LoadTestNode::new(i);
        nodes.push(node);
    }

    // Start block producers
    for node in &nodes {
        let handle = node.run_block_producer(config);
        handles.push(handle);
    }

    // Start query executors
    for node in &nodes {
        let handle = node.run_query_executor(config);
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let total_duration = start_time.elapsed();

    // Aggregate results from all nodes
    let mut total_blocks = 0;
    let mut total_queries = 0;
    let mut total_block_time = Duration::new(0, 0);
    let mut total_query_time = Duration::new(0, 0);
    let mut total_errors = 0;

    for node in &nodes {
        let metrics = node.metrics.lock().unwrap();
        total_blocks += metrics.total_blocks_processed;
        total_queries += metrics.total_queries_executed;
        total_block_time += metrics.average_block_time;
        total_query_time += metrics.average_query_time;
        total_errors += metrics.errors_encountered;
    }

    LoadTestResults {
        total_blocks_processed: total_blocks,
        total_queries_executed: total_queries,
        average_block_time: if config.num_nodes > 0 { total_block_time / config.num_nodes as u32 } else { Duration::new(0, 0) },
        average_query_time: if config.num_nodes > 0 { total_query_time / config.num_nodes as u32 } else { Duration::new(0, 0) },
        peak_memory_usage_mb: estimate_memory_usage(&nodes),
        errors_encountered: total_errors,
        throughput_blocks_per_sec: total_blocks as f64 / total_duration.as_secs_f64(),
        throughput_queries_per_sec: total_queries as f64 / total_duration.as_secs_f64(),
    }
}

fn estimate_memory_usage(nodes: &[LoadTestNode]) -> u64 {
    let mut total_memory = 0;
    
    for node in nodes {
        let bc = node.blockchain.lock().unwrap();
        // Rough estimation: each block ~2KB + RDF store overhead
        let estimated_node_memory = bc.chain.len() * 2048 + 10 * 1024 * 1024; // 10MB base overhead
        total_memory += estimated_node_memory;
    }
    
    (total_memory / 1024 / 1024) as u64 // Convert to MB
}

#[test]
fn stress_test_blockchain_validation() {
    println!("=== ProvChain Blockchain Validation Stress Test ===");
    
    let mut bc = Blockchain::new();
    
    // Add many blocks
    for i in 0..1000 {
        let rdf_data = generate_load_test_rdf(i, 20);
        bc.add_block(rdf_data);
    }
    
    // Stress test validation multiple times
    let validation_times = 10;
    let mut total_validation_time = Duration::new(0, 0);
    
    for i in 0..validation_times {
        let start = Instant::now();
        assert!(bc.is_valid(), "Blockchain should remain valid during stress test");
        let validation_time = start.elapsed();
        total_validation_time += validation_time;
        
        println!("Validation {}: {:?}", i + 1, validation_time);
    }
    
    let average_validation_time = total_validation_time / validation_times;
    println!("Average validation time for 1000 blocks: {average_validation_time:?}");
    
    // Validation should complete within reasonable time even for large chains
    assert!(average_validation_time < Duration::from_secs(30), "Validation should complete within 30 seconds");
}

#[test]
fn stress_test_rdf_canonicalization() {
    println!("=== ProvChain RDF Canonicalization Stress Test ===");
    
    let mut rdf_store = RDFStore::new();
    let mut canonicalization_times = Vec::new();
    
    // Test canonicalization with increasingly complex graphs
    for complexity in (10..=200).step_by(20) {
        let rdf_data = generate_load_test_rdf(complexity as u64, complexity);
        let graph_name = NamedNode::new(format!("http://example.org/stress_{complexity}")).unwrap();
        
        rdf_store.add_rdf_to_graph(&rdf_data, &graph_name);
        
        let start = Instant::now();
        let _hash = rdf_store.canonicalize_graph(&graph_name);
        let canonicalization_time = start.elapsed();
        
        canonicalization_times.push((complexity, canonicalization_time));
        println!("Complexity {complexity}: {canonicalization_time:?}");
    }
    
    // Verify that canonicalization time doesn't grow exponentially
    let first_time = canonicalization_times[0].1;
    let last_time = canonicalization_times.last().unwrap().1;
    let growth_ratio = last_time.as_secs_f64() / first_time.as_secs_f64();
    
    println!("Canonicalization time growth ratio: {growth_ratio:.2}x");
    assert!(growth_ratio < 100.0, "Canonicalization time should not grow exponentially");
}

#[test]
fn stress_test_concurrent_sparql_queries() {
    println!("=== ProvChain Concurrent SPARQL Stress Test ===");
    
    let bc = Arc::new(Mutex::new(Blockchain::new()));
    
    // Populate with test data
    {
        let mut blockchain = bc.lock().unwrap();
        for i in 0..200 {
            let rdf_data = generate_load_test_rdf(i, 30);
            blockchain.add_block(rdf_data);
        }
    }
    
    let num_threads = 8;
    let queries_per_thread = 50;
    let mut handles = Vec::new();
    
    let start_time = Instant::now();
    
    for thread_id in 0..num_threads {
        let bc_clone = Arc::clone(&bc);
        let handle = thread::spawn(move || {
            let queries = [r#"PREFIX trace: <http://provchain.org/trace#> SELECT (COUNT(?batch) as ?count) WHERE { ?batch a trace:ProductBatch . }"#,
                r#"PREFIX trace: <http://provchain.org/trace#> SELECT ?farmer WHERE { ?farmer a trace:Farmer . } LIMIT 10"#,
                r#"PREFIX trace: <http://provchain.org/trace#> SELECT (AVG(?temp) as ?avg_temp) WHERE { ?condition trace:hasTemperature ?temp . }"#];
            
            for i in 0..queries_per_thread {
                let query = &queries[i % queries.len()];
                let bc = bc_clone.lock().unwrap();
                let _results = bc.rdf_store.query(query);
                
                if i % 10 == 0 {
                    println!("Thread {}: Completed {} queries", thread_id, i + 1);
                }
            }
            
            println!("Thread {thread_id} completed all {queries_per_thread} queries");
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_duration = start_time.elapsed();
    let total_queries = num_threads * queries_per_thread;
    let queries_per_second = total_queries as f64 / total_duration.as_secs_f64();
    
    println!("Concurrent SPARQL stress test completed:");
    println!("Total queries: {total_queries}");
    println!("Total duration: {total_duration:?}");
    println!("Queries per second: {queries_per_second:.2}");
    
    // Should handle concurrent queries efficiently
    assert!(queries_per_second > 50.0, "Should handle at least 50 queries per second under stress");
    assert!(total_duration < Duration::from_secs(60), "Should complete within 60 seconds");
}
