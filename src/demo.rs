use crate::blockchain::Blockchain;
use std::fs;

pub fn run_demo() {
    let mut bc = Blockchain::new();

    // Farmer RDF
    let farmer_data = r#"///
        @prefix ex: <http://example.org/> .
        @prefix prov: <http://www.w3.org/ns/prov#> .

        ex:milkBatch1 a ex:Milk ;
            prov:wasAttributedTo ex:FarmerJohn ;
            prov:generatedAtTime "2025-08-08T10:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
    "#;
    bc.add_block(farmer_data.into());

    // Manufacturer RDF
    let manufacturer_data = r#"///
        @prefix ex: <http://example.org/> .
        @prefix prov: <http://www.w3.org/ns/prov#> .

        ex:productUHT1 a ex:UHTMilk ;
            prov:used ex:milkBatch1 ;
            prov:wasAttributedTo ex:UHTFactoryA ;
            prov:generatedAtTime "2025-08-08T12:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
    "#;
    bc.add_block(manufacturer_data.into());

    println!("Blockchain valid? {}", bc.is_valid());
    println!("\n--- Blockchain Dump ---");
    println!("{}", bc.dump());
    println!("\n--- Running Queries ---");


    // Run Step 4 queries if present in ./queries/
    let queries = vec![
        "trace_by_batch.sparql",
        "trace_origin.sparql",
        "env_conditions_for_batch.sparql",
        "blockchain_metadata.sparql",
    ];

    for qfile in queries {
        let path = format!("queries/{}", qfile);
        if let Ok(qtext) = fs::read_to_string(&path) {
            println!("\n=== Running query: {} ===", qfile);
            if let oxigraph::sparql::QueryResults::Solutions(solutions) = bc.rdf_store.query(&qtext) {
                for solution in solutions {
                    println!("{:?}", solution.unwrap());
                }
            }
        }
    }
}
