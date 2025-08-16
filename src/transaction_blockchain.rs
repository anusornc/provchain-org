//! Enhanced blockchain with transaction and wallet integration
//! 
//! This module implements:
//! - Transaction-based blockchain operations
//! - Multi-participant wallet integration
//! - Supply chain specific transaction processing
//! - Persistent storage with disk persistence

use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use anyhow::{Result, anyhow};

use crate::blockchain::Blockchain;
use crate::transaction::{Transaction, TransactionPool, TransactionType, TransactionMetadata, TransactionOutput, EnvironmentalConditions, QualityData, TransactionPayload};
use crate::wallet::{WalletManager, Wallet, Participant, ParticipantType};

/// Enhanced blockchain with transaction support
pub struct TransactionBlockchain {
    /// Underlying blockchain
    pub blockchain: Blockchain,
    /// Transaction pool for pending transactions
    pub transaction_pool: TransactionPool,
    /// Wallet manager for participants
    pub wallet_manager: WalletManager,
    /// Transaction index for fast lookups
    pub transaction_index: HashMap<String, (u64, usize)>, // tx_id -> (block_index, tx_index)
    /// UTXO set for tracking unspent outputs
    pub utxo_set: HashMap<String, TransactionOutput>, // output_id -> output
}

impl TransactionBlockchain {
    /// Create a new transaction blockchain
    pub fn new(data_dir: &str) -> Result<Self> {
        let blockchain = Blockchain::new_persistent(data_dir)?;
        let transaction_pool = TransactionPool::new(1000); // Max 1000 pending transactions
        let wallet_manager = WalletManager::new(format!("{}/wallets", data_dir))?;
        
        Ok(Self {
            blockchain,
            transaction_pool,
            wallet_manager,
            transaction_index: HashMap::new(),
            utxo_set: HashMap::new(),
        })
    }

    /// Submit a transaction to the blockchain
    pub fn submit_transaction(&mut self, transaction: Transaction) -> Result<String> {
        // Validate transaction
        transaction.validate()?;
        
        // Check if submitter has permission
        if let Some(signer) = transaction.signatures.first() {
            if let Some(wallet) = self.wallet_manager.get_wallet(signer.signer_id) {
                let operation = match transaction.tx_type {
                    TransactionType::Production => "produce",
                    TransactionType::Processing => "process",
                    TransactionType::Transport => "transport",
                    TransactionType::Quality => "quality_test",
                    TransactionType::Compliance => "audit",
                    TransactionType::Transfer => "transfer",
                    _ => "unknown",
                };
                
                if !wallet.has_permission(operation) {
                    return Err(anyhow!("Participant does not have permission for {} operation", operation));
                }
            } else {
                return Err(anyhow!("Unknown participant"));
            }
        }

        // Add to transaction pool
        let tx_id = transaction.id.clone();
        self.transaction_pool.add_transaction(transaction)?;
        
        Ok(tx_id)
    }

    /// Create a new block with pending transactions
    pub fn create_block(&mut self, max_transactions: usize) -> Result<()> {
        let transactions = self.transaction_pool.get_transactions_for_block(max_transactions);
        
        if transactions.is_empty() {
            return Ok(()); // No transactions to process
        }

        // Create block data from transactions
        let block_data = self.create_block_rdf_data(&transactions)?;
        
        // Add block to blockchain
        self.blockchain.add_block(block_data)?;
        
        // Update transaction index and UTXO set
        let block_index = self.blockchain.chain.len() as u64 - 1;
        for (tx_index, transaction) in transactions.iter().enumerate() {
            // Update transaction index
            self.transaction_index.insert(
                transaction.id.clone(),
                (block_index, tx_index)
            );
            
            // Update UTXO set
            for output in &transaction.outputs {
                self.utxo_set.insert(output.id.clone(), output.clone());
            }
            
            // Remove spent outputs
            for input in &transaction.inputs {
                self.utxo_set.remove(&format!("{}:{}", input.prev_tx_id, input.output_index));
            }
            
            // Remove from transaction pool
            self.transaction_pool.remove_transaction(&transaction.id);
        }

        Ok(())
    }

    /// Create RDF data for a block from transactions
    fn create_block_rdf_data(&self, transactions: &[Transaction]) -> Result<String> {
        let mut rdf_data = String::new();
        
        // Add prefixes
        rdf_data.push_str(r#"
@prefix ex: <http://example.org/> .
@prefix trace: <http://provchain.org/trace#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix tx: <http://provchain.org/tx#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

"#);

        // Add transaction data
        for transaction in transactions {
            rdf_data.push_str(&transaction.to_rdf());
            rdf_data.push('\n');
            
            // Add the original RDF data from the transaction
            rdf_data.push_str(&transaction.rdf_data);
            rdf_data.push('\n');
        }

        Ok(rdf_data)
    }

    /// Get transaction by ID
    pub fn get_transaction(&self, tx_id: &str) -> Option<Transaction> {
        // First check transaction pool
        if let Some(tx) = self.transaction_pool.pending.get(tx_id) {
            return Some(tx.clone());
        }

        // Then check blockchain
        if let Some((_block_index, _tx_index)) = self.transaction_index.get(tx_id) {
            // In a full implementation, we would parse the block data to extract the transaction
            // For now, we'll return None as this requires more complex RDF parsing
            None
        } else {
            None
        }
    }

    /// Get transactions by participant
    pub fn get_transactions_by_participant(&self, participant_id: Uuid) -> Vec<String> {
        let mut tx_ids = Vec::new();
        
        // Check transaction pool
        for tx in self.transaction_pool.pending.values() {
            if tx.signatures.iter().any(|sig| sig.signer_id == participant_id) {
                tx_ids.push(tx.id.clone());
            }
        }
        
        // In a full implementation, we would also search the blockchain
        tx_ids
    }

    /// Register a new participant
    pub fn register_participant(&mut self, participant: Participant) -> Result<Uuid> {
        self.wallet_manager.create_wallet(participant)
    }

    /// Get participant wallet
    pub fn get_participant_wallet(&self, participant_id: Uuid) -> Option<&Wallet> {
        self.wallet_manager.get_wallet(participant_id)
    }

    /// Create a production transaction (farmer produces raw materials)
    pub fn create_production_transaction(
        &self,
        producer_id: Uuid,
        batch_id: String,
        quantity: f64,
        location: String,
        environmental_conditions: Option<EnvironmentalConditions>,
    ) -> Result<Transaction> {
        let wallet = self.wallet_manager.get_wallet(producer_id)
            .ok_or_else(|| anyhow!("Producer wallet not found"))?;

        if !wallet.has_permission("produce") {
            return Err(anyhow!("Producer does not have production permission"));
        }

        // Create transaction output
        let output = TransactionOutput {
            id: format!("{}:0", batch_id),
            owner: producer_id,
            asset_type: "raw_material_batch".to_string(),
            value: quantity,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("batch_id".to_string(), batch_id.clone());
                meta.insert("location".to_string(), location.clone());
                meta
            },
        };

        // Create RDF data
        let rdf_data = format!(
            r#"
ex:{} a trace:ProductBatch ;
    trace:hasBatchID "{}" ;
    trace:producedAt "{}"^^xsd:dateTime ;
    prov:wasAttributedTo ex:participant_{} ;
    trace:hasQuantity "{}"^^xsd:decimal ;
    trace:hasLocation "{}" .

ex:participant_{} a trace:Farmer ;
    rdfs:label "{}" .
"#,
            batch_id,
            batch_id,
            Utc::now().to_rfc3339(),
            producer_id,
            quantity,
            location,
            producer_id,
            wallet.participant.name
        );

        let metadata = TransactionMetadata {
            location: Some(location),
            environmental_conditions,
            compliance_info: None,
            quality_data: None,
            custom_fields: HashMap::new(),
        };

        let mut transaction = Transaction::new(
            TransactionType::Production,
            vec![], // No inputs for production
            vec![output],
            rdf_data.clone(),
            metadata,
            TransactionPayload::RdfData(rdf_data.clone()),
        );

        // Sign the transaction
        transaction.sign(
            wallet.signing_key.as_ref().unwrap(),
            producer_id,
        )?;

        Ok(transaction)
    }

    /// Create a processing transaction (manufacturer processes raw materials)
    pub fn create_processing_transaction(
        &self,
        processor_id: Uuid,
        input_batch_ids: Vec<String>,
        output_batch_id: String,
        process_type: String,
        environmental_conditions: Option<EnvironmentalConditions>,
    ) -> Result<Transaction> {
        let wallet = self.wallet_manager.get_wallet(processor_id)
            .ok_or_else(|| anyhow!("Processor wallet not found"))?;

        if !wallet.has_permission("process") {
            return Err(anyhow!("Processor does not have processing permission"));
        }

        // Create transaction inputs (simplified - in reality we'd look up the actual UTXOs)
        let inputs = input_batch_ids.iter().enumerate().map(|(_i, batch_id)| {
            crate::transaction::TransactionInput {
                prev_tx_id: batch_id.clone(),
                output_index: 0,
                signature: None,
                public_key: None,
            }
        }).collect();

        // Create transaction output
        let output = TransactionOutput {
            id: format!("{}:0", output_batch_id),
            owner: processor_id,
            asset_type: "processed_batch".to_string(),
            value: 1.0, // Simplified
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("batch_id".to_string(), output_batch_id.clone());
                meta.insert("process_type".to_string(), process_type.clone());
                meta
            },
        };

        // Create RDF data
        let rdf_data = format!(
            r#"
ex:{} a trace:ProductBatch ;
    trace:hasBatchID "{}" ;
    trace:producedAt "{}"^^xsd:dateTime ;
    prov:wasGeneratedBy ex:process_{} ;
    prov:wasAttributedTo ex:participant_{} .

ex:process_{} a trace:ProcessingActivity ;
    trace:recordedAt "{}"^^xsd:dateTime ;
    trace:hasProcessType "{}" ;
    prov:wasAssociatedWith ex:participant_{} .

ex:participant_{} a trace:Manufacturer ;
    rdfs:label "{}" .
"#,
            output_batch_id,
            output_batch_id,
            Utc::now().to_rfc3339(),
            output_batch_id,
            processor_id,
            output_batch_id,
            Utc::now().to_rfc3339(),
            process_type,
            processor_id,
            processor_id,
            wallet.participant.name
        );

        let metadata = TransactionMetadata {
            location: wallet.participant.location.clone(),
            environmental_conditions,
            compliance_info: None,
            quality_data: None,
            custom_fields: HashMap::new(),
        };

        let mut transaction = Transaction::new(
            TransactionType::Processing,
            inputs,
            vec![output],
            rdf_data.clone(),
            metadata,
            TransactionPayload::RdfData(rdf_data.clone()),
        );

        // Sign the transaction
        transaction.sign(
            wallet.signing_key.as_ref().unwrap(),
            processor_id,
        )?;

        Ok(transaction)
    }

    /// Create a quality control transaction
    pub fn create_quality_transaction(
        &self,
        lab_id: Uuid,
        batch_id: String,
        test_type: String,
        test_result: String,
        test_value: Option<f64>,
    ) -> Result<Transaction> {
        let wallet = self.wallet_manager.get_wallet(lab_id)
            .ok_or_else(|| anyhow!("Lab wallet not found"))?;

        if !wallet.has_permission("quality_test") {
            return Err(anyhow!("Lab does not have quality testing permission"));
        }

        let quality_data = QualityData {
            test_type: test_type.clone(),
            test_result: test_result.clone(),
            test_value,
            test_unit: None,
            lab_id: Some(lab_id),
            test_timestamp: Utc::now(),
        };

        // Create RDF data
        let rdf_data = format!(
            r#"
ex:quality_test_{} a trace:QualityCheck ;
    prov:used ex:{} ;
    prov:wasAssociatedWith ex:participant_{} ;
    trace:recordedAt "{}"^^xsd:dateTime ;
    trace:hasResult "{}" ;
    trace:hasTestType "{}" .

ex:participant_{} a trace:QualityLab ;
    rdfs:label "{}" .
"#,
            batch_id,
            batch_id,
            lab_id,
            Utc::now().to_rfc3339(),
            test_result,
            test_type,
            lab_id,
            wallet.participant.name
        );

        let metadata = TransactionMetadata {
            location: wallet.participant.location.clone(),
            environmental_conditions: None,
            compliance_info: None,
            quality_data: Some(quality_data),
            custom_fields: HashMap::new(),
        };

        let mut transaction = Transaction::new(
            TransactionType::Quality,
            vec![], // Quality checks don't consume inputs
            vec![], // Quality checks don't produce outputs
            rdf_data.clone(),
            metadata,
            TransactionPayload::RdfData(rdf_data.clone()),
        );

        // Sign the transaction
        transaction.sign(
            wallet.signing_key.as_ref().unwrap(),
            lab_id,
        )?;

        Ok(transaction)
    }

    /// Create a transport transaction
    pub fn create_transport_transaction(
        &self,
        logistics_id: Uuid,
        batch_id: String,
        from_location: String,
        to_location: String,
        environmental_conditions: Option<EnvironmentalConditions>,
    ) -> Result<Transaction> {
        let wallet = self.wallet_manager.get_wallet(logistics_id)
            .ok_or_else(|| anyhow!("Logistics provider wallet not found"))?;

        if !wallet.has_permission("transport") {
            return Err(anyhow!("Provider does not have transport permission"));
        }

        // Create RDF data
        let rdf_data = format!(
            r#"
ex:transport_{} a trace:TransportActivity ;
    prov:used ex:{} ;
    prov:wasAssociatedWith ex:participant_{} ;
    trace:recordedAt "{}"^^xsd:dateTime ;
    trace:hasFromLocation "{}" ;
    trace:hasToLocation "{}" .

ex:participant_{} a trace:LogisticsProvider ;
    rdfs:label "{}" .
"#,
            batch_id,
            batch_id,
            logistics_id,
            Utc::now().to_rfc3339(),
            from_location,
            to_location,
            logistics_id,
            wallet.participant.name
        );

        let metadata = TransactionMetadata {
            location: Some(format!("{} -> {}", from_location, to_location)),
            environmental_conditions,
            compliance_info: None,
            quality_data: None,
            custom_fields: HashMap::new(),
        };

        let mut transaction = Transaction::new(
            TransactionType::Transport,
            vec![], // Simplified - no inputs/outputs for transport
            vec![],
            rdf_data.clone(),
            metadata,
            TransactionPayload::RdfData(rdf_data.clone()),
        );

        // Sign the transaction
        transaction.sign(
            wallet.signing_key.as_ref().unwrap(),
            logistics_id,
        )?;

        Ok(transaction)
    }

    /// Get blockchain statistics
    pub fn get_statistics(&self) -> TransactionBlockchainStats {
        let pool_stats = self.transaction_pool.get_stats();
        let wallet_stats = self.wallet_manager.get_statistics();
        
        TransactionBlockchainStats {
            total_blocks: self.blockchain.chain.len(),
            pending_transactions: pool_stats.total_transactions,
            total_participants: wallet_stats.total_participants,
            total_utxos: self.utxo_set.len(),
            participant_distribution: wallet_stats.type_distribution,
            transaction_distribution: pool_stats.type_distribution,
        }
    }

    /// Validate the entire blockchain
    pub fn validate(&self) -> bool {
        self.blockchain.is_valid()
    }

    /// Save blockchain state to disk
    pub fn save_to_disk(&self) -> Result<()> {
        self.blockchain.rdf_store.save_to_disk()
    }
}

/// Statistics for the transaction blockchain
#[derive(Debug, Clone)]
pub struct TransactionBlockchainStats {
    pub total_blocks: usize,
    pub pending_transactions: usize,
    pub total_participants: usize,
    pub total_utxos: usize,
    pub participant_distribution: HashMap<ParticipantType, usize>,
    pub transaction_distribution: HashMap<TransactionType, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::wallet::Participant;

    #[test]
    fn test_transaction_blockchain_creation() {
        let temp_dir = tempdir().unwrap();
        let blockchain = TransactionBlockchain::new(temp_dir.path().to_str().unwrap()).unwrap();
        
        let stats = blockchain.get_statistics();
        assert_eq!(stats.total_blocks, 1); // Genesis block
        assert_eq!(stats.pending_transactions, 0);
        assert_eq!(stats.total_participants, 0);
    }

    #[test]
    fn test_participant_registration() {
        let temp_dir = tempdir().unwrap();
        let mut blockchain = TransactionBlockchain::new(temp_dir.path().to_str().unwrap()).unwrap();
        
        let farmer = Participant::new_farmer(
            "John's Dairy Farm".to_string(),
            "Vermont, USA".to_string()
        );
        
        let participant_id = blockchain.register_participant(farmer).unwrap();
        assert!(blockchain.get_participant_wallet(participant_id).is_some());
        
        let stats = blockchain.get_statistics();
        assert_eq!(stats.total_participants, 1);
    }

    #[test]
    fn test_production_transaction() {
        let temp_dir = tempdir().unwrap();
        let mut blockchain = TransactionBlockchain::new(temp_dir.path().to_str().unwrap()).unwrap();
        
        let farmer = Participant::new_farmer(
            "John's Dairy Farm".to_string(),
            "Vermont, USA".to_string()
        );
        
        let farmer_id = blockchain.register_participant(farmer).unwrap();
        
        let tx = blockchain.create_production_transaction(
            farmer_id,
            "MILK-001".to_string(),
            1000.0,
            "Vermont, USA".to_string(),
            None,
        ).unwrap();
        
        assert_eq!(tx.tx_type, TransactionType::Production);
        assert_eq!(tx.outputs.len(), 1);
        assert_eq!(tx.outputs[0].value, 1000.0);
    }
}
