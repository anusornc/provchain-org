use chrono::Utc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use crate::rdf_store::{RDFStore, StorageConfig};
use crate::trace_optimization::{EnhancedTraceabilitySystem, EnhancedTraceResult};
use crate::atomic_operations::AtomicOperationContext;
use oxigraph::model::NamedNode;
use std::path::Path;
use anyhow::{Result, bail};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: String, // RDF in Turtle format
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        self.calculate_hash_with_store(None)
    }

    pub fn calculate_hash_with_store(&self, rdf_store: Option<&RDFStore>) -> String {
        let rdf_hash = if let Some(store) = rdf_store {
            // Use RDF canonicalization for the data
            let graph_name = NamedNode::new(format!("http://provchain.org/block/{}", self.index)).unwrap();
            store.canonicalize_graph(&graph_name)
        } else {
            // Fallback to simple hash if no store provided (for genesis block)
            let mut hasher = Sha256::new();
            hasher.update(self.data.as_bytes());
            format!("{:x}", hasher.finalize())
        };

        // Combine block metadata with canonicalized RDF hash
        let record = format!(
            "{0}{1}{2}{3}",
            self.index,
            self.timestamp,
            rdf_hash,
            self.previous_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub rdf_store: RDFStore,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

impl Blockchain {
    /// Create a new in-memory blockchain (for testing and development)
    pub fn new() -> Self {
        let mut bc = Blockchain {
            chain: Vec::new(),
            rdf_store: RDFStore::new(),
        };
        
        // Load the traceability ontology
        bc.load_ontology();
        
        let genesis_block = bc.create_genesis_block();
        
        // Add genesis block data to RDF store
        let graph_name = NamedNode::new("http://provchain.org/block/0").unwrap();
        bc.rdf_store.add_rdf_to_graph(&genesis_block.data, &graph_name);
        bc.rdf_store.add_block_metadata(&genesis_block);
        
        bc.chain.push(genesis_block);
        bc
    }

    /// Create a new persistent blockchain with RocksDB backend
    pub fn new_persistent<P: AsRef<Path>>(data_dir: P) -> Result<Self> {
        let rdf_store = RDFStore::new_persistent(data_dir)?;
        
        let mut bc = Blockchain {
            chain: Vec::new(),
            rdf_store,
        };
        
        // Load the traceability ontology
        bc.load_ontology();
        
        // Check if we need to create genesis block or load existing chain
        let store_len = bc.rdf_store.store.len().unwrap_or(0);
        if store_len == 0 {
            // Create genesis block for new blockchain
            let genesis_block = bc.create_genesis_block();
            
            // Add genesis block data to RDF store
            let graph_name = NamedNode::new("http://provchain.org/block/0").unwrap();
            bc.rdf_store.add_rdf_to_graph(&genesis_block.data, &graph_name);
            bc.rdf_store.add_block_metadata(&genesis_block);
            
            bc.chain.push(genesis_block);
            
            // Save to disk
            if let Err(e) = bc.rdf_store.save_to_disk() {
                eprintln!("Warning: Could not save to disk: {}", e);
            }
        } else {
            // Load existing blockchain from persistent storage
            bc.load_chain_from_store()?;
            
            // If no blocks were loaded but store has data, something is wrong
            // Create genesis block as fallback
            if bc.chain.is_empty() {
                eprintln!("Warning: Store has data but no blocks loaded, creating genesis block");
                let genesis_block = bc.create_genesis_block();
                
                let graph_name = NamedNode::new("http://provchain.org/block/0").unwrap();
                bc.rdf_store.add_rdf_to_graph(&genesis_block.data, &graph_name);
                bc.rdf_store.add_block_metadata(&genesis_block);
                
                bc.chain.push(genesis_block);
            }
        }
        
        Ok(bc)
    }

    /// Create a persistent blockchain with custom storage configuration
    pub fn new_persistent_with_config(config: StorageConfig) -> Result<Self> {
        let rdf_store = RDFStore::new_persistent_with_config(config)?;
        
        let mut bc = Blockchain {
            chain: Vec::new(),
            rdf_store,
        };
        
        // Load the traceability ontology
        bc.load_ontology();
        
        // Check if we need to create genesis block or load existing chain
        if bc.rdf_store.store.len().unwrap_or(0) == 0 {
            // Create genesis block for new blockchain
            let genesis_block = bc.create_genesis_block();
            
            // Add genesis block data to RDF store
            let graph_name = NamedNode::new("http://provchain.org/block/0").unwrap();
            bc.rdf_store.add_rdf_to_graph(&genesis_block.data, &graph_name);
            bc.rdf_store.add_block_metadata(&genesis_block);
            
            bc.chain.push(genesis_block);
        } else {
            // Load existing blockchain from persistent storage
            bc.load_chain_from_store()?;
        }
        
        Ok(bc)
    }

    /// Load blockchain from persistent RDF store
    fn load_chain_from_store(&mut self) -> Result<()> {
        use oxigraph::sparql::QueryResults;
        
        // Query to get all blocks ordered by index
        let query = r#"
            PREFIX prov: <http://provchain.org/>
            SELECT ?block ?index ?timestamp ?hash ?prevHash ?dataGraph WHERE {
                GRAPH <http://provchain.org/blockchain> {
                    ?block a prov:Block ;
                           prov:hasIndex ?index ;
                           prov:hasTimestamp ?timestamp ;
                           prov:hasHash ?hash ;
                           prov:hasPreviousHash ?prevHash ;
                           prov:hasDataGraphIRI ?dataGraph .
                }
            }
            ORDER BY ?index
        "#;
        
        if let QueryResults::Solutions(solutions) = self.rdf_store.query(query) {
            for solution in solutions {
                if let Ok(sol) = solution {
                    if let (Some(index_term), Some(timestamp_term), Some(hash_term), Some(prev_hash_term), Some(data_graph_term)) = (
                        sol.get("index"),
                        sol.get("timestamp"), 
                        sol.get("hash"),
                        sol.get("prevHash"),
                        sol.get("dataGraph")
                    ) {
                        // Parse block data
                        let index: u64 = index_term.to_string().parse().unwrap_or(0);
                        let timestamp = timestamp_term.to_string().trim_matches('"').to_string();
                        let hash = hash_term.to_string().trim_matches('"').to_string();
                        let previous_hash = prev_hash_term.to_string().trim_matches('"').to_string();
                        
                        // Extract RDF data from the block's graph
                        let data_graph_string = data_graph_term.to_string();
                        println!("Raw data graph string: '{}'", data_graph_string);
                        // Handle typed literals - extract the URI part before the type annotation
                        let data_graph_uri = if let Some(uri_part) = data_graph_string.split("^^").next() {
                            uri_part.trim_matches('"').trim_matches('<').trim_matches('>')
                        } else {
                            data_graph_string.trim_matches('"').trim_matches('<').trim_matches('>')
                        };
                        println!("Processed data graph URI: '{}'", data_graph_uri);
                        let data = self.extract_rdf_data_from_graph(data_graph_uri)?;
                        
                        let block = Block {
                            index,
                            timestamp,
                            data,
                            previous_hash,
                            hash,
                        };
                        
                        self.chain.push(block);
                    }
                }
            }
        }
        
        println!("Loaded {} blocks from persistent storage", self.chain.len());
        Ok(())
    }

    /// Extract RDF data from a specific graph
    fn extract_rdf_data_from_graph(&self, graph_uri: &str) -> Result<String> {
        
        // Debug output
        println!("Attempting to extract RDF data from graph: '{}'", graph_uri);
        
        let graph_name = NamedNode::new(graph_uri)?;
        
        // Collect all triples from the specific graph
        let mut triples = Vec::new();
        let graph_name_ref = oxigraph::model::GraphNameRef::NamedNode((&graph_name).into());
        for quad_result in self.rdf_store.store.quads_for_pattern(None, None, None, Some(graph_name_ref)) {
            if let Ok(quad) = quad_result {
                // Create a triple from the quad (without the graph component)
                let triple = oxigraph::model::Triple::new(
                    quad.subject,
                    quad.predicate,
                    quad.object,
                );
                triples.push(triple);
            }
        }
        
        println!("Found {} triples in graph '{}'", triples.len(), graph_uri);
        
        // If no triples, return empty string
        if triples.is_empty() {
            return Ok(String::new());
        }
        
        // Manually serialize triples to Turtle format
        let mut turtle_data = String::new();
        
        // Add prefixes (simplified - in a real implementation we'd extract actual prefixes)
        turtle_data.push_str("@prefix ex: <http://example.org/> .\n");
        turtle_data.push_str("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n");
        turtle_data.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n");
        turtle_data.push_str("@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n\n");
        
        // Serialize each triple
        for triple in triples {
            let subject_str = match &triple.subject {
                oxigraph::model::Subject::NamedNode(node) => format!("<{}>", node.as_str()),
                oxigraph::model::Subject::BlankNode(node) => format!("_:{}", node.as_str()),
                oxigraph::model::Subject::Triple(_) => "<http://example.org/quoted-triple>".to_string(), // Simplified
            };
            
            let predicate_str = match &triple.predicate {
                node => format!("<{}>", node.as_str()),
            };
            
            let object_str = match &triple.object {
                oxigraph::model::Term::NamedNode(node) => format!("<{}>", node.as_str()),
                oxigraph::model::Term::BlankNode(node) => format!("_:{}", node.as_str()),
                oxigraph::model::Term::Literal(lit) => format!("{}", lit),
                oxigraph::model::Term::Triple(_) => "<< >>".to_string(), // Simplified
            };
            
            turtle_data.push_str(&format!("{} {} {} .\n", subject_str, predicate_str, object_str));
        }
        
        println!("Generated Turtle data: {}", turtle_data);
        Ok(turtle_data)
    }

    /// Get storage statistics
    pub fn get_storage_stats(&self) -> Result<crate::rdf_store::StorageStats> {
        self.rdf_store.get_storage_stats()
    }

    /// Create a backup of the blockchain
    pub fn create_backup(&self) -> Result<crate::rdf_store::BackupInfo> {
        self.rdf_store.create_backup()
    }

    /// List available backups
    pub fn list_backups(&self) -> Result<Vec<crate::rdf_store::BackupInfo>> {
        self.rdf_store.list_backups()
    }

    /// Restore blockchain from backup
    pub fn restore_from_backup<P: AsRef<Path>>(backup_path: P, target_dir: P) -> Result<Self> {
        let rdf_store = RDFStore::restore_from_backup(backup_path, target_dir)?;
        
        let mut bc = Blockchain {
            chain: Vec::new(),
            rdf_store,
        };
        
        // Load the chain from the restored store
        bc.load_chain_from_store()?;
        
        Ok(bc)
    }

    /// Flush any pending writes to disk
    pub fn flush(&self) -> Result<()> {
        self.rdf_store.flush()
    }

    /// Optimize the underlying database
    pub fn optimize(&self) -> Result<()> {
        self.rdf_store.optimize()
    }

    /// Check database integrity
    pub fn check_integrity(&self) -> Result<crate::rdf_store::IntegrityReport> {
        self.rdf_store.check_integrity()
    }

    fn create_genesis_block(&self) -> Block {
        Block::new(
            0,
            "@prefix ex: <http://example.org/> . ex:genesis ex:type \"Genesis Block\".".into(),
            "0".into()
        )
    }

    pub fn add_block(&mut self, data: String) -> Result<()> {
        // Ensure we have at least a genesis block
        if self.chain.is_empty() {
            let genesis_block = self.create_genesis_block();
            let graph_name = NamedNode::new("http://provchain.org/block/0").unwrap();
            self.rdf_store.add_rdf_to_graph(&genesis_block.data, &graph_name);
            self.rdf_store.add_block_metadata(&genesis_block);
            self.chain.push(genesis_block);
        }

        let prev_block = self.chain.last().unwrap();
        let mut new_block = Block::new(prev_block.index + 1, data.clone(), prev_block.hash.clone());

        // Use atomic operations to ensure consistency
        let mut atomic_context = AtomicOperationContext::new(&mut self.rdf_store);
        
        // Add RDF data and block metadata atomically
        atomic_context.add_block_atomically(&new_block)?;
        
        // Recalculate hash using RDF canonicalization after successful atomic operation
        new_block.hash = new_block.calculate_hash_with_store(Some(&self.rdf_store));
        
        // Update the block metadata with the new hash
        self.rdf_store.add_block_metadata(&new_block);

        self.chain.push(new_block);
        
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let prev = &self.chain[i - 1];
            
            // Check if the block's data matches what's stored in the RDF store
            if !self.validate_block_data_integrity(current) {
                return false;
            }
            
            // Use RDF canonicalization for validation
            let expected_hash = current.calculate_hash_with_store(Some(&self.rdf_store));
            
            if current.hash != expected_hash || current.previous_hash != prev.hash {
                return false;
            }
        }
        true
    }

    /// Validate that the block's data field matches what's stored in the RDF store
    pub fn validate_block_data_integrity(&self, block: &Block) -> bool {
        // Create a temporary RDF store to parse the block's data
        let mut temp_rdf_store = crate::rdf_store::RDFStore::new();
        
        // Use the same graph name structure as the main store for proper comparison
        let graph_name = NamedNode::new(format!("http://provchain.org/block/{}", block.index)).unwrap();
        
        // Add the block's data to the temporary store using the same graph name
        temp_rdf_store.add_rdf_to_graph(&block.data, &graph_name);
        
        // Get canonical hash from temporary store
        let temp_canonical_hash = temp_rdf_store.canonicalize_graph(&graph_name);
        
        // Get canonical hash from main store for this block's graph
        let main_canonical_hash = self.rdf_store.canonicalize_graph(&graph_name);
        
        // Compare canonical hashes - this handles blank node differences correctly
        temp_canonical_hash == main_canonical_hash
    }

    fn load_ontology(&mut self) {
        if let Ok(ontology_data) = std::fs::read_to_string("ontology/traceability.owl.ttl") {
            let ontology_graph = NamedNode::new("http://provchain.org/ontology").unwrap();
            self.rdf_store.load_ontology(&ontology_data, &ontology_graph);
            println!("Loaded traceability ontology from ontology/traceability.owl.ttl");
        } else {
            eprintln!("Warning: Could not load ontology file ontology/traceability.owl.ttl");
        }
    }

    /// Enhanced trace function applying SSSP-inspired optimization concepts
    pub fn enhanced_trace(&self, batch_id: &str, optimization_level: u8) -> EnhancedTraceResult {
        let trace_system = EnhancedTraceabilitySystem::new(self);
        trace_system.enhanced_trace(batch_id, optimization_level)
    }

    pub fn dump(&self) -> String {
        serde_json::to_string_pretty(&self.chain).unwrap()
    }
}
