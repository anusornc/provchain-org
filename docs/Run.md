# How to Run ProvChain Blockchain

This document provides comprehensive instructions for building and running the ProvChain blockchain system with ontology integration and RDF canonicalization.

## Prerequisites

- **Rust**: Install Rust and Cargo from [rustup.rs](https://rustup.rs/) (version 1.70+)
- **Git**: For cloning the repository (if needed)

## Quick Start

### 1. Build the Project

```bash
# Development build (faster compilation)
cargo build

# Release build (optimized performance)
cargo build --release
```

### 2. Run the Ontology-Integrated Demo

The easiest way to see the project in action is to run the built-in demo with ontology integration:

```bash
cargo run demo
```

This will:
- Automatically load the traceability ontology from `ontology/traceability.owl.ttl`
- Create a blockchain with ontology-validated supply chain data
- Add blocks containing RDF triples for farmer, processing, and transport data
- Validate the blockchain integrity using RDF canonicalization
- Dump the blockchain contents as JSON
- Run all available SPARQL queries against the data

**Expected Output:**
```
Loaded traceability ontology from ontology/traceability.owl.ttl
Blockchain valid? true

--- Blockchain Dump ---
[4 blocks with ontology-based supply chain data]

--- Running Queries ---
=== Running query: trace_by_batch_ontology.sparql ===
[Ontology-aware traceability results]

=== Running query: blockchain_metadata.sparql ===
[Blockchain metadata with timestamps and hashes]
```

## Available CLI Commands

The CLI application supports several commands for blockchain operations:

### Add RDF File as New Block

Add a Turtle (.ttl) RDF file as a new block to the blockchain:

```bash
cargo run -- add-file <path-to-rdf-file>
```

Examples:
```bash
# Add simple supply chain data
cargo run -- add-file test_data/simple_supply_chain_test.ttl

# Add comprehensive supply chain data
cargo run -- add-file test_data/complete_supply_chain_test.ttl

# Add minimal test data
cargo run -- add-file test_data/minimal_test_data.ttl
```

### Run SPARQL Query

Execute a SPARQL query file against the blockchain's RDF store:

```bash
cargo run -- query <path-to-sparql-file>
```

Examples:
```bash
# Ontology-aware batch tracing
cargo run -- query queries/trace_by_batch_ontology.sparql

# Traditional batch tracing
cargo run -- query queries/trace_by_batch.sparql

# Find product origins
cargo run -- query queries/trace_origin.sparql

# Environmental conditions analysis
cargo run -- query queries/env_conditions_for_batch.sparql

# Blockchain metadata
cargo run -- query queries/blockchain_metadata.sparql
```

### Validate Blockchain

Check the integrity of the blockchain using RDF canonicalization:

```bash
cargo run -- validate
```

This command:
- Validates all block hashes using RDF canonicalization
- Checks data integrity between blocks and RDF store
- Verifies the chain of previous hash references
- Reports any tampering or corruption

### Dump Blockchain

Output the entire blockchain as formatted JSON:

```bash
cargo run -- dump
```

## Sample Queries

The project includes several pre-built SPARQL queries in the `queries/` directory:

### Ontology-Aware Queries
- `trace_by_batch_ontology.sparql` - Trace products using ontology classes
- Advanced queries leveraging `trace:ProductBatch`, `trace:ProcessingActivity`, etc.

### Traditional Queries
- `trace_by_batch.sparql` - Trace products by batch ID
- `trace_origin.sparql` - Find the origin of products
- `env_conditions_for_batch.sparql` - Environmental conditions for batches
- `blockchain_metadata.sparql` - Blockchain metadata information

## Configuration

### Environment Variables

You can configure the system using environment variables:

```bash
# Network configuration
export PROVCHAIN_NETWORK_ID="my-supply-chain"
export PROVCHAIN_PORT=8080
export PROVCHAIN_PEERS="127.0.0.1:8081,127.0.0.1:8082"

# Authority configuration
export PROVCHAIN_AUTHORITY=true

# Storage configuration
export PROVCHAIN_DATA_DIR="./my-data"

# Logging configuration
export PROVCHAIN_LOG_LEVEL="debug"

# Run with custom configuration
cargo run demo
```

### Configuration File

Create a `config.toml` file for persistent configuration:

```toml
[network]
network_id = "provchain-supply-chain"
listen_port = 8080
known_peers = ["127.0.0.1:8081"]
max_peers = 50

[consensus]
is_authority = false
block_interval = 10

[storage]
data_dir = "./data"
persistent = true
store_type = "oxigraph"

[ontology]
path = "ontology/traceability.owl.ttl"
graph_name = "http://provchain.org/ontology"
auto_load = true
validate_data = false

[logging]
level = "info"
format = "pretty"
```

## Development

### Running Tests

Execute the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run unit tests only (96 tests)
cargo test --lib

# Run specific test categories
cargo test --test blockchain_tests          # Core blockchain (4 tests)
cargo test --test canonicalization_tests   # RDF canonicalization (3 tests)
cargo test --test ontology_integration_tests # Ontology features (5 tests)
cargo test --test comprehensive_user_journey_tests # User journey tests (4 tests)
cargo test --test e2e_api_workflows        # API workflow tests (7 tests)

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_ontology_loading
```

### Test Results Summary

As of August 12, 2025, the test suite has the following status:

**✅ Unit Tests**: 96/96 tests passing - Core functionality is solid
**✅ Core Integration Tests**: All passing - System integration works correctly
**✅ Performance Tests**: All passing - Benchmarking infrastructure is working
**⚠️ Infrastructure-Dependent Tests**: Some tests failing due to missing dependencies:
   - E2E Web Interface Tests: Failing due to missing ChromeDriver setup
   - Security Tests: Failing due to missing web server infrastructure
   - Ontology Integration Tests: Some failing due to test data dependencies

For detailed test analysis, see [TEST_VALIDATION_RESULTS.md](../tests/TEST_VALIDATION_RESULTS.md) and [FINAL_TEST_ANALYSIS_REPORT.md](../tests/FINAL_TEST_ANALYSIS_REPORT.md).

### Running E2E Tests with Web Server

To run the E2E tests that require a web server, you need to start the server first:

1. **Start the web server in one terminal:**
   ```bash
   cargo run --bin provchain-org -- web-server --port 8080
   ```

2. **Run E2E tests in another terminal:**
   ```bash
   # Run API workflow tests (don't require browser)
   cargo test --test e2e_api_workflows

   # Run user journey tests (don't require browser)
   cargo test --test comprehensive_user_journey_tests
   ```

3. **For browser-based tests, you need ChromeDriver:**
   ```bash
   # Install ChromeDriver (macOS)
   brew install --cask chromedriver

   # Start ChromeDriver
   chromedriver --port=9515 &

   # Run web interface tests
   cargo test --test e2e_web_interface
   ```

For more detailed instructions, see the [E2E Testing Guide](E2E_TESTING_GUIDE.md).

### Development Build

For faster compilation during development:

```bash
cargo build
cargo run demo
```

### Check Code Quality

```bash
# Check for compilation errors
cargo check

# Run clippy for code quality
cargo clippy

# Format code
cargo fmt
```

## Project Structure

```
src/
├── main.rs           # CLI entry point with subcommands
├── blockchain.rs     # Core blockchain with ontology integration
├── rdf_store.rs     # RDF store with canonicalization & validation
├── demo.rs          # Ontology-integrated demo
├── config.rs        # Comprehensive configuration management
├── network/         # Distributed networking foundation
│   ├── mod.rs       # Network manager
│   ├── messages.rs  # P2P message protocol
│   ├── peer.rs      # Peer connection management
│   └── discovery.rs # Peer discovery protocol
└── lib.rs          # Library exports

ontology/           # Traceability ontology
├── traceability.owl.ttl  # PROV-O extended ontology

test_data/          # Sample RDF data files
├── minimal_test_data.ttl
├── simple_supply_chain_test.ttl
└── complete_supply_chain_test.ttl

queries/            # SPARQL query examples
├── trace_by_batch_ontology.sparql  # Ontology-aware queries
├── trace_by_batch.sparql
├── trace_origin.sparql
├── env_conditions_for_batch.sparql
└── blockchain_metadata.sparql

tests/              # Comprehensive test suite
├── blockchain_tests.rs
├── rdf_tests.rs
├── canonicalization_tests.rs
├── ontology_integration_tests.rs
├── blockchain_with_test_data.rs
├── test_data_validation.rs
└── demo_tests.rs
```

## Creating Your Own RDF Data

### Ontology-Based Supply Chain Data

To create meaningful traceability data using the ontology, follow this structure:

```turtle
@prefix ex: <http://example.org/> .
@prefix trace: <http://provchain.org/trace#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

# Product batch with required properties
ex:milkBatch2 a trace:ProductBatch ;
    trace:hasBatchID "MB002" ;
    trace:producedAt "2025-08-09T08:00:00Z"^^xsd:dateTime ;
    prov:wasAttributedTo ex:FarmerMary .

# Agent with ontology classification
ex:FarmerMary a trace:Farmer ;
    rdfs:label "Mary's Organic Farm" .

# Processing activity with provenance
ex:processingActivity1 a trace:ProcessingActivity ;
    trace:recordedAt "2025-08-09T14:30:00Z"^^xsd:dateTime ;
    prov:used ex:milkBatch2 ;
    prov:wasAssociatedWith ex:ProcessingPlant .

# Transport with environmental conditions
ex:transport1 a trace:TransportActivity ;
    trace:recordedAt "2025-08-09T16:00:00Z"^^xsd:dateTime ;
    prov:used ex:processedMilk1 ;
    trace:hasCondition ex:condition1 .

ex:condition1 a trace:EnvironmentalCondition ;
    trace:hasTemperature "4.5"^^xsd:decimal ;
    trace:hasHumidity "62.0"^^xsd:decimal ;
    trace:hasConditionTimestamp "2025-08-09T16:00:00Z"^^xsd:dateTime .
```

Save this as `my_supply_chain_data.ttl` and add it to the blockchain:

```bash
cargo run -- add-file my_supply_chain_data.ttl
```

### Required Ontology Properties

When creating RDF data, ensure you include these required properties:

- **ProductBatch**: Must have `trace:hasBatchID`
- **Activities**: Must have `trace:recordedAt` timestamp
- **Environmental Conditions**: Should have temperature and humidity
- **Agents**: Should have proper ontology classification (Farmer, Manufacturer, etc.)

## Distributed Network (Foundation)

The project includes a foundation for distributed networking:

### Authority Node
```bash
# Start authority node
PROVCHAIN_PORT=8080 PROVCHAIN_AUTHORITY=true cargo run
```

### Regular Nodes
```bash
# Node 2
PROVCHAIN_PORT=8081 PROVCHAIN_PEERS="127.0.0.1:8080" cargo run

# Node 3
PROVCHAIN_PORT=8082 PROVCHAIN_PEERS="127.0.0.1:8080" cargo run
```

**Note**: Full P2P networking is currently in foundation stage. The protocol and message handling are implemented, but live synchronization is not yet complete.

## Example Workflows

### 1. Basic Exploration
```bash
# Start with the demo
cargo run demo

# Examine the blockchain
cargo run -- dump

# Run queries
cargo run -- query queries/blockchain_metadata.sparql
```

### 2. Add Custom Data
```bash
# Add your own supply chain data
cargo run -- add-file my_data.ttl

# Validate the blockchain
cargo run -- validate

# Query the new data
cargo run -- query queries/trace_by_batch_ontology.sparql
```

### 3. Development Testing
```bash
# Run all tests
cargo test

# Run specific functionality tests
cargo test --test ontology_integration_tests

# Check code quality
cargo clippy
```

## Troubleshooting

### Build Issues

If you encounter build issues:

```bash
# Update Rust
rustup update stable

# Clean and rebuild
cargo clean
cargo build
```

### Missing Files

Ensure you're running commands from the project root directory where `Cargo.toml` is located.

### Query Errors

Ensure SPARQL query files use the correct prefixes:
```sparql
PREFIX ex: <http://example.org/>
PREFIX trace: <http://provchain.org/trace#>
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
```

### Ontology Loading Issues

If ontology loading fails:
- Ensure `ontology/traceability.owl.ttl` exists
- Check file permissions
- Verify the file contains valid Turtle RDF

### Test Failures

If tests fail:
```bash
# Run tests with detailed output
cargo test -- --nocapture

# Run specific failing test
cargo test test_name -- --nocapture
```

## Performance Notes

- **RDF Canonicalization**: Computationally intensive but ensures semantic integrity
- **SPARQL Queries**: Performance depends on data complexity and query structure
- **Memory Usage**: Efficient for typical supply chain datasets
- **Blockchain Validation**: Fast due to optimized canonicalization algorithm

## Advanced Features

### RDF Canonicalization
- Deterministic hashing of RDF graphs
- Blank node canonicalization with Magic_S/Magic_O placeholders
- Semantic equivalence detection
- Blockchain integrity with varying RDF representations

### Ontology Integration
- Automatic loading on blockchain initialization
- Class-based validation for supply chain entities
- Required property enforcement
- Environmental condition integration

### Network Protocol Foundation
- WebSocket-based P2P communication
- Peer discovery and bootstrap mechanisms
- Message protocol for block synchronization
- Configuration management for network topology

## Next Steps

1. **Explore the Demo**: Start with `cargo run demo` to see all features
2. **Add Custom Data**: Create your own supply chain RDF data
3. **Run Tests**: Execute `cargo test` to verify functionality
4. **Experiment with Queries**: Try different SPARQL queries
5. **Configure Network**: Set up distributed nodes (foundation)

For more detailed information, see:
- [README.md](README.md) - Project overview and features
- [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md) - Technical details
- [TESTING_SUMMARY.md](TESTING_SUMMARY.md) - Testing analysis
- [ONTOLOGY_INTEGRATION_COMPLETE.md](ONTOLOGY_INTEGRATION_COMPLETE.md) - Ontology features
