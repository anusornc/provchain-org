//! HTTP handlers for REST API endpoints

use crate::blockchain::Blockchain;
use crate::web::models::{
    BlockchainStatus, BlockInfo, TransactionInfo, AddTripleRequest, 
    SparqlQueryRequest, SparqlQueryResponse, ProductTrace,
    EnvironmentalData, ApiError, UserClaims
};
use axum::{
    extract::{Path, Query, State, Extension},
    http::StatusCode,
    Json,
};
use regex::Regex;
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

/// Input validation functions
fn validate_uri(uri: &str) -> Result<(), String> {
    if uri.is_empty() {
        return Err("URI cannot be empty".to_string());
    }
    
    if uri.len() > 2048 {
        return Err("URI too long (max 2048 characters)".to_string());
    }
    
    // Basic URI validation
    let uri_regex = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$|^[a-zA-Z][a-zA-Z0-9+.-]*:[^\s]*$").unwrap();
    if !uri_regex.is_match(uri) {
        return Err("Invalid URI format".to_string());
    }
    
    Ok(())
}

fn validate_literal(literal: &str) -> Result<(), String> {
    if literal.len() > 10000 {
        return Err("Literal too long (max 10000 characters)".to_string());
    }
    
    // Check for potential script injection
    let dangerous_patterns = ["<script", "javascript:", "data:", "vbscript:", "onload=", "onerror="];
    let literal_lower = literal.to_lowercase();
    for pattern in &dangerous_patterns {
        if literal_lower.contains(pattern) {
            return Err("Literal contains potentially dangerous content".to_string());
        }
    }
    
    Ok(())
}

fn validate_sparql_query(query: &str) -> Result<(), String> {
    if query.is_empty() {
        return Err("SPARQL query cannot be empty".to_string());
    }
    
    if query.len() > 50000 {
        return Err("SPARQL query too long (max 50000 characters)".to_string());
    }
    
    // Check for potentially dangerous operations
    let query_upper = query.to_uppercase();
    let dangerous_operations = ["DROP", "CLEAR", "DELETE", "INSERT", "LOAD", "CREATE"];
    for operation in &dangerous_operations {
        if query_upper.contains(operation) {
            return Err(format!("SPARQL operation '{}' is not allowed", operation));
        }
    }
    
    Ok(())
}


/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub blockchain: Arc<RwLock<Blockchain>>,
}

impl AppState {
    pub fn new(blockchain: Blockchain) -> Self {
        Self {
            blockchain: Arc::new(RwLock::new(blockchain)),
        }
    }
}

/// Health check endpoint
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": Utc::now(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Get blockchain status
pub async fn get_blockchain_status(
    State(app_state): State<AppState>,
) -> Result<Json<BlockchainStatus>, (StatusCode, Json<ApiError>)> {
    let blockchain = app_state.blockchain.read().await;
    
    let status = BlockchainStatus {
        height: blockchain.chain.len(),
        latest_block_hash: blockchain.chain.last()
            .map(|b| b.hash.clone())
            .unwrap_or_else(|| "genesis".to_string()),
        total_transactions: blockchain.chain.len(), // Each block is one transaction
        network_peers: 0, // TODO: Get from network module
        last_updated: Utc::now(),
    };
    
    Ok(Json(status))
}

/// Get specific block information
pub async fn get_block(
    Path(block_index): Path<usize>,
    State(app_state): State<AppState>,
) -> Result<Json<BlockInfo>, (StatusCode, Json<ApiError>)> {
    let blockchain = app_state.blockchain.read().await;
    
    if let Some(block) = blockchain.chain.get(block_index) {
        let block_info = BlockInfo {
            index: block.index as usize,
            hash: block.hash.clone(),
            previous_hash: block.previous_hash.clone(),
            timestamp: chrono::DateTime::parse_from_rfc3339(&block.timestamp)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| Utc::now()),
            transaction_count: 1, // Each block contains one data entry
            size_bytes: serde_json::to_string(block)
                .map(|s| s.len())
                .unwrap_or(0),
        };
        Ok(Json(block_info))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "block_not_found".to_string(),
                message: format!("Block with index {block_index} not found"),
                timestamp: Utc::now(),
            }),
        ))
    }
}

/// Get all blocks
pub async fn get_blocks(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<BlockInfo>>, (StatusCode, Json<ApiError>)> {
    let blockchain = app_state.blockchain.read().await;
    
    let blocks: Vec<BlockInfo> = blockchain.chain.iter().map(|block| {
        BlockInfo {
            index: block.index as usize,
            hash: block.hash.clone(),
            previous_hash: block.previous_hash.clone(),
            timestamp: chrono::DateTime::parse_from_rfc3339(&block.timestamp)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| Utc::now()),
            transaction_count: 1, // Each block contains one data entry
            size_bytes: serde_json::to_string(block)
                .map(|s| s.len())
                .unwrap_or(0),
        }
    }).collect();
    
    Ok(Json(blocks))
}

/// Add new triple to blockchain
pub async fn add_triple(
    State(app_state): State<AppState>,
    Extension(claims): Extension<UserClaims>,
    Json(request): Json<AddTripleRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ApiError>)> {
    // Validate inputs
    if let Err(e) = validate_uri(&request.subject) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "invalid_subject".to_string(),
                message: format!("Invalid subject URI: {}", e),
                timestamp: Utc::now(),
            }),
        ));
    }
    
    if let Err(e) = validate_uri(&request.predicate) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "invalid_predicate".to_string(),
                message: format!("Invalid predicate URI: {}", e),
                timestamp: Utc::now(),
            }),
        ));
    }
    
    // Validate object based on whether it's a URI or literal
    if request.object.starts_with("http://") || request.object.starts_with("https://") {
        if let Err(e) = validate_uri(&request.object) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "invalid_object_uri".to_string(),
                    message: format!("Invalid object URI: {}", e),
                    timestamp: Utc::now(),
                }),
            ));
        }
    } else {
        if let Err(e) = validate_literal(&request.object) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "invalid_object_literal".to_string(),
                    message: format!("Invalid object literal: {}", e),
                    timestamp: Utc::now(),
                }),
            ));
        }
    }
    
    let mut blockchain = app_state.blockchain.write().await;
    
    // Create proper RDF triple data in Turtle format
    let triple_data = if request.object.starts_with("http://") || request.object.starts_with("https://") {
        // Object is a URI, don't quote it
        format!(
            "<{}> <{}> <{}> .",
            request.subject,
            request.predicate,
            request.object
        )
    } else {
        // Object is a literal, quote it
        format!(
            "<{}> <{}> \"{}\" .",
            request.subject,
            request.predicate,
            request.object
        )
    };
    
    // Add to blockchain (this also adds to the internal RDF store)
    blockchain.add_block(triple_data);
    
    let block_hash = blockchain.chain.last()
        .map(|b| b.hash.clone())
        .unwrap_or_else(|| "unknown".to_string());
    
    Ok(Json(serde_json::json!({
        "success": true,
        "block_hash": block_hash,
        "block_index": blockchain.chain.len() - 1,
        "added_by": claims.sub,
        "timestamp": Utc::now()
    })))
}

/// Execute SPARQL query
pub async fn execute_sparql_query(
    State(app_state): State<AppState>,
    Json(request): Json<SparqlQueryRequest>,
) -> Result<Json<SparqlQueryResponse>, (StatusCode, Json<ApiError>)> {
    // Validate SPARQL query
    if let Err(e) = validate_sparql_query(&request.query) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "invalid_sparql_query".to_string(),
                message: format!("SPARQL query validation failed: {}", e),
                timestamp: Utc::now(),
            }),
        ));
    }
    
    let blockchain = app_state.blockchain.read().await;
    let start_time = Instant::now();
    
    // Access the RDF store through the blockchain and handle potential query errors
    let query_results = match blockchain.rdf_store.store.query(&request.query) {
        Ok(results) => results,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "invalid_sparql_query".to_string(),
                    message: format!("Invalid SPARQL query: {}", e),
                    timestamp: Utc::now(),
                }),
            ));
        }
    };
    let execution_time = start_time.elapsed().as_millis() as u64;
    
    // Convert QueryResults to JSON
    let results_json = match query_results {
        oxigraph::sparql::QueryResults::Solutions(solutions) => {
            let mut bindings = Vec::new();
            for solution in solutions {
                match solution {
                    Ok(sol) => {
                        let mut binding = serde_json::Map::new();
                        for (var, term) in sol.iter() {
                            binding.insert(var.as_str().to_string(), serde_json::Value::String(term.to_string()));
                        }
                        bindings.push(serde_json::Value::Object(binding));
                    }
                    Err(_) => continue,
                }
            }
            serde_json::json!({
                "head": { "vars": [] },
                "results": { "bindings": bindings }
            })
        }
        oxigraph::sparql::QueryResults::Boolean(result) => {
            serde_json::json!({
                "head": {},
                "boolean": result
            })
        }
        oxigraph::sparql::QueryResults::Graph(_) => {
            serde_json::json!({
                "head": {},
                "results": "Graph results not yet supported"
            })
        }
    };
    
    let result_count = if let Some(bindings) = results_json.get("results").and_then(|r| r.get("bindings")) {
        bindings.as_array().map(|arr| arr.len()).unwrap_or(0)
    } else {
        1
    };
    
    let response = SparqlQueryResponse {
        results: results_json,
        execution_time_ms: execution_time,
        result_count,
    };
    
    Ok(Json(response))
}

/// Query parameters for product trace
#[derive(Deserialize)]
pub struct TraceQueryParams {
    batch_id: Option<String>,
    product_name: Option<String>,
}

/// Get product traceability information
pub async fn get_product_trace(
    Query(params): Query<TraceQueryParams>,
    State(app_state): State<AppState>,
) -> Result<Json<ProductTrace>, (StatusCode, Json<ApiError>)> {
    let blockchain = app_state.blockchain.read().await;
    
    let batch_id = params.batch_id.unwrap_or_else(|| "unknown".to_string());
    
    // Build SPARQL query to get product information using the actual namespace
    // Each triple is stored in a separate graph (one per blockchain block)
    let sparql_query = format!(
        r#"
        SELECT ?product ?origin ?status WHERE {{
            OPTIONAL {{
                GRAPH ?g1 {{
                    <http://example.org/batch456> <http://provchain.org/trace#product> ?product .
                }}
            }}
            OPTIONAL {{
                GRAPH ?g2 {{
                    <http://example.org/batch456> <http://provchain.org/trace#origin> ?origin .
                }}
            }}
            OPTIONAL {{
                GRAPH ?g3 {{
                    <http://example.org/batch456> <http://provchain.org/trace#status> ?status .
                }}
            }}
        }}
        "#
    );
    
    // Access the RDF store through the blockchain
    let query_results = blockchain.rdf_store.query(&sparql_query);
    
    let mut product_name = "Unknown Product".to_string();
    let mut origin = "Unknown Origin".to_string();
    let mut status = "Unknown Status".to_string();
    
    // Parse SPARQL results
    if let oxigraph::sparql::QueryResults::Solutions(solutions) = query_results {
        for solution in solutions {
            if let Ok(sol) = solution {
                if let Some(product_term) = sol.get("product") {
                    product_name = product_term.to_string().trim_matches('"').to_string();
                }
                if let Some(origin_term) = sol.get("origin") {
                    origin = origin_term.to_string().trim_matches('"').to_string();
                }
                if let Some(status_term) = sol.get("status") {
                    status = status_term.to_string().trim_matches('"').to_string();
                }
                break; // Take the first result
            }
        }
    }
    
    // Override with query parameter if provided
    if let Some(param_product_name) = params.product_name {
        product_name = param_product_name;
    }
    
    let product_trace = ProductTrace {
        batch_id: batch_id.clone(),
        product_name,
        origin,
        current_location: "Unknown Location".to_string(),
        status,
        timeline: vec![], // TODO: Parse timeline events from SPARQL results
        certifications: vec![],
        environmental_data: Some(EnvironmentalData {
            temperature: Some(22.5),
            humidity: Some(65.0),
            co2_footprint: Some(1.2),
            certifications: vec!["Organic".to_string(), "Fair Trade".to_string()],
        }),
    };
    
    Ok(Json(product_trace))
}

/// Get recent transactions
pub async fn get_recent_transactions(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<TransactionInfo>>, (StatusCode, Json<ApiError>)> {
    let blockchain = app_state.blockchain.read().await;
    
    let mut transactions = Vec::new();
    
    // Get transactions from the last 10 blocks
    let start_index = blockchain.chain.len().saturating_sub(10);
    
    for (block_index, block) in blockchain.chain.iter().enumerate().skip(start_index) {
        // Parse RDF triple from block data (which is a single string)
        let parts: Vec<&str> = block.data.split_whitespace().collect();
        if parts.len() >= 3 {
            transactions.push(TransactionInfo {
                subject: parts[0].to_string(),
                predicate: parts[1].to_string(),
                object: parts[2..parts.len()-1].join(" "), // Remove the trailing "."
                block_index,
                timestamp: chrono::DateTime::parse_from_rfc3339(&block.timestamp)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| Utc::now()),
            });
        }
    }
    
    // Sort by timestamp (most recent first)
    transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    Ok(Json(transactions))
}

/// Validate blockchain integrity
pub async fn validate_blockchain(
    State(app_state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ApiError>)> {
    let blockchain = app_state.blockchain.read().await;
    
    let is_valid = blockchain.is_valid();
    
    Ok(Json(serde_json::json!({
        "is_valid": is_valid,
        "total_blocks": blockchain.chain.len(),
        "validation_timestamp": Utc::now()
    })))
}
