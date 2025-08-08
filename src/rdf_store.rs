use oxigraph::MemoryStore;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;

pub struct RDFStore {
    pub store: MemoryStore,
}

impl RDFStore {
    pub fn new() -> Self {
        RDFStore {
            store: MemoryStore::new(),
        }
    }

    pub fn add_rdf(&mut self, rdf_data: &str) {
        self.store
            .load_graph(
                rdf_data.as_bytes(),
                GraphFormat::Turtle,
                &GraphName::DefaultGraph,
                None,
            )
            .unwrap();
    }

    pub fn query(&self, sparql: &str) -> QueryResults {
        self.store.query(sparql).unwrap()
    }
}
