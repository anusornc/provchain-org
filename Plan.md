# Plan & Design

## Goal
Implement a minimal blockchain backed by RDF to store traceability triples. Provide SPARQL access to query provenance and traceability information.

## Tech choices
- Rust
- Oxigraph (in-memory RDF + SPARQL)
- sha2 + hex for hashing
- chrono for timestamps

## Steps
1. ✅ Create Rust project skeleton.
2. ✅ Implement Block and Blockchain data structures.
3. ✅ Insert triples as named graphs.
4. ✅ Compute block hash from deterministic serialization of graph quads.
5. ✅ Store block metadata as RDF.
6. ✅ Add CLI commands for add-block, validate, dump, query.
7. ✅ Provide example dataset and SPARQL queries.
8. ✅ Implement RDF canonicalization algorithm for consistent hashing.

## RDF Canonicalization Algorithm

To address the canonicalization limitation, implement the following hash-based canonicalization algorithm:

### Hash Function for Triples

```rust
function Hash(triple):
    subject = subject of triple
    predicate = predicate of triple
    object = object of triple

    # Serialize subject
    if subject is BNode:
        serialisation_subject = "Magic_S"
    else:
        serialisation_subject = NTriples(subject)

    # Serialize object
    if object is BNode:
        serialisation_object = "Magic_O"
    else:
        serialisation_object = NTriples(object)

    # Serialize predicate (always with NTriples)
    serialisation_predicate = NTriples(predicate)

    # Concatenate and hash
    concatenation = Concatenate(serialisation_subject, serialisation_predicate, serialisation_object)
    return SHA-256(concatenation)
```

### Main Canonicalization Loop

```rust
# Main loop over graph
for triple in graph:
    basic_triple_hash = Hash(triple)

    subject1 = subject of triple
    predicate1 = predicate of triple
    object1 = object of triple

    # If subject is a blank node, hash all triples where it appears as object
    if subject1 is BNode:
        for triple2 in graph where subject1 == object of triple2:
            hash2 = Hash(triple2)
            add hash2 to total_hash

    # If object is a blank node, hash all triples where it appears as subject
    if object1 is BNode:
        for triple3 in graph where object1 == subject of triple3:
            hash3 = Hash(triple3)
            add hash3 to total_hash
```

### Implementation Notes

- Use "Magic_S" and "Magic_O" as placeholder strings for blank nodes in subject and object positions respectively
- This ensures consistent hashing regardless of blank node identifiers
- The algorithm handles blank node relationships by including connected triples in the hash calculation
- Use SHA-256 for cryptographic security
- NTriples serialization provides canonical string representation for non-blank nodes

## Limitations
- Single-writer chain — no consensus or signatures yet.
- ~~RDF canonicalization missing (URDNA2015)~~ ✅ **RESOLVED**: Custom canonicalization algorithm implemented (not URDNA2015 standard) — sufficient for proof-of-concept but may need standardization for interoperability.

## Implementation Status
- **Core Features**: ✅ Complete
- **RDF Canonicalization**: ✅ Complete  
- **Testing**: ✅ Complete (10 tests passing)
- **CLI Interface**: ✅ Complete
- **Demo & Queries**: ✅ Complete
