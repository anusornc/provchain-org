use chrono::Utc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use crate::rdf_store::{RDFStore, StorageConfig};
use crate::trace_optimization::{EnhancedTraceabilitySystem, EnhancedTraceResult};
use oxigraph::model::NamedNode;
use std::path::Path;
use anyhow::Result;

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
            
            // Save to disk if persistent
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
                        let data_graph_uri = data_graph_string.trim_matches('"');
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
        use oxigraph::io::RdfFormat;
        
        // Create a temporary store to export the graph data
        let temp_store = oxigraph::store::Store::new()?;
        let graph_name = NamedNode::new(graph_uri)?;
        
        // Copy all quads from the specific graph
        let graph_name_ref = oxigraph::model::GraphNameRef::NamedNode((&graph_name).into());
        for quad_result in self.rdf_store.store.quads_for_pattern(None, None, None, Some(graph_name_ref)) {
            if let Ok(quad) = quad_result {
                // Create a new quad without the graph component for export
                let triple_quad = oxigraph::model::Quad::new(
                    quad.subject,
                    quad.predicate,
                    quad.object,
                    oxigraph::model::GraphName::DefaultGraph,
                );
                temp_store.insert(&triple_quad)?;
            }
        }
        
        // Export as Turtle
        let mut buffer = Vec::new();
        temp_store.dump_to_writer(RdfFormat::Turtle, &mut buffer)?;
        
        Ok(String::from_utf8(buffer)?)
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

    pub fn add_block(&mut self, data: String) {
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

        // Add RDF data to the store first
        let graph_name = NamedNode::new(format!("http://provchain.org/block/{}", new_block.index)).unwrap();
        self.rdf_store.add_rdf_to_graph(&data, &graph_name);
        
        // Recalculate hash using RDF canonicalization
        new_block.hash = new_block.calculate_hash_with_store(Some(&self.rdf_store));
        
        // Add block metadata to store
        self.rdf_store.add_block_metadata(&new_block);

        self.chain.push(new_block);
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
