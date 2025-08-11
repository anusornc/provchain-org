//! Debug SPARQL integration test

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
async fn test_debug_sparql_integration() -> Result<()> {
    let (port, _server_handle) = start_test_server().await?;
    let base_url = format!("http://localhost:{}", port);
    let client = Client::new();
    
    println!("Debug SPARQL Integration on {}", base_url);
    
    // Step 1: Authentication
    let token = get_auth_token(&client, &base_url, "admin", "admin123").await?;
    assert!(!token.is_empty(), "Should receive authentication token");
    
    // Step 2: Add a single triple
    println!("Adding single triple...");
    let triple_data = json!({
        "subject": "http://example.org/test123",
        "predicate": "http://provchain.org/trace#name",
        "object": "Test Value",
        "graph_name": "debug_test"
    });
    
    let response = client
        .post(&format!("{}/api/blockchain/add-triple", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&triple_data)
        .send()
        .await?;
    
    assert!(response.status().is_success(), "Should successfully add triple");
    let response_data: serde_json::Value = response.json().await?;
    println!("Add response: {}", serde_json::to_string_pretty(&response_data)?);
    
    // Step 3: Wait a moment for processing
    sleep(Duration::from_millis(500)).await;
    
    // Step 4: Try different SPARQL queries to debug
    
    // Query 1: Simple query without graph
    println!("\n--- Query 1: Simple query without graph ---");
    let query1 = json!({
        "query": "SELECT * WHERE { ?s ?p ?o } LIMIT 10",
        "format": "json"
    });
    
    let query1_response = client
        .post(&format!("{}/api/sparql/query", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&query1)
        .send()
        .await?;
    
    let query1_data: serde_json::Value = query1_response.json().await?;
    println!("Query 1 results: {}", serde_json::to_string_pretty(&query1_data)?);
    
    // Query 2: Query with GRAPH clause
    println!("\n--- Query 2: Query with GRAPH clause ---");
    let query2 = json!({
        "query": "SELECT * WHERE { GRAPH ?g { ?s ?p ?o } } LIMIT 10",
        "format": "json"
    });
    
    let query2_response = client
        .post(&format!("{}/api/sparql/query", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&query2)
        .send()
        .await?;
    
    let query2_data: serde_json::Value = query2_response.json().await?;
    println!("Query 2 results: {}", serde_json::to_string_pretty(&query2_data)?);
    
    // Query 3: Query specific subject
    println!("\n--- Query 3: Query specific subject ---");
    let query3 = json!({
        "query": "SELECT * WHERE { <http://example.org/test123> ?p ?o }",
        "format": "json"
    });
    
    let query3_response = client
        .post(&format!("{}/api/sparql/query", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&query3)
        .send()
        .await?;
    
    let query3_data: serde_json::Value = query3_response.json().await?;
    println!("Query 3 results: {}", serde_json::to_string_pretty(&query3_data)?);
    
    // Query 4: Query specific subject with GRAPH
    println!("\n--- Query 4: Query specific subject with GRAPH ---");
    let query4 = json!({
        "query": "SELECT * WHERE { GRAPH ?g { <http://example.org/test123> ?p ?o } }",
        "format": "json"
    });
    
    let query4_response = client
        .post(&format!("{}/api/sparql/query", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&query4)
        .send()
        .await?;
    
    let query4_data: serde_json::Value = query4_response.json().await?;
    println!("Query 4 results: {}", serde_json::to_string_pretty(&query4_data)?);
    
    // Step 5: Check blockchain status
    println!("\n--- Blockchain Status ---");
    let status_response = client
        .get(&format!("{}/api/blockchain/status", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    
    let status_data: serde_json::Value = status_response.json().await?;
    println!("Blockchain status: {}", serde_json::to_string_pretty(&status_data)?);
    
    Ok(())
}
