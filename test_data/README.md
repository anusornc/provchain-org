# ProvChain Test Data

This directory contains RDF test data files for the ProvChain blockchain system.

## Test Files

### `minimal_test_data.ttl`
- **Purpose**: Basic test file with minimal RDF content
- **Content**: Simple namespace declarations and one test triple
- **Use case**: Basic RDF parsing and storage testing
- **Example usage**:
  ```rust
  let turtle_data = fs::read_to_string("test_data/minimal_test_data.ttl")?;
  store.add_rdf_to_graph(&turtle_data, &graph_name);
  ```

### `simple_supply_chain_test.ttl`
- **Purpose**: Basic supply chain traceability example (recommended for blockchain testing)
- **Content**: Simple milk supply chain trace without complex structures
- **Features**:
  - Farmer entities and milk batches
  - UHT processing activities
  - Quality control checks
  - Proper PROV-O relationships
  - No blank nodes (blockchain-compatible)
- **Use case**: 
  - Blockchain integration testing
  - SPARQL query testing
  - Reliable workflow demonstration
- **Example usage**:
  ```rust
  let turtle_data = fs::read_to_string("test_data/simple_supply_chain_test.ttl")?;
  blockchain.add_block(turtle_data);
  ```

### `complete_supply_chain_test.ttl`
- **Purpose**: Comprehensive supply chain traceability example
- **Content**: Complete milk supply chain trace from farm to transport
- **Features**:
  - Farmer entities with geolocation (includes blank nodes)
  - UHT processing activities
  - Environmental conditions monitoring
  - Quality control checks
  - Transport activities
  - Proper PROV-O relationships
- **Use case**: 
  - RDF store testing (not recommended for blockchain due to blank nodes)
  - Complex SPARQL query testing
  - Ontology demonstration
- **Example usage**:
  ```rust
  let turtle_data = fs::read_to_string("test_data/complete_supply_chain_test.ttl")?;
  store.add_rdf_to_graph(&turtle_data, &graph_name);
  ```

## How to Use

1. **For unit tests**: Use `minimal_test_data.ttl` for basic RDF functionality testing
2. **For blockchain integration tests**: Use `simple_supply_chain_test.ttl` for reliable blockchain testing
3. **For RDF store tests**: Use `complete_supply_chain_test.ttl` for complex RDF functionality testing
4. **For SPARQL queries**: All files can be used with queries in the `queries/` directory
5. **For blockchain testing**: Use `minimal_test_data.ttl` or `simple_supply_chain_test.ttl` as block data

## Recommendations

- **Blockchain testing**: Use `simple_supply_chain_test.ttl` - it's designed to work reliably with the blockchain validation system
- **RDF functionality testing**: Use `complete_supply_chain_test.ttl` for comprehensive RDF features
- **Basic testing**: Use `minimal_test_data.ttl` for simple validation

## Ontology Alignment

Both test files use:
- **ProvChain namespace**: `http://provchain.org/`
- **PROV-O ontology**: `http://www.w3.org/ns/prov#`
- **Traceability ontology**: Defined in `ontology/traceability.owl.ttl`

The complete test file demonstrates proper usage of all ontology classes and properties for supply chain traceability.
