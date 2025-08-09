use chrono::Utc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use crate::rdf_store::RDFStore;
use oxigraph::model::NamedNode;

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

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub rdf_store: RDFStore,
}

impl Blockchain {
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

    fn create_genesis_block(&self) -> Block {
        Block::new(
            0,
            "@prefix ex: <http://example.org/> . ex:genesis ex:type \"Genesis Block\".".into(),
            "0".into()
        )
    }

    pub fn add_block(&mut self, data: String) {
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

    pub fn dump(&self) -> String {
        serde_json::to_string_pretty(&self.chain).unwrap()
    }
}
