//! Data models for web API responses and requests

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Response model for blockchain status
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainStatus {
    pub height: usize,
    pub latest_block_hash: String,
    pub total_transactions: usize,
    pub network_peers: usize,
    pub last_updated: DateTime<Utc>,
}

/// Response model for block information
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockInfo {
    pub index: usize,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: DateTime<Utc>,
    pub transaction_count: usize,
    pub size_bytes: usize,
}

/// Response model for transaction/triple information
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub block_index: usize,
    pub timestamp: DateTime<Utc>,
}

/// Request model for adding new triples
#[derive(Debug, Serialize, Deserialize)]
pub struct AddTripleRequest {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub graph_name: Option<String>,
}

/// Request model for SPARQL queries
#[derive(Debug, Serialize, Deserialize)]
pub struct SparqlQueryRequest {
    pub query: String,
    pub format: Option<String>, // json, xml, turtle, etc.
}

/// Response model for SPARQL query results
#[derive(Debug, Serialize, Deserialize)]
pub struct SparqlQueryResponse {
    pub results: serde_json::Value,
    pub execution_time_ms: u64,
    pub result_count: usize,
}

/// Response model for product traceability
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductTrace {
    pub batch_id: String,
    pub product_name: String,
    pub origin: String,
    pub current_location: String,
    pub status: String,
    pub timeline: Vec<TraceEvent>,
    pub certifications: Vec<String>,
    pub environmental_data: Option<EnvironmentalData>,
}

/// Individual trace event in product journey
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceEvent {
    pub timestamp: DateTime<Utc>,
    pub location: String,
    pub actor: String,
    pub action: String,
    pub details: String,
    pub block_hash: String,
}

/// Environmental conditions data
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalData {
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
    pub co2_footprint: Option<f64>,
    pub certifications: Vec<String>,
}

/// API error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

/// Authentication request
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

/// Authentication response
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub user_role: String,
}

/// User claims for JWT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: String, // user id
    pub role: String,
    pub exp: usize, // expiration timestamp
}

/// Supply chain actor roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorRole {
    Farmer,
    Processor,
    Transporter,
    Retailer,
    Consumer,
    Auditor,
    Admin,
}

impl std::fmt::Display for ActorRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorRole::Farmer => write!(f, "farmer"),
            ActorRole::Processor => write!(f, "processor"),
            ActorRole::Transporter => write!(f, "transporter"),
            ActorRole::Retailer => write!(f, "retailer"),
            ActorRole::Consumer => write!(f, "consumer"),
            ActorRole::Auditor => write!(f, "auditor"),
            ActorRole::Admin => write!(f, "admin"),
        }
    }
}
