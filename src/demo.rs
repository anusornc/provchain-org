use crate::blockchain::Blockchain;
use crate::rdf_store::RDFStore;

pub fn run_demo() {
    let mut bc = Blockchain::new();
    let mut rdf_store = RDFStore::new();

    // Farmer RDF
    let farmer_data = r#"
        @prefix ex: <http://example.org/> .
        @prefix prov: <http://www.w3.org/ns/prov#> .

        ex:milkBatch1 a ex:Milk ;
            prov:wasAttributedTo ex:FarmerJohn ;
            prov:generatedAtTime "2025-08-08T10:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
    "#;
    bc.add_block(farmer_data.into());
    rdf_store.add_rdf(farmer_data);

    // Manufacturer RDF
    let manufacturer_data = r#"
        @prefix ex: <http://example.org/> .
        @prefix prov: <http://www.w3.org/ns/prov#> .

        ex:productUHT1 a ex:UHTMilk ;
            prov:used ex:milkBatch1 ;
            prov:wasAttributedTo ex:UHTFactoryA ;
            prov:generatedAtTime "2025-08-08T12:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
    "#;
    bc.add_block(manufacturer_data.into());
    rdf_store.add_rdf(manufacturer_data);

    println!("Blockchain valid? {}", bc.is_valid());

    let query = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";
    if let oxigraph::sparql::QueryResults::Solutions(solutions) = rdf_store.query(query) {
        for solution in solutions {
            println!("{:?}", solution.unwrap());
        }
    }
}
