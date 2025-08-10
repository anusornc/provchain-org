//! Phase 3: Knowledge Graph & Advanced Analytics Test Suite
//! 
//! This test suite validates the knowledge graph construction, entity linking,
//! and advanced analytics capabilities implemented in Phase 3.

use provchain_org::knowledge_graph::{
    KnowledgeGraph, KnowledgeEntity, KnowledgeRelationship,
    entity_linking::EntityLinker, 
    graph_db::GraphDatabase
};
use provchain_org::analytics::{
    AnalyticsEngine, supply_chain::SupplyChainAnalyzer, 
    sustainability::SustainabilityTracker, predictive::PredictiveAnalyzer
};
use provchain_org::rdf_store::RDFStore;
use provchain_org::blockchain::Blockchain;
use std::collections::HashMap;
use anyhow::Result;
use chrono::Utc;
use petgraph::Graph;

#[tokio::test]
async fn test_knowledge_graph_basic_operations() -> Result<()> {
    // Create test knowledge graph
    let knowledge_graph = create_test_knowledge_graph();
    
    // Validate knowledge graph structure
    assert!(!knowledge_graph.entities.is_empty(), "Knowledge graph should contain entities");
    assert!(!knowledge_graph.relationships.is_empty(), "Knowledge graph should contain relationships");
    
    // Check for expected entity types
    let entity_types: Vec<String> = knowledge_graph.entities.values()
        .map(|e| e.entity_type.clone())
        .collect();
    
    assert!(entity_types.contains(&"ProductBatch".to_string()));
    assert!(entity_types.contains(&"Farmer".to_string()));
    assert!(entity_types.contains(&"ProcessingActivity".to_string()));
    
    println!("✓ Knowledge graph basic operations test passed");
    Ok(())
}

#[tokio::test]
async fn test_entity_linking_and_resolution() -> Result<()> {
    // Create test knowledge graph with duplicate entities
    let mut knowledge_graph = create_test_knowledge_graph_with_duplicates();
    
    // Initialize entity linker
    let entity_linker = EntityLinker::new();
    
    // Perform entity resolution
    let resolution_report = entity_linker.resolve_entities(&mut knowledge_graph)?;
    
    // Validate resolution results
    assert!(!resolution_report.merged_entities.is_empty(), "Should have merged duplicate entities");
    assert!(resolution_report.enriched_entities > 0, "Should have enriched entities with external data");
    
    // Check that duplicates were properly merged
    let farmer_entities: Vec<_> = knowledge_graph.entities.values()
        .filter(|e| e.entity_type == "Farmer")
        .collect();
    
    // Should have fewer farmer entities after deduplication
    assert!(farmer_entities.len() < 3, "Duplicate farmers should be merged");
    
    println!("✓ Entity linking and resolution test passed");
    Ok(())
}

#[tokio::test]
async fn test_graph_database_operations() -> Result<()> {
    // Create test knowledge graph
    let knowledge_graph = create_test_knowledge_graph();
    
    // Initialize graph database
    let graph_db = GraphDatabase::new(knowledge_graph);
    
    // Test basic graph operations
    assert!(!graph_db.get_entities().is_empty(), "Should have entities");
    assert!(!graph_db.get_relationships().is_empty(), "Should have relationships");
    
    println!("✓ Graph database operations test passed");
    Ok(())
}

#[tokio::test]
async fn test_supply_chain_analytics() -> Result<()> {
    // Create test knowledge graph
    let knowledge_graph = create_test_knowledge_graph();
    
    // Initialize supply chain analyzer
    let analyzer = SupplyChainAnalyzer::new(&knowledge_graph);
    
    // Test comprehensive metrics calculation
    let metrics = analyzer.calculate_metrics()?;
    
    // Validate risk assessment
    assert!(metrics.risk_assessment.overall_risk_score >= 0.0);
    assert!(metrics.risk_assessment.overall_risk_score <= 1.0);
    assert!(!metrics.risk_assessment.risk_factors.is_empty());
    
    // Validate supplier performance
    assert!(!metrics.supplier_performance.is_empty());
    for supplier in &metrics.supplier_performance {
        assert!(supplier.overall_score >= 0.0 && supplier.overall_score <= 1.0);
    }
    
    // Validate quality metrics
    assert!(metrics.quality_metrics.quality_pass_rate >= 0.0);
    assert!(metrics.quality_metrics.quality_pass_rate <= 1.0);
    
    // Validate compliance status
    assert!(metrics.compliance_status.overall_compliance_rate >= 0.0);
    assert!(metrics.compliance_status.overall_compliance_rate <= 1.0);
    
    // Validate traceability coverage
    assert!(metrics.traceability_coverage.overall_coverage_percentage >= 0.0);
    assert!(metrics.traceability_coverage.overall_coverage_percentage <= 1.0);
    
    // Test batch-specific risk assessment
    let batch_risk = analyzer.assess_batch_risk("batch_001")?;
    assert!(batch_risk.overall_risk_score >= 0.0);
    assert!(batch_risk.overall_risk_score <= 1.0);
    
    println!("✓ Supply chain analytics test passed");
    Ok(())
}

#[tokio::test]
async fn test_sustainability_tracking() -> Result<()> {
    // Create test knowledge graph
    let knowledge_graph = create_test_knowledge_graph();
    
    // Initialize sustainability tracker
    let tracker = SustainabilityTracker::new(&knowledge_graph);
    
    // Test comprehensive sustainability metrics
    let metrics = tracker.calculate_metrics()?;
    
    // Validate carbon footprint
    assert!(metrics.carbon_footprint.total_co2_equivalent_kg >= 0.0);
    assert!(!metrics.carbon_footprint.emissions_by_source.is_empty());
    
    // Validate environmental impact
    assert!(metrics.environmental_impact.biodiversity_impact.impact_score >= 0.0);
    assert!(metrics.environmental_impact.biodiversity_impact.impact_score <= 1.0);
    
    // Validate ESG score
    assert!(metrics.esg_score.overall_score >= 0.0);
    assert!(metrics.esg_score.overall_score <= 1.0);
    assert!(metrics.esg_score.environmental_score >= 0.0);
    assert!(metrics.esg_score.social_score >= 0.0);
    assert!(metrics.esg_score.governance_score >= 0.0);
    
    // Validate certifications
    assert!(!metrics.sustainability_certifications.is_empty());
    
    // Validate renewable energy metrics
    assert!(metrics.renewable_energy_usage.renewable_percentage >= 0.0);
    assert!(metrics.renewable_energy_usage.renewable_percentage <= 100.0);
    
    // Test batch-specific carbon footprint
    let batch_footprint = tracker.calculate_batch_carbon_footprint("batch_001")?;
    assert!(batch_footprint.total_co2_equivalent_kg >= 0.0);
    
    println!("✓ Sustainability tracking test passed");
    Ok(())
}

#[tokio::test]
async fn test_predictive_analytics() -> Result<()> {
    // Create test knowledge graph
    let knowledge_graph = create_test_knowledge_graph();
    
    // Initialize predictive analyzer
    let analyzer = PredictiveAnalyzer::new(&knowledge_graph);
    
    // Test comprehensive predictive insights
    let insights = analyzer.generate_insights()?;
    
    // Validate demand forecast
    assert!(!insights.demand_forecast.forecast_points.is_empty());
    assert!(insights.demand_forecast.forecast_accuracy >= 0.0);
    assert!(insights.demand_forecast.forecast_accuracy <= 1.0);
    
    // Validate quality predictions
    for prediction in &insights.quality_predictions {
        assert!(prediction.probability >= 0.0 && prediction.probability <= 1.0);
        assert!(prediction.confidence_score >= 0.0 && prediction.confidence_score <= 1.0);
    }
    
    // Validate risk predictions
    for risk in &insights.risk_predictions {
        assert!(risk.probability >= 0.0 && risk.probability <= 1.0);
        assert!(risk.confidence_score >= 0.0 && risk.confidence_score <= 1.0);
    }
    
    // Validate optimization recommendations
    assert!(!insights.optimization_recommendations.is_empty());
    for recommendation in &insights.optimization_recommendations {
        assert!(recommendation.estimated_savings >= 0.0);
    }
    
    // Validate market trends
    assert!(!insights.market_trends.is_empty());
    for trend in &insights.market_trends {
        assert!(trend.strength >= 0.0 && trend.strength <= 1.0);
        assert!(trend.confidence >= 0.0 && trend.confidence <= 1.0);
    }
    
    // Test specific demand forecasting
    let demand_forecast = analyzer.forecast_demand(30)?;
    assert_eq!(demand_forecast.forecast_period_days, 30);
    assert_eq!(demand_forecast.forecast_points.len(), 30);
    
    println!("✓ Predictive analytics test passed");
    Ok(())
}

#[tokio::test]
async fn test_analytics_engine_integration() -> Result<()> {
    // Create test knowledge graph
    let knowledge_graph = create_test_knowledge_graph();
    let rdf_store = RDFStore::new();
    
    // Initialize analytics engine
    let analytics_engine = AnalyticsEngine::new(knowledge_graph, rdf_store);
    
    // Test basic analytics functionality
    assert!(analytics_engine.get_knowledge_graph().entities.len() > 0);
    
    println!("✓ Analytics engine integration test passed");
    Ok(())
}

#[tokio::test]
async fn test_knowledge_graph_querying() -> Result<()> {
    // Create test knowledge graph
    let knowledge_graph = create_test_knowledge_graph();
    
    // Test entity queries
    let farmers: Vec<_> = knowledge_graph.entities.values()
        .filter(|e| e.entity_type == "Farmer")
        .collect();
    assert!(!farmers.is_empty(), "Should find farmer entities");
    
    // Test relationship queries
    let processing_relationships: Vec<_> = knowledge_graph.relationships.iter()
        .filter(|r| r.predicate == "processedBatch")
        .collect();
    assert!(!processing_relationships.is_empty(), "Should find processing relationships");
    
    // Test property-based queries
    let entities_with_location: Vec<_> = knowledge_graph.entities.values()
        .filter(|e| e.properties.contains_key("location"))
        .collect();
    assert!(!entities_with_location.is_empty(), "Should find entities with location");
    
    println!("✓ Knowledge graph querying test passed");
    Ok(())
}

#[tokio::test]
async fn test_performance_benchmarks() -> Result<()> {
    // Create larger test knowledge graph for performance testing
    let knowledge_graph = create_large_test_knowledge_graph();
    
    // Benchmark knowledge graph operations
    let start = std::time::Instant::now();
    let _graph_db = GraphDatabase::new(knowledge_graph.clone());
    let graph_construction_time = start.elapsed();
    
    // Benchmark analytics operations
    let start = std::time::Instant::now();
    let analyzer = SupplyChainAnalyzer::new(&knowledge_graph);
    let _metrics = analyzer.calculate_metrics()?;
    let analytics_time = start.elapsed();
    
    // Benchmark entity linking
    let start = std::time::Instant::now();
    let mut kg_copy = knowledge_graph.clone();
    let entity_linker = EntityLinker::new();
    let _resolution = entity_linker.resolve_entities(&mut kg_copy)?;
    let entity_linking_time = start.elapsed();
    
    // Performance assertions (adjust thresholds as needed)
    assert!(graph_construction_time.as_millis() < 1000, "Graph construction should be fast");
    assert!(analytics_time.as_millis() < 2000, "Analytics should be reasonably fast");
    assert!(entity_linking_time.as_millis() < 3000, "Entity linking should complete in reasonable time");
    
    println!("✓ Performance benchmarks test passed");
    println!("  Graph construction: {:?}", graph_construction_time);
    println!("  Analytics: {:?}", analytics_time);
    println!("  Entity linking: {:?}", entity_linking_time);
    
    Ok(())
}

// Helper functions for creating test data

fn create_test_knowledge_graph() -> KnowledgeGraph {
    let mut entities = HashMap::new();
    let mut relationships = Vec::new();
    
    // Create test entities
    entities.insert("farmer001".to_string(), KnowledgeEntity {
        uri: "farmer001".to_string(),
        entity_type: "Farmer".to_string(),
        label: Some("Green Valley Farm".to_string()),
        properties: {
            let mut props = HashMap::new();
            props.insert("location".to_string(), "California, USA".to_string());
            props.insert("certifiedOrganic".to_string(), "true".to_string());
            props
        },
        confidence_score: 0.95,
    });
    
    entities.insert("batch001".to_string(), KnowledgeEntity {
        uri: "batch001".to_string(),
        entity_type: "ProductBatch".to_string(),
        label: Some("Organic Tomatoes Batch".to_string()),
        properties: {
            let mut props = HashMap::new();
            props.insert("batchId".to_string(), "batch_001".to_string());
            props.insert("product".to_string(), "Organic Tomatoes".to_string());
            props.insert("quantity".to_string(), "1000".to_string());
            props
        },
        confidence_score: 0.98,
    });
    
    entities.insert("processing001".to_string(), KnowledgeEntity {
        uri: "processing001".to_string(),
        entity_type: "ProcessingActivity".to_string(),
        label: Some("Tomato Processing".to_string()),
        properties: {
            let mut props = HashMap::new();
            props.insert("facility".to_string(), "Fresh Processing Co".to_string());
            props.insert("recordedAt".to_string(), "2024-01-15T10:00:00Z".to_string());
            props
        },
        confidence_score: 0.92,
    });
    
    entities.insert("quality001".to_string(), KnowledgeEntity {
        uri: "quality001".to_string(),
        entity_type: "QualityCheck".to_string(),
        label: Some("Pesticide Test".to_string()),
        properties: {
            let mut props = HashMap::new();
            props.insert("testType".to_string(), "Pesticide Residue".to_string());
            props.insert("result".to_string(), "Pass".to_string());
            props
        },
        confidence_score: 0.99,
    });
    
    entities.insert("cert001".to_string(), KnowledgeEntity {
        uri: "cert001".to_string(),
        entity_type: "Certificate".to_string(),
        label: Some("Organic Certification".to_string()),
        properties: {
            let mut props = HashMap::new();
            props.insert("certificationType".to_string(), "Organic".to_string());
            props.insert("issuedBy".to_string(), "USDA Organic".to_string());
            props
        },
        confidence_score: 0.97,
    });
    
    // Create test relationships
    relationships.push(KnowledgeRelationship {
        subject: "batch001".to_string(),
        predicate: "harvestedBy".to_string(),
        object: "farmer001".to_string(),
        confidence_score: 0.95,
        temporal_info: None,
    });
    
    relationships.push(KnowledgeRelationship {
        subject: "processing001".to_string(),
        predicate: "processedBatch".to_string(),
        object: "batch001".to_string(),
        confidence_score: 0.93,
        temporal_info: None,
    });
    
    relationships.push(KnowledgeRelationship {
        subject: "quality001".to_string(),
        predicate: "checkedBatch".to_string(),
        object: "batch001".to_string(),
        confidence_score: 0.98,
        temporal_info: None,
    });
    
    relationships.push(KnowledgeRelationship {
        subject: "cert001".to_string(),
        predicate: "certifies".to_string(),
        object: "farmer001".to_string(),
        confidence_score: 0.96,
        temporal_info: None,
    });
    
    KnowledgeGraph {
        entities,
        relationships,
        entity_index: HashMap::new(),
        graph: Graph::new(),
    }
}

fn create_test_knowledge_graph_with_duplicates() -> KnowledgeGraph {
    let mut kg = create_test_knowledge_graph();
    
    // Add duplicate farmer with slightly different name
    kg.entities.insert("farmer002".to_string(), KnowledgeEntity {
        uri: "farmer002".to_string(),
        entity_type: "Farmer".to_string(),
        label: Some("Green Valley Farms".to_string()), // Slightly different
        properties: {
            let mut props = HashMap::new();
            props.insert("location".to_string(), "California, USA".to_string());
            props.insert("certifiedOrganic".to_string(), "true".to_string());
            props
        },
        confidence_score: 0.90,
    });
    
    // Add another similar farmer
    kg.entities.insert("farmer003".to_string(), KnowledgeEntity {
        uri: "farmer003".to_string(),
        entity_type: "Farmer".to_string(),
        label: Some("Green Valley Farm Inc".to_string()), // Similar name
        properties: {
            let mut props = HashMap::new();
            props.insert("location".to_string(), "CA, USA".to_string()); // Similar location
            props.insert("certifiedOrganic".to_string(), "true".to_string());
            props
        },
        confidence_score: 0.88,
    });
    
    kg
}

fn create_large_test_knowledge_graph() -> KnowledgeGraph {
    let mut entities = HashMap::new();
    let mut relationships = Vec::new();
    
    // Create 100 test entities of various types
    for i in 0..100 {
        let entity_type = match i % 5 {
            0 => "Farmer",
            1 => "ProductBatch",
            2 => "ProcessingActivity",
            3 => "QualityCheck",
            _ => "Certificate",
        };
        
        entities.insert(format!("entity_{:03}", i), KnowledgeEntity {
            uri: format!("entity_{:03}", i),
            entity_type: entity_type.to_string(),
            label: Some(format!("{} {}", entity_type, i)),
            properties: {
                let mut props = HashMap::new();
                props.insert("id".to_string(), i.to_string());
                props.insert("type".to_string(), entity_type.to_string());
                props
            },
            confidence_score: 0.8 + ((i % 20) as f64) / 100.0,
        });
        
        // Create relationships between consecutive entities
        if i > 0 {
            relationships.push(KnowledgeRelationship {
                subject: format!("entity_{:03}", i),
                predicate: "relatedTo".to_string(),
                object: format!("entity_{:03}", i - 1),
                confidence_score: 0.85,
                temporal_info: None,
            });
        }
    }
    
    KnowledgeGraph {
        entities,
        relationships,
        entity_index: HashMap::new(),
        graph: Graph::new(),
    }
}
