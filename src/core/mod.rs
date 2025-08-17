//! Core blockchain functionality
//!
//! This module contains the core blockchain implementation including
//! block structure, state management, and atomic operations.

pub mod blockchain;
pub mod atomic_operations;

// Re-exports for convenience
pub use blockchain::Blockchain;
pub use atomic_operations::AtomicOperationContext;
