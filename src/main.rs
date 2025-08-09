mod blockchain;
mod rdf_store;
mod demo;

use clap::{Parser, Subcommand};
use std::fs;
use crate::blockchain::Blockchain;

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
}

fn main() {
    let cli = Cli::parse();
    let mut bc = Blockchain::new(); // In a real app, you'd load this from a file

    match cli.command {
        Commands::AddFile { path } => {
            let rdf_data = fs::read_to_string(path).expect("Cannot read RDF file");
            bc.add_block(rdf_data);
            println!("Added RDF as a new block. Blockchain is valid: {}", bc.is_valid());
        }
        Commands::Query { path } => {
            let query = fs::read_to_string(path).expect("Cannot read query file");
            if let oxigraph::sparql::QueryResults::Solutions(solutions) = bc.rdf_store.query(&query) {
                for solution in solutions {
                    println!("{:?}", solution.unwrap());
                }
            }
        }
        Commands::Validate => {
            if bc.is_valid() {
                println!("Blockchain is valid.");
            } else {
                println!("Blockchain is NOT valid.");
            }
        }
        Commands::Dump => {
            println!("{}", bc.dump());
        }
        Commands::Demo => {
            demo::run_demo();
        }
    }
}
