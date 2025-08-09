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
    fn validate_block_data_integrity(&self, block: &Block) -> bool {
        // Create a temporary store to parse the block's data
        let temp_store = oxigraph::store::Store::new().unwrap();
        let reader = std::io::Cursor::new(block.data.as_bytes());
        
        // Try to parse the block's data as RDF
        match temp_store.load_from_reader(oxigraph::io::RdfFormat::Turtle, reader) {
            Ok(_) => {
                // Compare the parsed data with what's in our RDF store for this block's graph
                let graph_name = NamedNode::new(format!("http://provchain.org/block/{}", block.index)).unwrap();
                
                // Get triples from both stores and compare
                let mut temp_triples = Vec::new();
                for quad in temp_store.iter() {
                    if let Ok(q) = quad {
                        temp_triples.push(format!("{} {} {}", q.subject, q.predicate, q.object));
                    }
                }
                
                let mut store_triples = Vec::new();
                for quad_result in self.rdf_store.store.quads_for_pattern(None, None, None, Some((&graph_name).into())) {
                    if let Ok(q) = quad_result {
                        store_triples.push(format!("{} {} {}", q.subject, q.predicate, q.object));
                    }
                }
                
                // Sort both sets for comparison
                temp_triples.sort();
                store_triples.sort();
                
                temp_triples == store_triples
            }
            Err(_) => {
                // If it's not valid RDF, check if it matches the simple literal we would have created
                let expected_subject = NamedNode::new(format!("http://provchain.org/data/{}", block.index)).unwrap();
                let expected_predicate = NamedNode::new("http://provchain.org/hasData").unwrap();
                let expected_object = oxigraph::model::Literal::new_simple_literal(&block.data);
                let graph_name = NamedNode::new(format!("http://provchain.org/block/{}", block.index)).unwrap();
                
                // Check if this exact triple exists in the store
                self.rdf_store.store.contains(&oxigraph::model::Quad::new(
                    expected_subject,
                    expected_predicate,
                    expected_object,
                    graph_name
                )).unwrap_or(false)
            }
        }
    }

    pub fn dump(&self) -> String {
        serde_json::to_string_pretty(&self.chain).unwrap()
    }
}
