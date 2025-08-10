# Qwen Code Context

This document provides context for Qwen Code about the 'UHT Traceability Blockchain (PoC)' project.

## Project Summary

This is a Rust proof-of-concept for a blockchain where each block stores RDF triples in a named graph using an in-memory Oxigraph store. The ontology is based on PROV-O, extended for traceability, with a focus on UHT manufacturing but designed to be applicable to other supply chains.

### Key Features

*   **RDF Storage:** Utilizes Oxigraph as an in-memory RDF dataset.
*   **Named Graphs:** Each blockchain block's data is stored in a separate named graph, identified by a URI like `http://example.org/block/{index}`.
*   **RDF Metadata:** Blockchain metadata itself is stored as RDF within a dedicated graph (`http://example.org/blockchain`).
*   **Simple Hashing:** Employs SHA-256 for block hashing (note: lacks RDF canonicalization in this PoC).

## Project Plan & Design

The goal is to implement a minimal blockchain backed by RDF to store traceability information and allow SPARQL queries.

### Technology Stack

*   **Language:** Rust
*   **RDF/SPARQL Engine:** Oxigraph
*   **Hashing:** sha2 + hex
*   **Timestamps:** chrono

### Implementation Steps

1.  Set up the Rust project.
2.  Develop core `Block` and `Blockchain` data structures.
3.  Implement logic to insert RDF triples into named graphs corresponding to blocks.
4.  Compute block hashes, ideally from a deterministic serialization of the graph's quads (RDF canonicalization is a future consideration).
5.  Store block metadata (e.g., index, previous hash, timestamp) as RDF.
6.  Add Command-Line Interface (CLI) commands for key operations: adding blocks, validating the chain, dumping data, and querying.
7.  Provide example datasets and sample SPARQL queries for demonstration.

### Acknowledged Limitations

*   **RDF Canonicalization:** The PoC does not implement RDF canonicalization (like URDNA2015), which is important for consistent hashing in a production environment. This is a known gap.
*   **Consensus & Security:** It's a single-writer chain without consensus mechanisms or cryptographic signatures for blocks.