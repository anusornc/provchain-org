use clap::{Parser, Subcommand};
use provchain_org::{blockchain::Blockchain, rdf_store::RDFStore, demo, web::server::create_web_server, demo_runner::run_demo_with_args};
use std::fs;
use tracing::info;

#[derive(Parser)]
#[command(name = "TraceChain")]
#[command(about = "Blockchain + RDF Traceability CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a Turtle RDF file as a new block
    AddFile { path: String },

    /// Run a SPARQL query file
    Query { path: String },

    /// Validate the integrity of the blockchain
    Validate,

    /// Dump the blockchain to stdout as JSON
    Dump,

    /// Run the built-in UHT manufacturing demo
    Demo,

    /// Run transaction blockchain demos
    TransactionDemo {
        /// Demo type: uht, basic, signing, multi, all, interactive
        #[arg(short, long, default_value = "interactive")]
        demo_type: String,
    },

    /// Start the web server for Phase 2 REST API
    WebServer {
        /// Port to run the web server on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let mut blockchain = Blockchain::new();
    let _rdf_store = RDFStore::new();

    match cli.command {
        Commands::AddFile { path } => {
            let rdf_data = fs::read_to_string(&path)
                .map_err(|e| format!("Cannot read RDF file '{path}': {e}"))?;
            
            blockchain.add_block(rdf_data);
            let block_hash = blockchain.chain.last()
                .map(|b| b.hash.clone())
                .unwrap_or_else(|| "unknown".to_string());
            
            println!("Added RDF as a new block with hash: {block_hash}");
            println!("Blockchain is valid: {}", blockchain.is_valid());
        }
        Commands::Query { path } => {
            let query = fs::read_to_string(&path)
                .map_err(|e| format!("Cannot read query file '{path}': {e}"))?;
            
            let _results = blockchain.rdf_store.query(&query);
            println!("Query results:");
            // For now, just print that query was executed
            println!("Query executed successfully");
        }
        Commands::Validate => {
            if blockchain.is_valid() {
                println!("✅ Blockchain is valid.");
            } else {
                println!("❌ Blockchain is NOT valid.");
            }
        }
        Commands::Dump => {
            let json = blockchain.dump();
            println!("{json}");
        }
        Commands::Demo => {
            info!("Running built-in demo...");
            demo::run_demo();
        }
        Commands::TransactionDemo { demo_type } => {
            info!("Running transaction blockchain demo: {}", demo_type);
            let args = vec!["provchain".to_string(), demo_type];
            if let Err(e) = run_demo_with_args(args) {
                eprintln!("Demo error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::WebServer { port } => {
            info!("Starting Phase 2 web server on port {}", port);
            
            // Load some demo data into the blockchain
            let demo_data = vec![
                "<http://example.org/batch1> <http://example.org/product> \"Organic Tomatoes\" .",
                "<http://example.org/batch1> <http://example.org/origin> \"Farm A\" .",
                "<http://example.org/batch1> <http://example.org/status> \"In Transit\" .",
            ];
            
            // Add each piece of demo data as a separate block
            for data in demo_data {
                blockchain.add_block(data.to_string());
            }
            
            // Create and start the web server
            let web_server = create_web_server(blockchain, Some(port)).await?;
            
            info!("🚀 Web server starting...");
            info!("📡 API available at: http://localhost:{}", port);
            info!("🔍 Health check: http://localhost:{}/health", port);
            info!("🔐 Login endpoint: http://localhost:{}/auth/login", port);
            info!("📊 Blockchain status: http://localhost:{}/api/blockchain/status", port);
            info!("");
            info!("Default users for testing:");
            info!("  - admin/admin123 (Admin role)");
            info!("  - farmer1/farmer123 (Farmer role)");
            info!("  - processor1/processor123 (Processor role)");
            info!("");
            info!("Press Ctrl+C to stop the server");
            
            web_server.start().await?;
        }
    }

    Ok(())
}
