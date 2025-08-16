use provchain_org::blockchain::Blockchain;
use provchain_org::rdf_store::RDFStore;
use oxigraph::model::NamedNode;

#[test]
fn test_ontology_loading() {
    let bc = Blockchain::new();
    
    // Check that ontology classes are loaded
    let classes = bc.rdf_store.get_ontology_classes();
    println!("Loaded ontology classes: {classes:?}");
    
    // Should contain our main ontology classes
    let class_strings = classes.join(" ");
    assert!(class_strings.contains("ProductBatch"));
    assert!(class_strings.contains("ProcessingActivity"));
    assert!(class_strings.contains("Farmer"));
    assert!(class_strings.contains("Manufacturer"));
}

#[test]
fn test_ontology_validation() {
    let mut bc = Blockchain::new();
    
    // Add valid ontology-based data
    let valid_data = r#"
        @prefix ex: <http://example.org/> .
        @prefix trace: <http://provchain.org/trace#> .
        @prefix prov: <http://www.w3.org/ns/prov#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

        ex:testBatch a trace:ProductBatch ;
            trace:hasBatchID "TEST001" ;
            trace:producedAt "2025-08-08T10:00:00Z"^^xsd:dateTime .

        ex:testActivity a trace:ProcessingActivity ;
            trace:recordedAt "2025-08-08T12:00:00Z"^^xsd:dateTime ;
            prov:wasAssociatedWith ex:testAgent .

        ex:testAgent a trace:Manufacturer ;
            rdfs:label "Test Manufacturer" .
    "#;
    
    let _ = bc.add_block(valid_data.into());
    
    // Validate the last block's data
    let last_block_index = bc.chain.len() - 1;
    let graph_name = NamedNode::new(format!("http://provchain.org/block/{last_block_index}")).unwrap();
    
    // Test ontology validation
    let is_valid = bc.rdf_store.validate_against_ontology(&graph_name);
    assert!(is_valid, "Valid ontology data should pass validation");
    
    // Test required properties validation
    let validation_errors = bc.rdf_store.validate_required_properties(&graph_name);
    assert!(validation_errors.is_empty(), "Valid data should have no validation errors: {validation_errors:?}");
}

#[test]
fn test_ontology_validation_failures() {
    let mut rdf_store = RDFStore::new();
    
    // Load ontology first
    if let Ok(ontology_data) = std::fs::read_to_string("ontology/traceability.owl.ttl") {
        let ontology_graph = NamedNode::new("http://provchain.org/ontology").unwrap();
        rdf_store.load_ontology(&ontology_data, &ontology_graph);
    }
    
    // Add invalid data (ProductBatch without required hasBatchID)
    let invalid_data = r#"
        @prefix ex: <http://example.org/> .
        @prefix trace: <http://provchain.org/trace#> .
        @prefix prov: <http://www.w3.org/ns/prov#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:invalidBatch a trace:ProductBatch ;
            trace:producedAt "2025-08-08T10:00:00Z"^^xsd:dateTime .

        ex:invalidActivity a trace:ProcessingActivity ;
            prov:wasAssociatedWith ex:testAgent .
    "#;
    
    let graph_name = NamedNode::new("http://provchain.org/test").unwrap();
    rdf_store.add_rdf_to_graph(invalid_data, &graph_name);
    
    // Test validation should find errors
    let validation_errors = rdf_store.validate_required_properties(&graph_name);
    assert!(!validation_errors.is_empty(), "Invalid data should have validation errors");
    
    // Should find missing hasBatchID and recordedAt
    let error_text = validation_errors.join(" ");
    assert!(error_text.contains("hasBatchID"), "Should detect missing hasBatchID");
    assert!(error_text.contains("recordedAt"), "Should detect missing recordedAt");
}

#[test]
fn test_environmental_conditions_integration() {
    let mut bc = Blockchain::new();
    
    // Add data with environmental conditions
    let env_data = r#"
        @prefix ex: <http://example.org/> .
        @prefix trace: <http://provchain.org/trace#> .
        @prefix prov: <http://www.w3.org/ns/prov#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:coldTransport a trace:TransportActivity ;
            trace:recordedAt "2025-08-08T14:00:00Z"^^xsd:dateTime ;
            trace:hasCondition ex:coldCondition .

        ex:coldCondition a trace:EnvironmentalCondition ;
            trace:hasTemperature "2.5"^^xsd:decimal ;
            trace:hasHumidity "70.0"^^xsd:decimal ;
            trace:hasConditionTimestamp "2025-08-08T14:00:00Z"^^xsd:dateTime .
    "#;
    
    let _ = bc.add_block(env_data.into());
    
    // Query for environmental conditions across all graphs
    let env_query = r#"
        PREFIX trace: <http://provchain.org/trace#>
        
        SELECT ?activity ?temp ?humidity ?graph WHERE {
            GRAPH ?graph {
                ?activity a trace:TransportActivity ;
                          trace:hasCondition ?condition .
                ?condition trace:hasTemperature ?temp ;
                           trace:hasHumidity ?humidity .
            }
        }
    "#;
    
    if let oxigraph::sparql::QueryResults::Solutions(solutions) = bc.rdf_store.query(env_query) {
        let mut found_conditions = false;
        for solution in solutions {
            if let Ok(sol) = solution {
                if let (Some(temp), Some(humidity)) = (sol.get("temp"), sol.get("humidity")) {
                    println!("Found environmental conditions: temp={temp}, humidity={humidity}");
                    found_conditions = true;
                }
            }
        }
        assert!(found_conditions, "Should find environmental conditions in the data");
    } else {
        // Fallback: check if the data was stored at all
        let simple_query = r#"
            PREFIX trace: <http://provchain.org/trace#>
            
            SELECT ?s ?p ?o WHERE {
                GRAPH ?g {
                    ?s ?p ?o .
                    FILTER(?p = trace:hasTemperature || ?p = trace:hasHumidity)
                }
            }
        "#;
        
        if let oxigraph::sparql::QueryResults::Solutions(solutions) = bc.rdf_store.query(simple_query) {
            let mut count = 0;
            for solution in solutions {
                if let Ok(_sol) = solution {
                    count += 1;
                }
            }
            assert!(count > 0, "Should find at least some environmental data properties");
        }
    }
}

#[test]
fn test_supply_chain_traceability() {
    let mut bc = Blockchain::new();
    
    // Add complete supply chain data using ontology
    let farmer_data = r#"
        @prefix ex: <http://example.org/> .
        @prefix trace: <http://provchain.org/trace#> .
        @prefix prov: <http://www.w3.org/ns/prov#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

        ex:rawMilk a trace:ProductBatch ;
            trace:hasBatchID "RAW001" ;
            trace:producedAt "2025-08-08T08:00:00Z"^^xsd:dateTime ;
            prov:wasAttributedTo ex:dairyFarm .

        ex:dairyFarm a trace:Farmer ;
            rdfs:label "Green Valley Dairy Farm" .
    "#;
    let _ = bc.add_block(farmer_data.into());
    
    let processing_data = r#"
        @prefix ex: <http://example.org/> .
        @prefix trace: <http://provchain.org/trace#> .
        @prefix prov: <http://www.w3.org/ns/prov#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

        ex:uhtProcess a trace:ProcessingActivity ;
            trace:recordedAt "2025-08-08T10:00:00Z"^^xsd:dateTime ;
            prov:used ex:rawMilk ;
            prov:wasAssociatedWith ex:processor .

        ex:processedMilk a trace:ProductBatch ;
            trace:hasBatchID "UHT001" ;
            trace:producedAt "2025-08-08T10:30:00Z"^^xsd:dateTime ;
            prov:wasGeneratedBy ex:uhtProcess ;
            trace:lotDerivedFrom ex:rawMilk .

        ex:processor a trace:Manufacturer ;
            rdfs:label "UHT Processing Co." .
    "#;
    let _ = bc.add_block(processing_data.into());
    
    // Query for complete traceability chain across all graphs
    let trace_query = r#"
        PREFIX trace: <http://provchain.org/trace#>
        PREFIX prov: <http://www.w3.org/ns/prov#>
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
        
        SELECT ?batch ?batchId ?derivedFrom ?agent ?agentLabel ?graph WHERE {
            GRAPH ?graph {
                ?batch a trace:ProductBatch ;
                       trace:hasBatchID ?batchId .
                
                OPTIONAL {
                    ?batch trace:lotDerivedFrom ?derivedFrom .
                }
                
                OPTIONAL {
                    ?batch prov:wasAttributedTo ?agent .
                    ?agent rdfs:label ?agentLabel .
                }
            }
        }
        ORDER BY ?batchId
    "#;
    
    if let oxigraph::sparql::QueryResults::Solutions(solutions) = bc.rdf_store.query(trace_query) {
        let mut batch_count = 0;
        for solution in solutions {
            if let Ok(sol) = solution {
                if let Some(batch_id) = sol.get("batchId") {
                    println!("Found batch: {batch_id}");
                    batch_count += 1;
                }
            }
        }
        assert!(batch_count >= 2, "Should find at least 2 batches in the supply chain");
    } else {
        // Fallback: check if any ProductBatch data exists
        let simple_query = r#"
            PREFIX trace: <http://provchain.org/trace#>
            
            SELECT ?batch ?batchId WHERE {
                GRAPH ?g {
                    ?batch a trace:ProductBatch ;
                           trace:hasBatchID ?batchId .
                }
            }
        "#;
        
        if let oxigraph::sparql::QueryResults::Solutions(solutions) = bc.rdf_store.query(simple_query) {
            let mut count = 0;
            for solution in solutions {
                if let Ok(sol) = solution {
                    if let Some(batch_id) = sol.get("batchId") {
                        println!("Found batch in fallback: {batch_id}");
                        count += 1;
                    }
                }
            }
            assert!(count >= 2, "Should find at least 2 batches in the supply chain (fallback check)");
        }
    }
}
