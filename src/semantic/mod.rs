//! Semantic web functionality
//!
//! This module contains semantic web implementations including
//! ontology management, SHACL validation, and SPARQL processing.

pub mod shacl_validator;

// Re-exports for convenience
pub use shacl_validator::ShaclValidator;
