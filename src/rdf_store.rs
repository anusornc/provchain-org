use oxigraph::io::RdfFormat;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;

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
        // Try to parse as RDF, if it fails, treat as plain text and create a simple triple
        match self.store.load_graph(rdf_data.as_bytes(), RdfFormat::Turtle, graph_name.clone(), None) {
            Ok(_) => {}, // Successfully parsed as RDF
            Err(_) => {
                // If parsing fails, create a simple triple with the data as a literal
                let subject = NamedNode::new(format!("http://example.org/data/{}", graph_name.as_str().replace("http://example.org/block/", ""))).unwrap();
                let predicate = NamedNode::new("http://example.org/hasData").unwrap();
                let object = Literal::new_simple_literal(rdf_data);
                let quad = Quad::new(subject, predicate, object, graph_name.clone());
                self.store.insert(&quad).unwrap();
            }
        }
    }

    pub fn load_ontology(&mut self, ontology_data: &str, graph_name: &NamedNode) {
        self.store
            .load_graph(ontology_data.as_bytes(), RdfFormat::Turtle, graph_name.clone(), None)
            .unwrap();
    }

    pub fn add_block_metadata(&mut self, block: &Block) {
        let graph_name = NamedNode::new("http://example.org/blockchain").unwrap();
        let block_uri = NamedNode::new(format!("http://example.org/block/{}", block.index)).unwrap();
        let prev_block_uri = if block.index > 0 {
            Some(NamedNode::new(format!("http://example.org/block/{}", block.index - 1)).unwrap())
        } else {
            None
        };

        let mut quads = vec![
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
                NamedNode::new("http://example.org/Block").unwrap(),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://example.org/index").unwrap(),
                Literal::new_typed_literal(
                    block.index.to_string(),
                    NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"),
                ),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://example.org/timestamp").unwrap(),
                Literal::new_typed_literal(
                    block.timestamp.clone(),
                    NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#dateTime"),
                ),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://example.org/hash").unwrap(),
                Literal::new_simple_literal(block.hash.clone()),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://example.org/previous_hash").unwrap(),
                Literal::new_simple_literal(block.previous_hash.clone()),
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

    pub fn dump_to_string(&self) -> String {
        let mut buffer = Vec::new();
        self.store.dump_to_writer(RdfFormat::NQuads, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}
