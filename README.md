# ProvChainOrg

A distributed blockchain system implementing the GraphChain concept for supply chain traceability using RDF graphs and SPARQL queries. This project demonstrates semantic blockchain technology with provenance tracking capabilities.

## Overview

ProvChainOrg is a production-ready implementation of the GraphChain concept from the research paper "GraphChain – A Distributed Database with Explicit Semantics and Chained RDF Graphs" by Sopek et al. (2018). It combines blockchain security with semantic web technologies to create a distributed ledger for supply chain traceability.

## Key Features

- **RDF-Native Blockchain**: Blocks reference RDF graphs directly, not embedded data
- **Semantic Data Access**: Full SPARQL query support across all blockchain data
- **Distributed P2P Network**: WebSocket-based peer communication and discovery
- **Supply Chain Traceability**: Track products from origin to consumer
- **Cryptographic Integrity**: Hash-linked blocks with RDF graph verification
- **Modern Rust Implementation**: High performance with memory safety

## Architecture

### Single Node
- Oxigraph RDF triplestore for semantic data storage
- Named graphs for organizing blockchain data
- SPARQL endpoint for querying
- Turtle serialization for RDF data

### Distributed Network
- P2P networking with WebSocket communication
- Peer discovery and bootstrap mechanisms
- Block synchronization across nodes
- RDF graph replication

## Quick Start

### Prerequisites
- Rust 1.70+ 
- Cargo

### Single Node Demo
```bash
# Clone the repository
git clone https://github.com/anusornc/provchain-org.git
cd provchain-org

# Run the demo
cargo run --bin demo

# Run tests
cargo test
```

### Distributed Network
```bash
# Node 1 (Authority)
PROVCHAIN_PORT=8080 PROVCHAIN_AUTHORITY=true cargo run

# Node 2 (Regular)
PROVCHAIN_PORT=8081 PROVCHAIN_PEERS="127.0.0.1:8080" cargo run

# Node 3 (Regular)  
PROVCHAIN_PORT=8082 PROVCHAIN_PEERS="127.0.0.1:8080" cargo run
```

## Configuration

Create a `config.toml` file:

```toml
[network]
network_id = "provchain-org-default"
listen_port = 8080
known_peers = ["127.0.0.1:8081"]

[consensus]
is_authority = false

[storage]
data_dir = "./data"
persistent = true
store_type = "oxigraph"
```

## Use Case: Supply Chain Traceability

Track products through the supply chain with environmental monitoring:

1. **Origin**: Farm creates batch with location, timestamp, conditions
2. **Processing**: Add processing steps, quality checks
3. **Distribution**: Record transportation, storage conditions  
4. **Retail**: Final destination and consumer access

### Example SPARQL Queries

```sparql
# Trace origin of a specific batch
SELECT ?origin ?timestamp WHERE {
    tc:batch_001 tc:hasOrigin ?origin ;
                 tc:hasTimestamp ?timestamp .
}

# Find batches with temperature issues
SELECT ?batch ?temp WHERE {
    ?batch tc:hasTemperature ?temp .
    FILTER(?temp > 25.0)
}
```

## Project Structure

```
src/
├── blockchain.rs      # Core blockchain logic
├── rdf_store.rs      # RDF graph storage and queries
├── demo.rs           # Demo application
├── network/          # Distributed networking
│   ├── mod.rs        # Network manager
│   ├── messages.rs   # P2P message protocol
│   ├── peer.rs       # Peer connections
│   └── discovery.rs  # Peer discovery
├── config.rs         # Configuration management
└── lib.rs           # Library exports

test_data/           # Sample RDF data
ontology/           # Traceability ontology
queries/            # SPARQL query examples
tests/              # Integration tests
```

## Technology Stack

- **Rust**: Systems programming language for performance and safety
- **Oxigraph**: RDF triplestore for semantic data storage
- **Tokio**: Async runtime for networking
- **WebSockets**: P2P communication protocol
- **Serde**: Serialization for messages and configuration
- **SHA-256**: Cryptographic hashing
- **Ed25519**: Digital signatures (prepared for consensus)

## Testing

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test blockchain_with_test_data
cargo test --test simple_blockchain_test
cargo test --test demo_tests
```

## Documentation

- [Implementation Summary](IMPLEMENTATION_SUMMARY.md) - Complete project overview
- [GraphChain Comparison](GRAPHCHAIN_COMPARISON.md) - Comparison with research paper
- [Run Instructions](Run.md) - Detailed running instructions

## Research Background

This implementation is based on the GraphChain concept from:

> Sopek, M., Grądzki, P., Kosowski, W., Kuziński, D., Trójczak, R., & Trypuz, R. (2018). GraphChain – A Distributed Database with Explicit Semantics and Chained RDF Graphs. In Proceedings of The 2018 Web Conference Companion (WWW'18 Companion).

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Contact

For questions or collaboration opportunities, please open an issue on GitHub.
