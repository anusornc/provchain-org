# Gemini Summary: UHT Traceability Blockchain (PoC)

This document provides a summary of the project based on the information found in `README.md` and `Plan.md`.

## Project Goal

The goal of this project is to implement a minimal blockchain backed by RDF to store traceability triples. It provides SPARQL access to query provenance and traceability information. The project is a proof-of-concept for a UHT manufacturing traceability system, but it is general enough for other supply chains.

## Technology Stack

*   **Language:** Rust
*   **RDF Store:** Oxigraph (in-memory)
*   **Hashing:** sha2 + hex
*   **Timestamps:** chrono

## Implementation Steps

1.  Create a Rust project skeleton.
2.  Implement `Block` and `Blockchain` data structures.
3.  Insert triples as named graphs.
4.  Compute the block hash from a deterministic serialization of graph quads.
5.  Store block metadata as RDF.
6.  Add CLI commands for `add-block`, `validate`, `dump`, and `query`.
7.  Provide an example dataset and SPARQL queries.

## Key Features

*   The blockchain uses an in-memory RDF store (Oxigraph) as its dataset.
*   Each block in the blockchain corresponds to a named graph with a URI like `http://example.org/block/{index}`.
*   Block metadata is stored as RDF in the `http://example.org/blockchain` graph.
*   The project uses a simple SHA-256-based block hashing mechanism.

## Limitations

*   **RDF Canonicalization:** The current implementation is missing RDF canonicalization (e.g., URDNA2015). For a production environment, canonicalization should be added before hashing.
*   **Single-Writer Chain:** The blockchain is currently a single-writer chain and does not yet include consensus mechanisms or digital signatures.

## Operational Guidelines

*   **Repetitive Tool Calls:** To prevent infinite loops or excessive resource consumption, the agent will stop executing a sequence of identical tool calls if the count of such consecutive calls reaches 2 or more. This rule applies to any tool call that is repeated with the exact same arguments.