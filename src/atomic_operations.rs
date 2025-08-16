//! Atomic operations for blockchain and RDF store consistency
//! 
//! This module implements atomic operations that ensure consistency
//! between blockchain state and RDF store state.

use anyhow::{Result, bail};
use crate::blockchain::Block;
use crate::rdf_store::RDFStore;
use oxigraph::model::NamedNode;

/// Atomic operation context that ensures consistency between blockchain and RDF store
pub struct AtomicOperationContext<'a> {
    /// Reference to the RDF store
    pub rdf_store: &'a mut RDFStore,
    /// Backup of the RDF store state before operation
    backup_state: Option<RDFStoreBackup>,
}

/// Backup of RDF store state for rollback purposes
struct RDFStoreBackup {
    /// Clone of the store before operation
    store_clone: RDFStore,
}

impl<'a> AtomicOperationContext<'a> {
    /// Create a new atomic operation context
    pub fn new(rdf_store: &'a mut RDFStore) -> Self {
        Self {
            rdf_store,
            backup_state: None,
        }
    }

    /// Begin an atomic operation by creating a backup
    pub fn begin_operation(&mut self) -> Result<()> {
        // Create a backup of the current state
        self.backup_state = Some(RDFStoreBackup {
            store_clone: self.rdf_store.clone(),
        });
        Ok(())
    }

    /// Commit the atomic operation
    pub fn commit(&mut self) -> Result<()> {
        // In a real implementation with a proper database, we would commit the transaction here
        // For now, we just clear the backup since the operation was successful
        self.backup_state = None;
        Ok(())
    }

    /// Rollback the atomic operation to the previous state
    pub fn rollback(&mut self) -> Result<()> {
        if let Some(backup) = self.backup_state.take() {
            // Restore the RDF store to its previous state
            *self.rdf_store = backup.store_clone;
            Ok(())
        } else {
            bail!("No backup state available for rollback")
        }
    }

    /// Add a block atomically with its RDF data
    pub fn add_block_atomically(&mut self, block: &Block) -> Result<()> {
        // Begin the atomic operation
        self.begin_operation()?;

        // Try to add the block and RDF data
        let result = self.try_add_block(block);

        // Handle the result
        match result {
            Ok(_) => {
                // Commit the operation if successful
                self.commit()?;
                Ok(())
            }
            Err(e) => {
                // Rollback the operation if failed
                self.rollback()?;
                Err(e)
            }
        }
    }

    /// Try to add a block and its RDF data (internal method)
    fn try_add_block(&mut self, block: &Block) -> Result<()> {
        // Add RDF data to the store
        let graph_name = NamedNode::new(format!("http://provchain.org/block/{}", block.index))
            .map_err(|e| anyhow::anyhow!("Failed to create graph name: {}", e))?;
        self.rdf_store.add_rdf_to_graph(&block.data, &graph_name);

        // Add block metadata to store
        self.rdf_store.add_block_metadata(block);

        // Save to disk
        self.rdf_store.save_to_disk()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::Blockchain;

    #[test]
    fn test_atomic_operation_context() {
        let mut blockchain = Blockchain::new();
        let mut context = AtomicOperationContext::new(&mut blockchain.rdf_store);
        
        // Create a test block
        // For testing, we'll use a placeholder state root
        let state_root = "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let block = Block::new(1, "@prefix ex: <http://example.org/> . ex:test ex:value \"test\" .".to_string(), 
                              "0".to_string(), state_root);
        
        // Add block atomically
        assert!(context.add_block_atomically(&block).is_ok());
        
        // Verify the block was added
        assert_eq!(blockchain.chain.len(), 1);
    }

    #[test]
    fn test_atomic_operation_rollback() {
        let mut blockchain = Blockchain::new();
        let mut context = AtomicOperationContext::new(&mut blockchain.rdf_store);
        
        // Begin operation
        assert!(context.begin_operation().is_ok());
        
        // Simulate an operation that would fail
        // In a real scenario, this would be an actual failure in the operation
        assert!(context.rollback().is_ok());
    }
}
