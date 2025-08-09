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
        // For now, use a simple hash of the data without RDF canonicalization
        // In production, proper RDF canonicalization should be implemented
        let record = format!(
            "{0}{1}{2}{3}",
            self.index,
            self.timestamp,
            self.data,
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
        let new_block = Block::new(prev_block.index + 1, data.clone(), prev_block.hash.clone());

        let graph_name = NamedNode::new(format!("http://example.org/block/{}", new_block.index)).unwrap();
        self.rdf_store.add_rdf_to_graph(&data, &graph_name);
        self.rdf_store.add_block_metadata(&new_block);

        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let prev = &self.chain[i - 1];
            if current.hash != current.calculate_hash() || current.previous_hash != prev.hash {
                return false;
            }
        }
        true
    }

    pub fn dump(&self) -> String {
        serde_json::to_string_pretty(&self.chain).unwrap()
    }
}
