//! Transaction processing functionality
//!
//! This module contains transaction processing, validation, and blockchain integration.

pub mod transaction;
pub mod blockchain;

// Re-exports for convenience
pub use transaction::Transaction;
pub use blockchain::TransactionBlockchain;
