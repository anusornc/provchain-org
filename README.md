# UHT Traceability Blockchain (PoC)

A Rust proof-of-concept implementing a simple blockchain where each block stores RDF triples in a named graph (Oxigraph). The ontology is a minimal extension of PROV-O for traceability, suitable for UHT manufacturing but general enough for other supply chains.

## Highlights
- In-memory RDF store (Oxigraph) used as the dataset.
- Each block corresponds to a named graph (graph URI `http://example.org/block/{index}`).
- Block metadata stored as RDF in `http://example.org/blockchain` graph.
- Simple SHA-256-based block hashing (note: RDF canonicalization not implemented — PoC only).

See `RUN.md` for quick start.
