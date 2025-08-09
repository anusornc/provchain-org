use oxigraph::io::RdfFormat;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;
use std::io::Cursor;
use sha2::{Sha256, Digest};
use std::collections::HashSet;

use crate::blockchain::Block;

pub struct RDFStore {
    pub store: Store,
}

impl RDFStore {
    pub fn new() -> Self {
        RDFStore {
            store: Store::new().unwrap(),
        }
    }

    pub fn add_rdf_to_graph(&mut self, rdf_data: &str, graph_name: &NamedNode) {
        // Try to parse as RDF using a temporary store, if it fails, treat as plain text and create a simple triple
        let temp_store = Store::new().unwrap();
        let reader = Cursor::new(rdf_data.as_bytes());
        
        match temp_store.load_from_reader(RdfFormat::Turtle, reader) {
            Ok(_) => {
                // Successfully parsed as RDF, now copy all triples to the target graph
                for quad in temp_store.iter() {
                    if let Ok(original_quad) = quad {
                        // Create a new quad with the specified graph name
                        let new_quad = Quad::new(
                            original_quad.subject.clone(),
                            original_quad.predicate.clone(),
                            original_quad.object.clone(),
                            graph_name.clone()
                        );
                        self.store.insert(&new_quad).unwrap();
                    }
                }
            }
            Err(_) => {
                // If parsing fails, create a simple triple with the data as a literal
                let subject = NamedNode::new(format!("http://provchain.org/data/{}", graph_name.as_str().replace("http://provchain.org/block/", ""))).unwrap();
                let predicate = NamedNode::new("http://provchain.org/hasData").unwrap();
                let object = Literal::new_simple_literal(rdf_data);
                let quad = Quad::new(subject, predicate, object, graph_name.clone());
                self.store.insert(&quad).unwrap();
            }
        }
    }

    #[allow(dead_code)]
    pub fn load_ontology(&mut self, ontology_data: &str, _graph_name: &NamedNode) {
        let reader = Cursor::new(ontology_data.as_bytes());
        self.store
            .load_from_reader(RdfFormat::Turtle, reader)
            .unwrap();
    }

    pub fn add_block_metadata(&mut self, block: &Block) {
        let graph_name = NamedNode::new("http://provchain.org/blockchain").unwrap();
        let block_uri = NamedNode::new(format!("http://provchain.org/block/{}", block.index)).unwrap();
        let data_graph_uri = NamedNode::new(format!("http://provchain.org/block/{}", block.index)).unwrap();
        let prev_block_uri = if block.index > 0 {
            Some(NamedNode::new(format!("http://provchain.org/block/{}", block.index - 1)).unwrap())
        } else {
            None
        };

        // Determine block type (Genesis or regular Block)
        let block_type = if block.index == 0 {
            NamedNode::new("http://provchain.org/GenesisBlock").unwrap()
        } else {
            NamedNode::new("http://provchain.org/Block").unwrap()
        };

        let mut quads = vec![
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
                block_type,
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://provchain.org/hasIndex").unwrap(),
                Literal::new_typed_literal(
                    block.index.to_string(),
                    NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"),
                ),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://provchain.org/hasTimestamp").unwrap(),
                Literal::new_typed_literal(
                    block.timestamp.clone(),
                    NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#dateTime"),
                ),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://provchain.org/hasHash").unwrap(),
                Literal::new_simple_literal(block.hash.clone()),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://provchain.org/hasPreviousHash").unwrap(),
                Literal::new_simple_literal(block.previous_hash.clone()),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://provchain.org/hasDataGraphIRI").unwrap(),
                Literal::new_typed_literal(
                    data_graph_uri.as_str(),
                    NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#anyURI"),
                ),
                graph_name.clone(),
            ),
        ];

        if let Some(prev) = prev_block_uri {
            quads.push(Quad::new(
                block_uri,
                NamedNode::new("http://www.w3.org/ns/prov#wasPrecededBy").unwrap(),
                prev,
                graph_name,
            ));
        }

        for quad in &quads {
            self.store.insert(quad).unwrap();
        }
    }

    pub fn query(&self, sparql: &str) -> QueryResults {
        self.store.query(sparql).unwrap()
    }

    #[allow(dead_code)]
    pub fn dump_to_string(&self) -> String {
        let mut buffer = Vec::new();
        self.store.dump_to_writer(RdfFormat::NQuads, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    /// Hash a single triple using the canonicalization algorithm from Plan.md
    fn hash_triple(&self, triple: &Triple) -> String {
        // Serialize subject
        let serialisation_subject = match &triple.subject {
            Subject::BlankNode(_) => "Magic_S".to_string(),
            Subject::NamedNode(node) => node.to_string(),
            Subject::Triple(t) => format!("<<{}>>", self.triple_to_ntriples(t)),
        };

        // Serialize object
        let serialisation_object = match &triple.object {
            Term::BlankNode(_) => "Magic_O".to_string(),
            Term::NamedNode(node) => node.to_string(),
            Term::Literal(lit) => lit.to_string(),
            Term::Triple(t) => format!("<<{}>>", self.triple_to_ntriples(t)),
        };

        // Serialize predicate (always with NTriples)
        let serialisation_predicate = triple.predicate.to_string();

        // Concatenate and hash
        let concatenation = format!("{}{}{}", serialisation_subject, serialisation_predicate, serialisation_object);
        let mut hasher = Sha256::new();
        hasher.update(concatenation.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Convert a triple to NTriples format
    fn triple_to_ntriples(&self, triple: &Triple) -> String {
        format!("{} {} {}", 
            self.subject_to_ntriples(&triple.subject),
            triple.predicate.to_string(),
            self.term_to_ntriples(&triple.object)
        )
    }

    /// Convert a subject to NTriples format
    fn subject_to_ntriples(&self, subject: &Subject) -> String {
        match subject {
            Subject::NamedNode(node) => format!("<{}>", node.as_str()),
            Subject::BlankNode(node) => format!("_:{}", node.as_str()),
            Subject::Triple(t) => format!("<<{}>>", self.triple_to_ntriples(t)),
        }
    }

    /// Convert a term to NTriples format
    fn term_to_ntriples(&self, term: &Term) -> String {
        match term {
            Term::NamedNode(node) => format!("<{}>", node.as_str()),
            Term::BlankNode(node) => format!("_:{}", node.as_str()),
            Term::Literal(lit) => lit.to_string(),
            Term::Triple(t) => format!("<<{}>>", self.triple_to_ntriples(t)),
        }
    }

    /// Canonicalize and hash RDF data for a specific graph
    pub fn canonicalize_graph(&self, graph_name: &NamedNode) -> String {
        let mut total_hashes = HashSet::new();

        // Collect all triples in the specified graph
        let mut triples = Vec::new();
        for quad_result in self.store.quads_for_pattern(None, None, None, Some(graph_name.into())) {
            if let Ok(quad) = quad_result {
                let triple = Triple::new(
                    quad.subject.clone(),
                    quad.predicate.clone(),
                    quad.object.clone(),
                );
                triples.push(triple);
            }
        }

        // Main canonicalization loop from Plan.md
        for triple in &triples {
            let basic_triple_hash = self.hash_triple(triple);
            total_hashes.insert(basic_triple_hash);

            // If subject is a blank node, hash all triples where it appears as object
            if let Subject::BlankNode(subject_bnode) = &triple.subject {
                for triple2 in &triples {
                    if let Term::BlankNode(object_bnode) = &triple2.object {
                        if subject_bnode == object_bnode {
                            let hash2 = self.hash_triple(triple2);
                            total_hashes.insert(hash2);
                        }
                    }
                }
            }

            // If object is a blank node, hash all triples where it appears as subject
            if let Term::BlankNode(object_bnode) = &triple.object {
                for triple3 in &triples {
                    if let Subject::BlankNode(subject_bnode) = &triple3.subject {
                        if object_bnode == subject_bnode {
                            let hash3 = self.hash_triple(triple3);
                            total_hashes.insert(hash3);
                        }
                    }
                }
            }
        }

        // Combine all hashes into a final canonical hash
        let mut sorted_hashes: Vec<String> = total_hashes.into_iter().collect();
        sorted_hashes.sort();
        let combined = sorted_hashes.join("");
        
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Canonicalize and hash all RDF data in the store
    #[allow(dead_code)]
    pub fn canonicalize_all(&self) -> String {
        let mut all_graph_hashes = Vec::new();

        // Get all named graphs
        let mut graphs = HashSet::new();
        for quad_result in self.store.iter() {
            if let Ok(quad) = quad_result {
                if let GraphName::NamedNode(graph) = &quad.graph_name {
                    graphs.insert(graph.clone());
                }
            }
        }

        // Hash each graph and collect the results
        for graph in graphs {
            let graph_hash = self.canonicalize_graph(&graph);
            all_graph_hashes.push(format!("{}:{}", graph.as_str(), graph_hash));
        }

        // Sort and combine all graph hashes
        all_graph_hashes.sort();
        let combined = all_graph_hashes.join("|");
        
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
