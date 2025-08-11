//! Debug test to check what data is actually stored in the RDF store

use provchain_org::{
    blockchain::Blockchain,
    web::server::create_web_server,
};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use reqwest::Client;
use anyhow::Result;

/// Test helper to start a test web server
async fn start_test_server() -> Result<(u16, tokio::task::JoinHandle<()>)> {
    use std::net::TcpListener;
    
    // Find an available port
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    drop(listener); // Release the port for the server to use
    
    let blockchain = Blockchain::new();
    let server = create_web_server(blockchain, Some(port)).await.map_err(|e| anyhow::anyhow!("Failed to create server: {}", e))?;
    
    let handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            eprintln!("Server error: {}", e);
        }
    });
    
    // Give server time to start
    sleep(Duration::from_millis(1000)).await;
    
    Ok((port, handle))
}

/// Test helper to authenticate and get token
async fn get_auth_token(client: &Client, base_url: &str, username: &str, password: &str) -> Result<String> {
    let auth_response = client
        .post(&format!("{}/auth/login", base_url))
        .json(&json!({
            "username": username,
            "password": password
        }))
        .send()
        .await?;
    
    let auth_data: serde_json::Value = auth_response.json().await?;
    Ok(auth_data["token"].as_str().unwrap_or("").to_string())
}

#[tokio::test]
async fn debug_trace_data_storage() -> Result<()> {
    let (port, _server_handle) = start_test_server().await?;
    let base_url = format!("http://localhost:{}", port);
    let client = Client::new();
    
    println!("Debug: Testing trace data storage on {}", base_url);
    
    let token = get_auth_token(&client, &base_url, "admin", "admin123").await?;
    
    // Add the exact same data as the test
    let supply_chain_data = vec![
        json!({
            "subject": "http://example.org/batch456",
            "predicate": "http://provchain.org/trace#hasBatchID",
            "object": "BATCH456",
            "graph_name": "traceability"
        }),
        json!({
            "subject": "http://example.org/batch456",
            "predicate": "http://provchain.org/trace#product",
            "object": "Premium Coffee",
            "graph_name": "traceability"
        }),
        json!({
            "subject": "http://example.org/batch456",
            "predicate": "http://provchain.org/trace#origin",
            "object": "Mountain Farm, Colombia",
            "graph_name": "traceability"
        }),
    ];
    
    // Ingest data
    for data in supply_chain_data {
        println!("Adding: {:?}", data);
        let response = client
            .post(&format!("{}/api/blockchain/add-triple", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        assert!(response.status().is_success(), "Should ingest supply chain data");
        sleep(Duration::from_millis(100)).await;
    }
    
    sleep(Duration::from_millis(1000)).await;
    
    // Query 1: Check what's actually in the store
    let debug_query1 = json!({
        "query": "SELECT * WHERE { ?s ?p ?o } LIMIT 20",
        "format": "json"
    });
    
    let response1 = client
        .post(&format!("{}/api/sparql/query", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&debug_query1)
        .send()
        .await?;
    
    let data1: serde_json::Value = response1.json().await?;
    println!("Debug: All triples in store:");
    println!("{}", serde_json::to_string_pretty(&data1)?);
    
    // Query 2: Check for specific subject
    let debug_query2 = json!({
        "query": "SELECT * WHERE { <http://example.org/batch456> ?p ?o }",
        "format": "json"
    });
    
    let response2 = client
        .post(&format!("{}/api/sparql/query", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&debug_query2)
        .send()
        .await?;
    
    let data2: serde_json::Value = response2.json().await?;
    println!("Debug: Triples for batch456:");
    println!("{}", serde_json::to_string_pretty(&data2)?);
    
    // Query 3: Check with GRAPH clause
    let debug_query3 = json!({
        "query": "SELECT * WHERE { GRAPH ?g { <http://example.org/batch456> ?p ?o } }",
        "format": "json"
    });
    
    let response3 = client
        .post(&format!("{}/api/sparql/query", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&debug_query3)
        .send()
        .await?;
    
    let data3: serde_json::Value = response3.json().await?;
    println!("Debug: Triples for batch456 with GRAPH:");
    println!("{}", serde_json::to_string_pretty(&data3)?);
    
    // Query 4: Test the exact query from the handler
    let debug_query4 = json!({
        "query": r#"
        SELECT ?product ?origin ?status WHERE {
            OPTIONAL {
                GRAPH ?g1 {
                    <http://example.org/batch456> <http://provchain.org/trace#product> ?product .
                }
            }
            OPTIONAL {
                GRAPH ?g2 {
                    <http://example.org/batch456> <http://provchain.org/trace#origin> ?origin .
                }
            }
            OPTIONAL {
                GRAPH ?g3 {
                    <http://example.org/batch456> <http://provchain.org/trace#status> ?status .
                }
            }
        }
        "#,
        "format": "json"
    });
    
    let response4 = client
        .post(&format!("{}/api/sparql/query", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&debug_query4)
        .send()
        .await?;
    
    let data4: serde_json::Value = response4.json().await?;
    println!("Debug: Handler query result:");
    println!("{}", serde_json::to_string_pretty(&data4)?);
    
    Ok(())
}
