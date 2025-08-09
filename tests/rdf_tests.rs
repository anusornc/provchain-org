use uht_trace_blockchain::rdf_store::RDFStore;
use oxigraph::model::NamedNode;
use oxigraph::sparql::QueryResults;
use uht_trace_blockchain::blockchain::Block;

#[test]
fn test_rdf_insertion_and_query_in_named_graph() {
    let mut store = RDFStore::new();
    let turtle_data = r#"@prefix ex: <http://example.org/> .
        @prefix prov: <http://www.w3.org/ns/prov#> .

        ex:milkBatch1 a ex:Milk ;
            prov:wasAttributedTo ex:FarmerJohn .
    "#;
    let graph_name = NamedNode::new("http://example.org/test_graph").unwrap();
    store.add_rdf_to_graph(turtle_data, &graph_name);

    let query = r#"PREFIX ex: <http://example.org/>
        SELECT ?batch
        FROM <http://example.org/test_graph>
        WHERE {
            ?batch a ex:Milk .
        }
    "#;

    if let QueryResults::Solutions(solutions) = store.query(query) {
        let results: Vec<_> = solutions.collect();
        assert_eq!(results.len(), 1, "Should find exactly one milk batch in the named graph");
    } else {
        panic!("SPARQL query failed");
    }
}

#[test]
fn test_block_metadata_storage_and_query() {
    let mut store = RDFStore::new();
    let block = Block::new(1, "test data".into(), "some_hash".into());
    store.add_block_metadata(&block);

    let query = r#"PREFIX tc: <http://tracechain.org/>
        SELECT ?hash
        FROM <http://tracechain.org/blockchain>
        WHERE {
            <http://tracechain.org/block/1> tc:hasHash ?hash .
        }
    "#;

    if let QueryResults::Solutions(solutions) = store.query(query) {
        let results: Vec<_> = solutions.collect();
        assert_eq!(results.len(), 1, "Should find the block's hash");
        let solution = results[0].as_ref().unwrap();
        assert_eq!(solution.get("hash").unwrap().to_string(), format!("\"{}\"", block.hash));
    } else {
        panic!("SPARQL query failed");
    }
}
