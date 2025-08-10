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
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

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
    let mut blockchain = app_state.blockchain.write().await;
    
    // Create RDF triple data
    let triple_data = format!(
        "{} {} {} .",
        request.subject,
        request.predicate,
        request.object
    );
    
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
    let blockchain = app_state.blockchain.read().await;
    let start_time = Instant::now();
    
    // Access the RDF store through the blockchain
    let query_results = blockchain.rdf_store.query(&request.query);
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
    
    // Build SPARQL query to get product information
    let sparql_query = format!(
        r#"
        PREFIX tc: <http://example.org/tracechain#>
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
        
        SELECT ?product ?origin ?location ?status ?actor ?action ?timestamp
        WHERE {{
            ?batch tc:batchId "{batch_id}" .
            ?batch tc:product ?product .
            OPTIONAL {{ ?batch tc:origin ?origin }}
            OPTIONAL {{ ?batch tc:currentLocation ?location }}
            OPTIONAL {{ ?batch tc:status ?status }}
            OPTIONAL {{
                ?event tc:batch ?batch .
                ?event tc:actor ?actor .
                ?event tc:action ?action .
                ?event tc:timestamp ?timestamp .
            }}
        }}
        ORDER BY ?timestamp
        "#
    );
    
    // Access the RDF store through the blockchain
    let _query_results = blockchain.rdf_store.query(&sparql_query);
    
    // For now, return a mock response since we need to properly parse the results
    let product_trace = ProductTrace {
        batch_id: batch_id.clone(),
        product_name: params.product_name.unwrap_or_else(|| "Unknown Product".to_string()),
        origin: "Unknown Origin".to_string(),
        current_location: "Unknown Location".to_string(),
        status: "Unknown Status".to_string(),
        timeline: vec![], // TODO: Parse from SPARQL results
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
