use chrono::Utc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

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
        let record = format!(
            "{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut bc = Blockchain { chain: Vec::new() };
        bc.chain.push(bc.create_genesis_block());
        bc
    }

    fn create_genesis_block(&self) -> Block {
        Block::new(
            0,
            "@prefix ex: <http://example.org/> . ex:genesis ex:type \"Genesis Block\" .".into(),
            "0".into(),
        )
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block = self.chain.last().unwrap();
        let new_block = Block::new(
            prev_block.index + 1,
            data,
            prev_block.hash.clone(),
        );
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
}
