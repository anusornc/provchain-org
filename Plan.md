# Plan & Design

## Goal
Implement a minimal blockchain backed by RDF to store traceability triples. Provide SPARQL access to query provenance and traceability information.

## Tech choices
- Rust
- Oxigraph (in-memory RDF + SPARQL)
- sha2 + hex for hashing
- chrono for timestamps

## Steps
1. Create Rust project skeleton.
2. Implement Block and Blockchain data structures.
3. Insert triples as named graphs.
4. Compute block hash from deterministic serialization of graph quads.
5. Store block metadata as RDF.
6. Add CLI commands for add-block, validate, dump, query.
7. Provide example dataset and SPARQL queries.

## Limitations
- RDF canonicalization missing (URDNA2015) — for production, add canonicalization before hashing.
- Single-writer chain — no consensus or signatures yet.
