//! Web server implementation using Axum

use crate::blockchain::Blockchain;
use crate::web::{
    auth::{AuthState, authenticate, auth_middleware},
    handlers::{
        AppState, health_check, get_blockchain_status, get_block, get_blocks,
        add_triple, execute_sparql_query, get_product_trace, get_recent_transactions,
        validate_blockchain,
    },
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, error};

/// Web server for the blockchain API
pub struct WebServer {
    app_state: AppState,
    auth_state: AuthState,
    port: u16,
}

impl WebServer {
    /// Create a new web server instance
    pub fn new(blockchain: Blockchain, port: u16) -> Self {
        Self {
            app_state: AppState::new(blockchain),
            auth_state: AuthState::new(),
            port,
        }
    }

    /// Build the router with all routes and middleware
    fn build_router(&self) -> Router {
        // Public routes (no authentication required)
        let public_routes = Router::new()
            .route("/health", get(health_check))
            .route("/auth/login", post(authenticate))
            .with_state(self.auth_state.clone());

        // Protected routes (authentication required)
        let protected_routes = Router::new()
            .route("/api/blockchain/status", get(get_blockchain_status))
            .route("/api/blockchain/blocks", get(get_blocks))
            .route("/api/blockchain/blocks/:index", get(get_block))
            .route("/api/blockchain/validate", get(validate_blockchain))
            .route("/api/transactions/recent", get(get_recent_transactions))
            .route("/api/sparql/query", post(execute_sparql_query))
            .route("/api/products/trace", get(get_product_trace))
            .route("/api/blockchain/add-triple", post(add_triple))
            .layer(middleware::from_fn(auth_middleware))
            .with_state(self.app_state.clone());

        // Combine routes and add CORS
        Router::new()
            .merge(public_routes)
            .merge(protected_routes)
            .layer(
                ServiceBuilder::new()
                    .layer(
                        CorsLayer::new()
                            .allow_origin(Any)
                            .allow_methods(Any)
                            .allow_headers(Any),
                    )
                    .into_inner(),
            )
    }

    /// Start the web server
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let app = self.build_router();
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));

        info!("Starting web server on {}", addr);
        info!("API endpoints available:");
        info!("  GET  /health - Health check");
        info!("  POST /auth/login - Authentication");
        info!("  GET  /api/blockchain/status - Blockchain status");
        info!("  GET  /api/blockchain/blocks - All blocks");
        info!("  GET  /api/blockchain/blocks/:index - Specific block");
        info!("  GET  /api/blockchain/validate - Validate blockchain");
        info!("  GET  /api/transactions/recent - Recent transactions");
        info!("  POST /api/sparql/query - Execute SPARQL query");
        info!("  GET  /api/products/trace - Product traceability");
        info!("  POST /api/blockchain/add-triple - Add new triple");

        let listener = tokio::net::TcpListener::bind(addr).await?;
        
        match axum::serve(listener, app).await {
            Ok(_) => {
                info!("Web server started successfully");
                Ok(())
            }
            Err(e) => {
                error!("Failed to start web server: {}", e);
                Err(Box::new(e))
            }
        }
    }

    /// Get the server port
    pub fn port(&self) -> u16 {
        self.port
    }
}

/// Create and configure the web server
pub async fn create_web_server(
    blockchain: Blockchain,
    port: Option<u16>,
) -> Result<WebServer, Box<dyn std::error::Error>> {
    let server_port = port.unwrap_or(8080);
    let server = WebServer::new(blockchain, server_port);
    
    info!("Web server configured on port {}", server_port);
    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::Blockchain;

    #[tokio::test]
    async fn test_server_creation() {
        let blockchain = Blockchain::new();
        let server = WebServer::new(blockchain, 8080);
        assert_eq!(server.port(), 8080);
    }
}
