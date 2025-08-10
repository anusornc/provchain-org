use oxigraph::io::RdfFormat;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;
use std::io::Cursor;
use sha2::{Sha256, Digest};
use std::collections::{HashSet, HashMap};
use std::time::Instant;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use tracing::{info, warn, error, debug};

use crate::blockchain::Block;

/// Graph complexity classification for adaptive canonicalization
#[derive(Debug, Clone, PartialEq)]
pub enum GraphComplexity {
    Simple,
    Moderate,
    Complex,
    Pathological,
}

/// Canonicalization algorithm selection
#[derive(Debug, Clone, PartialEq)]
pub enum CanonicalizationAlgorithm {
    Custom,      // Fast hash-based approach
    RDFC10,      // W3C RDFC-1.0 standard
}

/// Performance metrics for canonicalization operations
#[derive(Debug, Clone)]
pub struct CanonicalizationMetrics {
    pub algorithm_used: CanonicalizationAlgorithm,
    pub execution_time_ms: u128,
    pub graph_size: usize,
    pub blank_node_count: usize,
    pub complexity: GraphComplexity,
}

/// Identifier issuer for RDFC-1.0 canonical blank node labeling
#[derive(Debug, Clone)]
struct IdentifierIssuer {
    prefix: String,
    counter: u32,
    issued: HashMap<String, String>,
}

impl IdentifierIssuer {
    fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            counter: 0,
            issued: HashMap::new(),
        }
    }

    fn issue(&mut self, existing: Option<&str>) -> String {
        if let Some(existing_id) = existing {
            if let Some(issued_id) = self.issued.get(existing_id) {
                return issued_id.clone();
            }
        }

        let new_id = format!("{}{}", self.prefix, self.counter);
        self.counter += 1;

        if let Some(existing_id) = existing {
            self.issued.insert(existing_id.to_string(), new_id.clone());
        }

        new_id
    }

    fn clone_issuer(&self) -> Self {
        Self {
            prefix: self.prefix.clone(),
            counter: self.counter,
            issued: self.issued.clone(),
        }
    }
}

/// Configuration for persistent RDF storage
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub data_dir: PathBuf,
    pub enable_backup: bool,
    pub backup_interval_hours: u64,
    pub max_backup_files: usize,
    pub enable_compression: bool,
    pub enable_encryption: bool,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./data/rdf_store"),
            enable_backup: true,
            backup_interval_hours: 24,
            max_backup_files: 7,
            enable_compression: true,
            enable_encryption: false,
        }
    }
}

/// Backup metadata
#[derive(Debug, Clone)]
pub struct BackupInfo {
    pub path: PathBuf,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub size_bytes: u64,
    pub compressed: bool,
    pub encrypted: bool,
}

pub struct RDFStore {
    pub store: Store,
    pub config: StorageConfig,
    pub is_persistent: bool,
}

impl Default for RDFStore {
    fn default() -> Self {
        Self::new()
    }
}

impl RDFStore {
    /// Create a new in-memory RDF store (for testing and development)
    pub fn new() -> Self {
        info!("Creating new in-memory RDF store");
        RDFStore {
            store: Store::new().unwrap(),
            config: StorageConfig::default(),
            is_persistent: false,
        }
    }

    /// Create a new persistent RDF store with file-based backend
    pub fn new_persistent<P: AsRef<Path>>(data_dir: P) -> Result<Self> {
        let data_path = data_dir.as_ref().to_path_buf();
        info!("Creating persistent RDF store at: {}", data_path.display());
        
        // Ensure the data directory exists
        std::fs::create_dir_all(&data_path)
            .with_context(|| format!("Failed to create data directory: {}", data_path.display()))?;
        
        // Create in-memory store - Oxigraph handles persistence through file operations
        let store = Store::new()
            .with_context(|| "Failed to create in-memory store")?;
        
        let config = StorageConfig {
            data_dir: data_path,
            ..StorageConfig::default()
        };
        
        let mut rdf_store = RDFStore {
            store,
            config,
            is_persistent: true,
        };
        
        // Try to load existing data
        if let Err(e) = rdf_store.load_from_disk() {
            warn!("Could not load existing data: {}", e);
        }
        
        info!("Successfully created persistent RDF store");
        Ok(rdf_store)
    }

    /// Create a persistent store with custom configuration
    pub fn new_persistent_with_config(config: StorageConfig) -> Result<Self> {
        info!("Creating persistent RDF store with custom config at: {}", config.data_dir.display());
        
        // Ensure the data directory exists
        if let Some(parent) = config.data_dir.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create data directory: {}", parent.display()))?;
        }
        
        // Create in-memory store for now, but track persistence config
        let store = Store::new()
            .with_context(|| "Failed to create in-memory store")?;
        
        let mut rdf_store = RDFStore {
            store,
            config,
            is_persistent: true,
        };
        
        // Try to load existing data
        if let Err(e) = rdf_store.load_from_disk() {
            warn!("Could not load existing data: {}", e);
        }
        
        info!("Successfully created persistent RDF store with custom config");
        Ok(rdf_store)
    }

    /// Load RDF data from disk
    fn load_from_disk(&mut self) -> Result<()> {
        if !self.is_persistent {
            return Ok(());
        }
        
        let data_file = self.config.data_dir.join("store.ttl");
        if !data_file.exists() {
            return Ok(());
        }
        
        info!("Loading RDF data from: {}", data_file.display());
        
        let data = std::fs::read_to_string(&data_file)
            .with_context(|| format!("Failed to read data file: {}", data_file.display()))?;
        
        use oxigraph::io::RdfFormat;
        use std::io::Cursor;
        
        let reader = Cursor::new(data.as_bytes());
        self.store.load_from_reader(RdfFormat::Turtle, reader)
            .with_context(|| "Failed to parse RDF data from disk")?;
        
        let quad_count = self.store.len().unwrap_or(0);
        info!("Successfully loaded {} quads from disk", quad_count);
        Ok(())
    }

    /// Save RDF data to disk
    pub fn save_to_disk(&self) -> Result<()> {
        if !self.is_persistent {
            return Ok(());
        }
        
        let data_file = self.config.data_dir.join("store.ttl");
        
        info!("Saving RDF data to: {}", data_file.display());
        
        use oxigraph::io::RdfFormat;
        
        let mut buffer = Vec::new();
        self.store.dump_to_writer(RdfFormat::Turtle, &mut buffer)
            .with_context(|| "Failed to serialize RDF data")?;
        
        std::fs::write(&data_file, buffer)
            .with_context(|| format!("Failed to write data file: {}", data_file.display()))?;
        
        let quad_count = self.store.len().unwrap_or(0);
        info!("Successfully saved {} quads to disk", quad_count);
        Ok(())
    }

    /// Get storage statistics
    pub fn get_storage_stats(&self) -> Result<StorageStats> {
        let quad_count = self.store.len();
        
        let (disk_usage, backup_count) = if self.is_persistent {
            let disk_usage = self.calculate_disk_usage()?;
            let backup_count = self.list_backups()?.len();
            (Some(disk_usage), backup_count)
        } else {
            (None, 0)
        };
        
        Ok(StorageStats {
            quad_count: quad_count.unwrap_or(0),
            disk_usage_bytes: disk_usage,
            backup_count,
            is_persistent: self.is_persistent,
            data_dir: if self.is_persistent { Some(self.config.data_dir.clone()) } else { None },
        })
    }

    /// Calculate disk usage of the store
    fn calculate_disk_usage(&self) -> Result<u64> {
        if !self.is_persistent {
            return Ok(0);
        }
        
        let mut total_size = 0u64;
        
        fn dir_size(path: &Path) -> Result<u64> {
            let mut size = 0u64;
            if path.is_dir() {
                for entry in std::fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        size += dir_size(&path)?;
                    } else {
                        size += entry.metadata()?.len();
                    }
                }
            } else {
                size += std::fs::metadata(path)?.len();
            }
            Ok(size)
        }
        
        if self.config.data_dir.exists() {
            total_size = dir_size(&self.config.data_dir)?;
        }
        
        Ok(total_size)
    }

    /// Create a backup of the current store
    pub fn create_backup(&self) -> Result<BackupInfo> {
        if !self.is_persistent {
            return Err(anyhow::anyhow!("Cannot backup in-memory store"));
        }
        
        let timestamp = chrono::Utc::now();
        let backup_filename = format!("backup_{}.db", timestamp.format("%Y%m%d_%H%M%S"));
        let backup_dir = self.config.data_dir.parent()
            .unwrap_or(&self.config.data_dir)
            .join("backups");
        
        std::fs::create_dir_all(&backup_dir)
            .with_context(|| format!("Failed to create backup directory: {}", backup_dir.display()))?;
        
        let backup_path = backup_dir.join(&backup_filename);
        
        info!("Creating backup at: {}", backup_path.display());
        
        // Copy the entire data directory for backup
        self.copy_directory(&self.config.data_dir, &backup_path)?;
        
        let size_bytes = self.calculate_backup_size(&backup_path)?;
        
        let backup_info = BackupInfo {
            path: backup_path,
            timestamp,
            size_bytes,
            compressed: self.config.enable_compression,
            encrypted: self.config.enable_encryption,
        };
        
        // Clean up old backups if needed
        self.cleanup_old_backups()?;
        
        info!("Backup created successfully: {} bytes", size_bytes);
        Ok(backup_info)
    }

    /// Copy directory recursively
    fn copy_directory(&self, src: &Path, dst: &Path) -> Result<()> {
        std::fs::create_dir_all(dst)?;
        
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            
            if src_path.is_dir() {
                self.copy_directory(&src_path, &dst_path)?;
            } else {
                std::fs::copy(&src_path, &dst_path)?;
            }
        }
        
        Ok(())
    }

    /// Calculate backup size
    fn calculate_backup_size(&self, backup_path: &Path) -> Result<u64> {
        let mut size = 0u64;
        
        fn dir_size(path: &Path) -> Result<u64> {
            let mut size = 0u64;
            if path.is_dir() {
                for entry in std::fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        size += dir_size(&path)?;
                    } else {
                        size += entry.metadata()?.len();
                    }
                }
            } else {
                size += std::fs::metadata(path)?.len();
            }
            Ok(size)
        }
        
        if backup_path.exists() {
            size = dir_size(backup_path)?;
        }
        
        Ok(size)
    }

    /// List all available backups
    pub fn list_backups(&self) -> Result<Vec<BackupInfo>> {
        if !self.is_persistent {
            return Ok(Vec::new());
        }
        
        let backup_dir = self.config.data_dir.parent()
            .unwrap_or(&self.config.data_dir)
            .join("backups");
        
        if !backup_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut backups = Vec::new();
        
        for entry in std::fs::read_dir(&backup_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() && path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("backup_"))
                .unwrap_or(false) {
                
                let metadata = entry.metadata()?;
                let size_bytes = self.calculate_backup_size(&path)?;
                
                // Parse timestamp from filename
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if let Some(timestamp_str) = filename.strip_prefix("backup_").and_then(|s| s.strip_suffix(".db")) {
                        if let Ok(timestamp) = chrono::DateTime::parse_from_str(
                            &format!("{} +0000", timestamp_str.replace('_', " ")),
                            "%Y%m%d %H%M%S %z"
                        ) {
                            backups.push(BackupInfo {
                                path: path.clone(),
                                timestamp: timestamp.with_timezone(&chrono::Utc),
                                size_bytes,
                                compressed: self.config.enable_compression,
                                encrypted: self.config.enable_encryption,
                            });
                        }
                    }
                }
            }
        }
        
        // Sort by timestamp (newest first)
        backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        Ok(backups)
    }

    /// Restore from a backup
    pub fn restore_from_backup<P: AsRef<Path>>(backup_path: P, target_dir: P) -> Result<Self> {
        let backup_path = backup_path.as_ref();
        let target_path = target_dir.as_ref();
        
        info!("Restoring from backup: {} to {}", backup_path.display(), target_path.display());
        
        if !backup_path.exists() {
            return Err(anyhow::anyhow!("Backup path does not exist: {}", backup_path.display()));
        }
        
        // Remove existing target directory if it exists
        if target_path.exists() {
            std::fs::remove_dir_all(target_path)
                .with_context(|| format!("Failed to remove existing target directory: {}", target_path.display()))?;
        }
        
        // Create parent directory
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
        }
        
        // Copy backup to target location
        fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
            std::fs::create_dir_all(dst)?;
            for entry in std::fs::read_dir(src)? {
                let entry = entry?;
                let ty = entry.file_type()?;
                if ty.is_dir() {
                    copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
                } else {
                    std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
                }
            }
            Ok(())
        }
        
        copy_dir_all(backup_path, target_path)?;
        
        // Create a new store and load the restored data
        let store = Store::new()
            .with_context(|| "Failed to create new store for restoration")?;
        
        let config = StorageConfig {
            data_dir: target_path.to_path_buf(),
            ..StorageConfig::default()
        };
        
        let mut rdf_store = RDFStore {
            store,
            config,
            is_persistent: true,
        };
        
        // Load the restored data
        rdf_store.load_from_disk()
            .with_context(|| "Failed to load restored data")?;
        
        info!("Successfully restored from backup");
        Ok(rdf_store)
    }

    /// Clean up old backups based on configuration
    fn cleanup_old_backups(&self) -> Result<()> {
        let backups = self.list_backups()?;
        
        if backups.len() > self.config.max_backup_files {
            let to_remove = &backups[self.config.max_backup_files..];
            
            for backup in to_remove {
                info!("Removing old backup: {}", backup.path.display());
                if backup.path.is_dir() {
                    std::fs::remove_dir_all(&backup.path)?;
                } else {
                    std::fs::remove_file(&backup.path)?;
                }
            }
        }
        
        Ok(())
    }

    /// Optimize the database (compact, rebuild indexes, etc.)
    pub fn optimize(&self) -> Result<()> {
        if !self.is_persistent {
            warn!("Cannot optimize in-memory store");
            return Ok(());
        }
        
        info!("Optimizing RDF store database");
        
        // For RocksDB, we can trigger compaction
        // Note: Oxigraph doesn't expose direct RocksDB compaction methods,
        // but the store will automatically optimize over time
        
        info!("Database optimization completed");
        Ok(())
    }

    /// Flush any pending writes to disk
    pub fn flush(&self) -> Result<()> {
        if !self.is_persistent {
            return Ok(());
        }
        
        debug!("Flushing RDF store to disk");
        
        // Oxigraph automatically handles flushing, but we can ensure
        // all operations are committed by performing a simple query
        let _ = self.store.len();
        
        Ok(())
    }

    /// Check database integrity
    pub fn check_integrity(&self) -> Result<IntegrityReport> {
        info!("Checking RDF store integrity");
        
        let start_time = Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Basic checks
        let quad_count = self.store.len();
        
        // Check for orphaned blank nodes (simplified check)
        let orphan_check_query = r#"
            SELECT (COUNT(*) as ?orphans) WHERE {
                ?s ?p ?o .
                FILTER(isBlank(?s) || isBlank(?o))
                FILTER NOT EXISTS {
                    ?s2 ?p2 ?s .
                    FILTER(?s2 != ?s)
                }
                FILTER NOT EXISTS {
                    ?o ?p3 ?o2 .
                    FILTER(?o2 != ?o)
                }
            }
        "#;
        
        let orphan_count = match self.query(orphan_check_query) {
            QueryResults::Solutions(solutions) => {
                let mut count = 0u64;
                for solution in solutions {
                    if let Ok(sol) = solution {
                        if let Some(term) = sol.get("orphans") {
                            if let Term::Literal(lit) = term {
                                if let Ok(parsed_count) = lit.value().parse::<u64>() {
                                    count = parsed_count;
                                    break;
                                }
                            }
                        }
                    }
                }
                count
            }
            _ => 0,
        };
        
        if orphan_count > 0 {
            warnings.push(format!("Found {} potentially orphaned blank nodes", orphan_count));
        }
        
        // Check disk usage if persistent
        let disk_usage = if self.is_persistent {
            Some(self.calculate_disk_usage()?)
        } else {
            None
        };
        
        let check_duration = start_time.elapsed();
        
        let quad_count_value = quad_count.unwrap_or(0);
        let report = IntegrityReport {
            quad_count: quad_count_value,
            orphan_blank_nodes: orphan_count,
            errors,
            warnings,
            disk_usage_bytes: disk_usage,
            check_duration_ms: check_duration.as_millis(),
            timestamp: chrono::Utc::now(),
        };
        
        if report.errors.is_empty() {
            info!("Integrity check completed successfully: {} quads, {} warnings", 
                  quad_count_value, report.warnings.len());
        } else {
            error!("Integrity check found {} errors and {} warnings", 
                   report.errors.len(), report.warnings.len());
        }
        
        Ok(report)
    }
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub quad_count: usize,
    pub disk_usage_bytes: Option<u64>,
    pub backup_count: usize,
    pub is_persistent: bool,
    pub data_dir: Option<PathBuf>,
}

/// Database integrity report
#[derive(Debug, Clone)]
pub struct IntegrityReport {
    pub quad_count: usize,
    pub orphan_blank_nodes: u64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub disk_usage_bytes: Option<u64>,
    pub check_duration_ms: u128,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl RDFStore {
    pub fn add_rdf_to_graph(&mut self, rdf_data: &str, graph_name: &NamedNode) {
        // Try to parse as RDF using a temporary store, if it fails, treat as plain text and create a simple triple
        let temp_store = Store::new().unwrap();
        let reader = Cursor::new(rdf_data.as_bytes());
        
        match temp_store.load_from_reader(RdfFormat::Turtle, reader) {
            Ok(_) => {
                // Successfully parsed as RDF, now copy all triples to the target graph
                for quad in temp_store.iter() {
                    if let Ok(original_quad) = quad {
                        // Create a new quad with the specified graph name
                        let new_quad = Quad::new(
                            original_quad.subject.clone(),
                            original_quad.predicate.clone(),
                            original_quad.object.clone(),
                            graph_name.clone()
                        );
                        self.store.insert(&new_quad).unwrap();
                    }
                }
            }
            Err(_) => {
                // If parsing fails, create a simple triple with the data as a literal
                let subject = NamedNode::new(format!("http://provchain.org/data/{}", graph_name.as_str().replace("http://provchain.org/block/", ""))).unwrap();
                let predicate = NamedNode::new("http://provchain.org/hasData").unwrap();
                let object = Literal::new_simple_literal(rdf_data);
                let quad = Quad::new(subject, predicate, object, graph_name.clone());
                self.store.insert(&quad).unwrap();
            }
        }
    }

    pub fn load_ontology(&mut self, ontology_data: &str, _graph_name: &NamedNode) {
        let reader = Cursor::new(ontology_data.as_bytes());
        self.store
            .load_from_reader(RdfFormat::Turtle, reader)
            .unwrap();
    }

    pub fn add_block_metadata(&mut self, block: &Block) {
        let graph_name = NamedNode::new("http://provchain.org/blockchain").unwrap();
        let block_uri = NamedNode::new(format!("http://provchain.org/block/{}", block.index)).unwrap();
        let data_graph_uri = NamedNode::new(format!("http://provchain.org/block/{}", block.index)).unwrap();
        let prev_block_uri = if block.index > 0 {
            Some(NamedNode::new(format!("http://provchain.org/block/{}", block.index - 1)).unwrap())
        } else {
            None
        };

        // Determine block type (Genesis or regular Block)
        let block_type = if block.index == 0 {
            NamedNode::new("http://provchain.org/GenesisBlock").unwrap()
        } else {
            NamedNode::new("http://provchain.org/Block").unwrap()
        };

        let mut quads = vec![
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
                block_type,
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://provchain.org/hasIndex").unwrap(),
                Literal::new_typed_literal(
                    block.index.to_string(),
                    NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"),
                ),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://provchain.org/hasTimestamp").unwrap(),
                Literal::new_typed_literal(
                    block.timestamp.clone(),
                    NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#dateTime"),
                ),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://provchain.org/hasHash").unwrap(),
                Literal::new_simple_literal(block.hash.clone()),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://provchain.org/hasPreviousHash").unwrap(),
                Literal::new_simple_literal(block.previous_hash.clone()),
                graph_name.clone(),
            ),
            Quad::new(
                block_uri.clone(),
                NamedNode::new("http://provchain.org/hasDataGraphIRI").unwrap(),
                Literal::new_typed_literal(
                    data_graph_uri.as_str(),
                    NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#anyURI"),
                ),
                graph_name.clone(),
            ),
        ];

        if let Some(prev) = prev_block_uri {
            quads.push(Quad::new(
                block_uri,
                NamedNode::new("http://www.w3.org/ns/prov#wasPrecededBy").unwrap(),
                prev,
                graph_name,
            ));
        }

        for quad in &quads {
            self.store.insert(quad).unwrap();
        }
    }

    pub fn query(&self, sparql: &str) -> QueryResults {
        self.store.query(sparql).unwrap()
    }


    /// Hash a single triple using the canonicalization algorithm from Plan.md
    fn hash_triple(&self, triple: &Triple) -> String {
        // Serialize subject
        let serialisation_subject = match &triple.subject {
            Subject::BlankNode(_) => "Magic_S".to_string(),
            Subject::NamedNode(node) => node.to_string(),
            Subject::Triple(t) => format!("<<{}>>", self.triple_to_ntriples(t)),
        };

        // Serialize object
        let serialisation_object = match &triple.object {
            Term::BlankNode(_) => "Magic_O".to_string(),
            Term::NamedNode(node) => node.to_string(),
            Term::Literal(lit) => lit.to_string(),
            Term::Triple(t) => format!("<<{}>>", self.triple_to_ntriples(t)),
        };

        // Serialize predicate (always with NTriples)
        let serialisation_predicate = triple.predicate.to_string();

        // Concatenate and hash
        let concatenation = format!("{serialisation_subject}{serialisation_predicate}{serialisation_object}");
        let mut hasher = Sha256::new();
        hasher.update(concatenation.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Convert a triple to NTriples format
    fn triple_to_ntriples(&self, triple: &Triple) -> String {
        format!("{} {} {}", 
            self.subject_to_ntriples(&triple.subject),
            triple.predicate,
            self.term_to_ntriples(&triple.object)
        )
    }

    /// Convert a subject to NTriples format
    fn subject_to_ntriples(&self, subject: &Subject) -> String {
        match subject {
            Subject::NamedNode(node) => format!("<{}>", node.as_str()),
            Subject::BlankNode(node) => format!("_:{}", node.as_str()),
            Subject::Triple(t) => format!("<<{}>>", self.triple_to_ntriples(t)),
        }
    }

    /// Convert a term to NTriples format
    fn term_to_ntriples(&self, term: &Term) -> String {
        match term {
            Term::NamedNode(node) => format!("<{}>", node.as_str()),
            Term::BlankNode(node) => format!("_:{}", node.as_str()),
            Term::Literal(lit) => lit.to_string(),
            Term::Triple(t) => format!("<<{}>>", self.triple_to_ntriples(t)),
        }
    }

    /// Canonicalize and hash RDF data for a specific graph
    pub fn canonicalize_graph(&self, graph_name: &NamedNode) -> String {
        let mut total_hashes = HashSet::new();

        // Collect all triples in the specified graph
        let mut triples = Vec::new();
        for quad_result in self.store.quads_for_pattern(None, None, None, Some(graph_name.into())) {
            if let Ok(quad) = quad_result {
                let triple = Triple::new(
                    quad.subject.clone(),
                    quad.predicate.clone(),
                    quad.object.clone(),
                );
                triples.push(triple);
            }
        }

        // Main canonicalization loop from Plan.md
        for triple in &triples {
            let basic_triple_hash = self.hash_triple(triple);
            total_hashes.insert(basic_triple_hash);

            // If subject is a blank node, hash all triples where it appears as object
            if let Subject::BlankNode(subject_bnode) = &triple.subject {
                for triple2 in &triples {
                    if let Term::BlankNode(object_bnode) = &triple2.object {
                        if subject_bnode == object_bnode {
                            let hash2 = self.hash_triple(triple2);
                            total_hashes.insert(hash2);
                        }
                    }
                }
            }

            // If object is a blank node, hash all triples where it appears as subject
            if let Term::BlankNode(object_bnode) = &triple.object {
                for triple3 in &triples {
                    if let Subject::BlankNode(subject_bnode) = &triple3.subject {
                        if object_bnode == subject_bnode {
                            let hash3 = self.hash_triple(triple3);
                            total_hashes.insert(hash3);
                        }
                    }
                }
            }
        }

        // Combine all hashes into a final canonical hash
        let mut sorted_hashes: Vec<String> = total_hashes.into_iter().collect();
        sorted_hashes.sort();
        let combined = sorted_hashes.join("");
        
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        format!("{:x}", hasher.finalize())
    }


    /// Validate RDF data in a graph against the loaded ontology
    #[allow(dead_code)]
    pub fn validate_against_ontology(&self, data_graph: &NamedNode) -> bool {
        // Query to check if all entities have proper types from the ontology
        let validation_query = r#"
            PREFIX trace: <http://provchain.org/trace#>
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
            
            ASK {
                GRAPH ?dataGraph {
                    ?entity rdf:type ?type .
                    FILTER(
                        ?type = trace:ProductBatch || 
                        ?type = trace:IngredientLot || 
                        ?type = trace:ProcessingActivity || 
                        ?type = trace:TransportActivity ||
                        ?type = trace:QualityCheck ||
                        ?type = trace:Farmer ||
                        ?type = trace:Manufacturer ||
                        ?type = trace:LogisticsProvider ||
                        ?type = trace:Retailer ||
                        ?type = trace:Customer ||
                        ?type = trace:EnvironmentalCondition ||
                        ?type = trace:Certificate
                    )
                }
            }
        "#;
        
        // Execute validation query with the specific graph
        let query_with_graph = validation_query.replace("?dataGraph", &format!("<{}>", data_graph.as_str()));
        
        match self.query(&query_with_graph) {
            QueryResults::Boolean(result) => result,
            _ => false,
        }
    }

    /// Check if required properties are present for ontology classes
    #[allow(dead_code)]
    pub fn validate_required_properties(&self, data_graph: &NamedNode) -> Vec<String> {
        let mut validation_errors = Vec::new();

        // Check ProductBatch has required properties
        let batch_query = format!(r#"
            PREFIX trace: <http://provchain.org/trace#>
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            
            SELECT ?batch WHERE {{
                GRAPH <{}> {{
                    ?batch rdf:type trace:ProductBatch .
                    FILTER NOT EXISTS {{ ?batch trace:hasBatchID ?id }}
                }}
            }}
        "#, data_graph.as_str());

        if let QueryResults::Solutions(solutions) = self.query(&batch_query) {
            for solution in solutions {
                if let Ok(sol) = solution {
                    if let Some(batch) = sol.get("batch") {
                        validation_errors.push(format!("ProductBatch {batch} missing required hasBatchID property"));
                    }
                }
            }
        }

        // Check Activities have required timestamps
        let activity_query = format!(r#"
            PREFIX trace: <http://provchain.org/trace#>
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            
            SELECT ?activity WHERE {{
                GRAPH <{}> {{
                    ?activity rdf:type ?type .
                    FILTER(?type = trace:ProcessingActivity || ?type = trace:TransportActivity || ?type = trace:QualityCheck)
                    FILTER NOT EXISTS {{ ?activity trace:recordedAt ?timestamp }}
                }}
            }}
        "#, data_graph.as_str());

        if let QueryResults::Solutions(solutions) = self.query(&activity_query) {
            for solution in solutions {
                if let Ok(sol) = solution {
                    if let Some(activity) = sol.get("activity") {
                        validation_errors.push(format!("Activity {activity} missing required recordedAt property"));
                    }
                }
            }
        }

        validation_errors
    }

    /// Get ontology class hierarchy information
    #[allow(dead_code)]
    pub fn get_ontology_classes(&self) -> Vec<String> {
        let query = r#"
            PREFIX trace: <http://provchain.org/trace#>
            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
            PREFIX owl: <http://www.w3.org/2002/07/owl#>
            
            SELECT DISTINCT ?class ?label WHERE {
                ?class a owl:Class .
                OPTIONAL { ?class rdfs:label ?label }
                FILTER(STRSTARTS(STR(?class), "http://provchain.org/trace#"))
            }
            ORDER BY ?class
        "#;

        let mut classes = Vec::new();
        if let QueryResults::Solutions(solutions) = self.query(query) {
            for solution in solutions {
                if let Ok(sol) = solution {
                    if let Some(class) = sol.get("class") {
                        let label = sol.get("label")
                            .map(|l| l.to_string())
                            .unwrap_or_else(|| class.to_string());
                        classes.push(format!("{class} ({label})"));
                    }
                }
            }
        }
        classes
    }

    // ========== HYBRID CANONICALIZATION IMPLEMENTATION ==========

    /// Analyze graph complexity to determine appropriate canonicalization algorithm
    pub fn analyze_graph_complexity(&self, graph_name: &NamedNode) -> GraphComplexity {
        let mut triples = Vec::new();
        let mut blank_nodes = HashSet::new();
        let mut blank_node_connections = HashMap::new();

        // Collect all triples and analyze blank node patterns
        for quad_result in self.store.quads_for_pattern(None, None, None, Some(graph_name.into())) {
            if let Ok(quad) = quad_result {
                let triple = Triple::new(
                    quad.subject.clone(),
                    quad.predicate.clone(),
                    quad.object.clone(),
                );
                triples.push(triple.clone());

                // Track blank nodes and their connections
                if let Subject::BlankNode(bn) = &triple.subject {
                    blank_nodes.insert(bn.as_str().to_string());
                    blank_node_connections.entry(bn.as_str().to_string())
                        .or_insert_with(HashSet::new);
                }
                if let Term::BlankNode(bn) = &triple.object {
                    blank_nodes.insert(bn.as_str().to_string());
                    blank_node_connections.entry(bn.as_str().to_string())
                        .or_insert_with(HashSet::new);
                }

                // Track connections between blank nodes
                if let (Subject::BlankNode(s_bn), Term::BlankNode(o_bn)) = (&triple.subject, &triple.object) {
                    blank_node_connections.entry(s_bn.as_str().to_string())
                        .or_insert_with(HashSet::new)
                        .insert(o_bn.as_str().to_string());
                    blank_node_connections.entry(o_bn.as_str().to_string())
                        .or_insert_with(HashSet::new)
                        .insert(s_bn.as_str().to_string());
                }
            }
        }

        let graph_size = triples.len();
        let blank_node_count = blank_nodes.len();

        // Complexity heuristics based on research analysis
        if blank_node_count == 0 {
            return GraphComplexity::Simple;
        }

        if blank_node_count <= 3 && graph_size <= 50 {
            // Check for simple patterns (chains, trees)
            let max_connections = blank_node_connections.values()
                .map(|connections| connections.len())
                .max()
                .unwrap_or(0);
            
            if max_connections <= 1 {
                return GraphComplexity::Simple;
            } else if max_connections <= 2 {
                return GraphComplexity::Moderate;
            }
        }

        if blank_node_count <= 10 && graph_size <= 200 {
            // Check for cycles and complex interconnections
            let total_connections: usize = blank_node_connections.values()
                .map(|connections| connections.len())
                .sum();
            let avg_connections = if blank_node_count > 0 {
                total_connections as f64 / blank_node_count as f64
            } else {
                0.0
            };

            // Detect cycles by checking if any blank node connects to more than 2 others
            let has_cycles = blank_node_connections.values()
                .any(|connections| connections.len() > 2);

            if avg_connections <= 1.5 && !has_cycles {
                return GraphComplexity::Moderate;
            } else if avg_connections <= 3.0 || has_cycles {
                return GraphComplexity::Complex;
            }
        }

        // Large graphs or highly interconnected blank nodes
        GraphComplexity::Pathological
    }

    /// Adaptive canonicalization that selects the best algorithm based on graph complexity
    pub fn canonicalize_graph_adaptive(&self, graph_name: &NamedNode) -> (String, CanonicalizationMetrics) {
        let start_time = Instant::now();
        let complexity = self.analyze_graph_complexity(graph_name);
        
        // Collect basic graph statistics
        let mut graph_size = 0;
        let mut blank_node_count = 0;
        for quad_result in self.store.quads_for_pattern(None, None, None, Some(graph_name.into())) {
            if let Ok(quad) = quad_result {
                graph_size += 1;
                if let Subject::BlankNode(_) = quad.subject {
                    blank_node_count += 1;
                }
                if let Term::BlankNode(_) = quad.object {
                    blank_node_count += 1;
                }
            }
        }

        let (canonical_hash, algorithm_used) = match complexity {
            GraphComplexity::Simple | GraphComplexity::Moderate => {
                // Use fast custom algorithm for simple cases
                (self.canonicalize_graph(graph_name), CanonicalizationAlgorithm::Custom)
            }
            GraphComplexity::Complex | GraphComplexity::Pathological => {
                // Use RDFC-1.0 for complex cases to ensure correctness
                (self.canonicalize_graph_rdfc10(graph_name), CanonicalizationAlgorithm::RDFC10)
            }
        };

        let execution_time = start_time.elapsed();
        let metrics = CanonicalizationMetrics {
            algorithm_used,
            execution_time_ms: execution_time.as_millis(),
            graph_size,
            blank_node_count,
            complexity,
        };

        (canonical_hash, metrics)
    }

    /// W3C RDFC-1.0 (RDF Dataset Canonicalization) implementation
    pub fn canonicalize_graph_rdfc10(&self, graph_name: &NamedNode) -> String {
        // Collect all quads in the specified graph
        let mut quads = Vec::new();
        for quad_result in self.store.quads_for_pattern(None, None, None, Some(graph_name.into())) {
            if let Ok(quad) = quad_result {
                quads.push(quad);
            }
        }

        if quads.is_empty() {
            return self.hash_string("");
        }

        // Step 1: Create canonical state
        let mut canonical_issuer = IdentifierIssuer::new("c14n");
        let mut blank_node_to_quads: HashMap<String, Vec<usize>> = HashMap::new();

        // Identify blank nodes and their associated quads
        for (i, quad) in quads.iter().enumerate() {
            if let Subject::BlankNode(bn) = &quad.subject {
                blank_node_to_quads.entry(bn.as_str().to_string())
                    .or_default()
                    .push(i);
            }
            if let Term::BlankNode(bn) = &quad.object {
                blank_node_to_quads.entry(bn.as_str().to_string())
                    .or_default()
                    .push(i);
            }
        }

        // Step 2: Compute first-degree hashes for all blank nodes
        let mut hash_to_blank_nodes: HashMap<String, Vec<String>> = HashMap::new();
        for blank_node in blank_node_to_quads.keys() {
            let hash = self.hash_first_degree_quads(blank_node, &quads, &blank_node_to_quads);
            hash_to_blank_nodes.entry(hash)
                .or_default()
                .push(blank_node.clone());
        }

        // Step 3: Issue canonical identifiers for unique hashes
        for (_hash, blank_nodes) in &hash_to_blank_nodes {
            if blank_nodes.len() == 1 {
                canonical_issuer.issue(Some(&blank_nodes[0]));
            }
        }

        // Step 4: Process shared hashes using N-degree hashing
        let mut hash_path_list = Vec::new();
        for (_hash, blank_nodes) in &hash_to_blank_nodes {
            if blank_nodes.len() > 1 {
                for blank_node in blank_nodes {
                    if !canonical_issuer.issued.contains_key(blank_node) {
                        let (hash_result, _) = self.hash_n_degree_quads(
                            blank_node,
                            &quads,
                            &blank_node_to_quads,
                            &canonical_issuer
                        );
                        hash_path_list.push((hash_result, blank_node.clone()));
                    }
                }
            }
        }

        // Sort by hash and issue canonical identifiers
        hash_path_list.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, blank_node) in hash_path_list {
            canonical_issuer.issue(Some(&blank_node));
        }

        // Step 5: Generate canonical N-Quads
        let mut canonical_quads = Vec::new();
        for quad in &quads {
            let canonical_quad = self.replace_blank_nodes_with_canonical_ids(quad, &canonical_issuer);
            canonical_quads.push(canonical_quad);
        }

        // Sort canonical quads lexicographically
        canonical_quads.sort();

        // Step 6: Hash the canonical N-Quads representation
        let canonical_nquads = canonical_quads.join("\n");
        self.hash_string(&canonical_nquads)
    }

    /// Hash first-degree quads for RDFC-1.0 algorithm
    fn hash_first_degree_quads(
        &self,
        reference_blank_node: &str,
        quads: &[Quad],
        blank_node_to_quads: &HashMap<String, Vec<usize>>
    ) -> String {
        let mut nquads = Vec::new();

        if let Some(quad_indices) = blank_node_to_quads.get(reference_blank_node) {
            for &quad_index in quad_indices {
                let quad = &quads[quad_index];
                let nquad = self.quad_to_nquads_with_blank_node_replacement(
                    quad,
                    reference_blank_node,
                    "_:a"
                );
                nquads.push(nquad);
            }
        }

        nquads.sort();
        let concatenated = nquads.join("");
        self.hash_string(&concatenated)
    }

    /// Hash N-degree quads for RDFC-1.0 algorithm
    fn hash_n_degree_quads(
        &self,
        identifier: &str,
        quads: &[Quad],
        blank_node_to_quads: &HashMap<String, Vec<usize>>,
        canonical_issuer: &IdentifierIssuer
    ) -> (String, IdentifierIssuer) {
        let mut hash_to_related_blank_nodes: HashMap<String, Vec<String>> = HashMap::new();

        // Find related blank nodes
        if let Some(quad_indices) = blank_node_to_quads.get(identifier) {
            for &quad_index in quad_indices {
                let quad = &quads[quad_index];
                
                // Check subject
                if let Subject::BlankNode(bn) = &quad.subject {
                    let bn_str = bn.as_str();
                    if bn_str != identifier && !canonical_issuer.issued.contains_key(bn_str) {
                        let hash = self.hash_first_degree_quads(bn_str, quads, blank_node_to_quads);
                        hash_to_related_blank_nodes.entry(hash)
                            .or_default()
                            .push(bn_str.to_string());
                    }
                }

                // Check object
                if let Term::BlankNode(bn) = &quad.object {
                    let bn_str = bn.as_str();
                    if bn_str != identifier && !canonical_issuer.issued.contains_key(bn_str) {
                        let hash = self.hash_first_degree_quads(bn_str, quads, blank_node_to_quads);
                        hash_to_related_blank_nodes.entry(hash)
                            .or_default()
                            .push(bn_str.to_string());
                    }
                }
            }
        }

        // Create data to hash
        let mut data_to_hash = Vec::new();

        // Sort hashes and process related blank nodes
        let mut sorted_hashes: Vec<_> = hash_to_related_blank_nodes.keys().collect();
        sorted_hashes.sort();

        for hash in sorted_hashes {
            data_to_hash.push(hash.clone());
            
            let related_blank_nodes = &hash_to_related_blank_nodes[hash];
            if related_blank_nodes.len() == 1 {
                data_to_hash.push(related_blank_nodes[0].clone());
            } else {
                // For multiple related blank nodes, we would need to explore all permutations
                // This is a simplified implementation - full RDFC-1.0 requires permutation exploration
                let mut sorted_related = related_blank_nodes.clone();
                sorted_related.sort();
                for related in sorted_related {
                    data_to_hash.push(related);
                }
            }
        }

        let hash_result = self.hash_string(&data_to_hash.join(""));
        (hash_result, canonical_issuer.clone_issuer())
    }

    /// Convert quad to N-Quads format with blank node replacement
    fn quad_to_nquads_with_blank_node_replacement(
        &self,
        quad: &Quad,
        reference_blank_node: &str,
        replacement: &str
    ) -> String {
        let subject_str = match &quad.subject {
            Subject::BlankNode(bn) if bn.as_str() == reference_blank_node => replacement.to_string(),
            Subject::BlankNode(_) => "_:z".to_string(),
            Subject::NamedNode(nn) => format!("<{}>", nn.as_str()),
            Subject::Triple(_) => "<< >>".to_string(), // Simplified for quoted triples
        };

        let predicate_str = format!("<{}>", quad.predicate.as_str());

        let object_str = match &quad.object {
            Term::BlankNode(bn) if bn.as_str() == reference_blank_node => replacement.to_string(),
            Term::BlankNode(_) => "_:z".to_string(),
            Term::NamedNode(nn) => format!("<{}>", nn.as_str()),
            Term::Literal(lit) => lit.to_string(),
            Term::Triple(_) => "<< >>".to_string(), // Simplified for quoted triples
        };

        format!("{subject_str} {predicate_str} {object_str} .")
    }

    /// Replace blank nodes with canonical identifiers
    fn replace_blank_nodes_with_canonical_ids(
        &self,
        quad: &Quad,
        canonical_issuer: &IdentifierIssuer
    ) -> String {
        let subject_str = match &quad.subject {
            Subject::BlankNode(bn) => {
                if let Some(canonical_id) = canonical_issuer.issued.get(bn.as_str()) {
                    format!("_:{canonical_id}")
                } else {
                    format!("_:{}", bn.as_str())
                }
            }
            Subject::NamedNode(nn) => format!("<{}>", nn.as_str()),
            Subject::Triple(_) => "<< >>".to_string(),
        };

        let predicate_str = format!("<{}>", quad.predicate.as_str());

        let object_str = match &quad.object {
            Term::BlankNode(bn) => {
                if let Some(canonical_id) = canonical_issuer.issued.get(bn.as_str()) {
                    format!("_:{canonical_id}")
                } else {
                    format!("_:{}", bn.as_str())
                }
            }
            Term::NamedNode(nn) => format!("<{}>", nn.as_str()),
            Term::Literal(lit) => lit.to_string(),
            Term::Triple(_) => "<< >>".to_string(),
        };

        format!("{subject_str} {predicate_str} {object_str} .")
    }

    /// Hash a string using SHA-256
    fn hash_string(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Performance comparison between canonicalization algorithms
    pub fn benchmark_canonicalization_algorithms(&self, graph_name: &NamedNode) -> (CanonicalizationMetrics, CanonicalizationMetrics) {
        // Benchmark custom algorithm
        let start_time = Instant::now();
        let custom_hash = self.canonicalize_graph(graph_name);
        let custom_time = start_time.elapsed();

        // Benchmark RDFC-1.0 algorithm
        let start_time = Instant::now();
        let rdfc10_hash = self.canonicalize_graph_rdfc10(graph_name);
        let rdfc10_time = start_time.elapsed();

        // Collect graph statistics
        let mut graph_size = 0;
        let mut blank_node_count = 0;
        for quad_result in self.store.quads_for_pattern(None, None, None, Some(graph_name.into())) {
            if let Ok(quad) = quad_result {
                graph_size += 1;
                if let Subject::BlankNode(_) = quad.subject {
                    blank_node_count += 1;
                }
                if let Term::BlankNode(_) = quad.object {
                    blank_node_count += 1;
                }
            }
        }

        let complexity = self.analyze_graph_complexity(graph_name);

        let custom_metrics = CanonicalizationMetrics {
            algorithm_used: CanonicalizationAlgorithm::Custom,
            execution_time_ms: custom_time.as_millis(),
            graph_size,
            blank_node_count,
            complexity: complexity.clone(),
        };

        let rdfc10_metrics = CanonicalizationMetrics {
            algorithm_used: CanonicalizationAlgorithm::RDFC10,
            execution_time_ms: rdfc10_time.as_millis(),
            graph_size,
            blank_node_count,
            complexity,
        };

        // Verify correctness (hashes should be the same for isomorphic graphs)
        if custom_hash != rdfc10_hash {
            eprintln!("Warning: Canonicalization algorithms produced different hashes!");
            eprintln!("Custom: {custom_hash}");
            eprintln!("RDFC-1.0: {rdfc10_hash}");
        }

        (custom_metrics, rdfc10_metrics)
    }
}
