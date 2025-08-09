# How to Run UHT Traceability Blockchain

This document provides instructions for building and running the UHT Traceability Blockchain proof-of-concept.

## Prerequisites

- **Rust**: Install Rust and Cargo from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository (if needed)

## Quick Start

### 1. Build the Project

```bash
cargo build --release
```

### 2. Run the Built-in Demo

The easiest way to see the project in action is to run the built-in demo:

```bash
cargo run -- demo
```

This will:
- Create a blockchain with sample UHT manufacturing data
- Add blocks containing RDF triples for farmer and manufacturer data
- Validate the blockchain integrity
- Dump the blockchain contents as JSON
- Run all available SPARQL queries against the data

## Available Commands

The CLI application supports several commands:

### Add RDF File as New Block

Add a Turtle (.ttl) RDF file as a new block to the blockchain:

```bash
cargo run -- add-file <path-to-rdf-file>
```

Example:
```bash
cargo run -- add-file test_tracechain.ttl
```

**Note about test_tracechain.ttl**: This is a minimal test file containing only basic namespace declarations and a simple test statement. It's primarily used for testing the RDF parsing functionality, not for demonstrating the full traceability capabilities.

For more comprehensive traceability data examples, refer to the built-in demo which includes:
- Farmer data with milk batch information
- Manufacturing processes and timestamps
- Product relationships and provenance chains
- Environmental conditions and metadata

### Run SPARQL Query

Execute a SPARQL query file against the blockchain's RDF store:

```bash
cargo run -- query <path-to-sparql-file>
```

Example:
```bash
cargo run -- query queries/trace_by_batch.sparql
```

### Validate Blockchain

Check the integrity of the blockchain:

```bash
cargo run -- validate
```

### Dump Blockchain

Output the entire blockchain as JSON:

```bash
cargo run -- dump
```

## Sample Queries

The project includes several pre-built SPARQL queries in the `queries/` directory:

- `trace_by_batch.sparql` - Trace products by milk batch
- `trace_origin.sparql` - Find the origin of products
- `env_conditions_for_batch.sparql` - Environmental conditions for batches
- `blockchain_metadata.sparql` - Blockchain metadata information

## Development

### Running Tests

Execute the test suite:

```bash
cargo test
```

### Development Build

For faster compilation during development:

```bash
cargo build
cargo run -- <command>
```

## Project Structure

- `src/main.rs` - CLI entry point and command handling
- `src/blockchain.rs` - Blockchain implementation
- `src/rdf_store.rs` - RDF store wrapper using Oxigraph
- `src/demo.rs` - Built-in demo with sample data
- `ontology/` - Traceability ontology files
- `queries/` - Sample SPARQL queries
- `tests/` - Test files

## Creating Your Own RDF Data

To create meaningful traceability data beyond the minimal test file, here's an example of a proper RDF file structure:

```turtle
@prefix ex: <http://example.org/> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

# Farm data
ex:milkBatch2 a ex:Milk ;
    prov:wasAttributedTo ex:FarmerMary ;
    prov:generatedAtTime "2025-08-09T08:00:00Z"^^xsd:dateTime ;
    ex:temperature "4.2"^^xsd:decimal ;
    ex:location "Farm Valley, Region A" .

# Processing data  
ex:productUHT2 a ex:UHTMilk ;
    prov:used ex:milkBatch2 ;
    prov:wasAttributedTo ex:UHTFactoryB ;
    prov:generatedAtTime "2025-08-09T14:30:00Z"^^xsd:dateTime ;
    ex:processingTemperature "135"^^xsd:decimal ;
    ex:batchSize "1000"^^xsd:integer .
```

Save this as `my_traceability_data.ttl` and add it to the blockchain:

```bash
cargo run -- add-file my_traceability_data.ttl
```

## Example Workflow

1. **Start with the demo** to understand the system:
   ```bash
   cargo run -- demo
   ```

2. **Add your own RDF data**:
   ```bash
   cargo run -- add-file your_data.ttl
   ```

3. **Query the data**:
   ```bash
   cargo run -- query queries/trace_by_batch.sparql
   ```

4. **Validate integrity**:
   ```bash
   cargo run -- validate
   ```

## Notes

- This is a proof-of-concept implementation
- The blockchain uses simple SHA-256 hashing
- RDF canonicalization is not implemented
- Data is stored in-memory only (not persisted between runs)
- Each block corresponds to a named graph in the RDF store

## Troubleshooting

### Build Issues

If you encounter build issues, ensure you have the latest stable Rust:

```bash
rustup update stable
```

### Missing Files

Make sure you're running commands from the project root directory where `Cargo.toml` is located.

### Query Errors

Ensure SPARQL query files use the correct prefixes:
```sparql
PREFIX ex: <http://example.org/>
PREFIX prov: <http://www.w3.org/ns/prov#>
