//! Domain-specific ontology loading and management

use crate::ontology::error::{OntologyError, ConsistencyError, ValidationError};
use crate::ontology::{OntologyConfig, ShaclValidator};
use oxigraph::store::Store;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Domain configuration for ontology management
#[derive(Debug, Clone)]
pub struct DomainConfig {
    /// Domain name (e.g., "uht_manufacturing", "pharmaceutical")
    pub domain_name: String,
    /// Domain description
    pub description: String,
    /// Supported transaction types for this domain
    pub supported_transaction_types: Vec<String>,
    /// Domain-specific validation rules
    pub validation_rules: HashMap<String, String>,
}

impl DomainConfig {
    /// Create a new domain configuration
    pub fn new(domain_name: String, description: String) -> Self {
        Self {
            domain_name,
            description,
            supported_transaction_types: Vec::new(),
            validation_rules: HashMap::new(),
        }
    }

    /// Add a supported transaction type
    pub fn add_transaction_type(&mut self, transaction_type: String) {
        if !self.supported_transaction_types.contains(&transaction_type) {
            self.supported_transaction_types.push(transaction_type);
        }
    }

    /// Add a validation rule
    pub fn add_validation_rule(&mut self, rule_name: String, rule_value: String) {
        self.validation_rules.insert(rule_name, rule_value);
    }

    /// Check if a transaction type is supported
    pub fn supports_transaction_type(&self, transaction_type: &str) -> bool {
        self.supported_transaction_types.contains(&transaction_type.to_string())
    }
}

/// Ontology manager for domain-specific operations
pub struct OntologyManager {
    /// Current ontology configuration
    pub config: OntologyConfig,
    /// Domain configuration
    pub domain_config: DomainConfig,
    /// SHACL validator
    pub validator: ShaclValidator,
    /// Loaded ontology store
    ontology_store: Store,
}

impl std::fmt::Debug for OntologyManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OntologyManager")
            .field("config", &self.config)
            .field("domain_config", &self.domain_config)
            .field("validator", &self.validator)
            .field("ontology_store", &"<Store>")
            .finish()
    }
}

impl Clone for OntologyManager {
    fn clone(&self) -> Self {
        // Since Store doesn't implement Clone, we need to recreate it
        let ontology_store = Self::load_ontology_store(&self.config)
            .unwrap_or_else(|_| Store::new().unwrap());
        
        OntologyManager {
            config: self.config.clone(),
            domain_config: self.domain_config.clone(),
            validator: self.validator.clone(),
            ontology_store,
        }
    }
}

impl OntologyManager {
    /// Create a new ontology manager
    pub fn new(config: OntologyConfig) -> Result<Self, OntologyError> {
        // Load domain configuration
        let domain_config = Self::load_domain_config(&config)?;
        
        // Create SHACL validator
        let validator = ShaclValidator::new(
            &config.core_shacl_path,
            &config.domain_shacl_path,
            config.ontology_hash.clone(),
        ).map_err(|e| OntologyError::OntologyLoadError {
            path: "SHACL validator".to_string(),
            source: Box::new(e),
        })?;

        // Load ontology into store
        let ontology_store = Self::load_ontology_store(&config)?;

        Ok(OntologyManager {
            config,
            domain_config,
            validator,
            ontology_store,
        })
    }

    /// Load domain configuration from ontology
    fn load_domain_config(config: &OntologyConfig) -> Result<DomainConfig, OntologyError> {
        let domain_name = config.domain_name()?;
        
        // Create domain configuration based on the ontology
        let mut domain_config = DomainConfig::new(
            domain_name.clone(),
            format!("Domain configuration for {}", domain_name),
        );

        // Add standard transaction types
        let standard_types = vec![
            "Production".to_string(),
            "Processing".to_string(),
            "Transport".to_string(),
            "Quality".to_string(),
            "Transfer".to_string(),
            "Environmental".to_string(),
            "Compliance".to_string(),
            "Governance".to_string(),
        ];

        for tx_type in standard_types {
            domain_config.add_transaction_type(tx_type);
        }

        // Load domain-specific configuration from ontology file
        if let Ok(ontology_content) = fs::read_to_string(&config.domain_ontology_path) {
            // Extract domain-specific information from ontology comments or annotations
            Self::extract_domain_info_from_ontology(&mut domain_config, &ontology_content)?;
        }

        Ok(domain_config)
    }

    /// Extract domain information from ontology content
    fn extract_domain_info_from_ontology(
        domain_config: &mut DomainConfig,
        ontology_content: &str,
    ) -> Result<(), OntologyError> {
        // Look for domain-specific annotations in the ontology
        // This is a simplified implementation - in practice, you'd parse RDF properly
        
        // Extract description from rdfs:comment
        if let Some(comment_start) = ontology_content.find("rdfs:comment") {
            if let Some(quote_start) = ontology_content[comment_start..].find('"') {
                let quote_start = comment_start + quote_start + 1;
                if let Some(quote_end) = ontology_content[quote_start..].find('"') {
                    let description = &ontology_content[quote_start..quote_start + quote_end];
                    domain_config.description = description.to_string();
                }
            }
        }

        // Look for domain-specific transaction types in annotations
        for line in ontology_content.lines() {
            if line.contains("# Transaction type:") {
                if let Some(tx_type) = line.split("# Transaction type:").nth(1) {
                    domain_config.add_transaction_type(tx_type.trim().to_string());
                }
            }
            
            // Look for validation rules
            if line.contains("# Validation rule:") {
                if let Some(rule_part) = line.split("# Validation rule:").nth(1) {
                    if let Some((rule_name, rule_value)) = rule_part.split_once('=') {
                        domain_config.add_validation_rule(
                            rule_name.trim().to_string(),
                            rule_value.trim().to_string(),
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// Load ontology into an RDF store
    fn load_ontology_store(config: &OntologyConfig) -> Result<Store, OntologyError> {
        let store = Store::new()
            .map_err(|e| OntologyError::OntologyLoadError {
                path: "RDF store creation".to_string(),
                source: Box::new(e),
            })?;

        // Load core ontology
        if Path::new(&config.core_ontology_path).exists() {
            let core_content = fs::read_to_string(&config.core_ontology_path)
                .map_err(|e| OntologyError::OntologyLoadError {
                    path: config.core_ontology_path.clone(),
                    source: Box::new(e),
                })?;

            let format = Self::detect_rdf_format(&core_content, &config.core_ontology_path)?;
            use std::io::Cursor;
            let reader = Cursor::new(core_content.as_bytes());
            store.load_from_reader(
                format,
                reader,
            ).map_err(|e| OntologyError::OntologyParseError {
                path: config.core_ontology_path.clone(),
                message: format!("Failed to parse core ontology: {}", e),
            })?;
        }

        // Load domain ontology
        let domain_content = fs::read_to_string(&config.domain_ontology_path)
            .map_err(|e| OntologyError::OntologyLoadError {
                path: config.domain_ontology_path.clone(),
                source: Box::new(e),
            })?;

        let format = Self::detect_rdf_format(&domain_content, &config.domain_ontology_path)?;
        use std::io::Cursor;
        let reader = Cursor::new(domain_content.as_bytes());
        store.load_from_reader(
            format,
            reader,
        ).map_err(|e| OntologyError::OntologyParseError {
            path: config.domain_ontology_path.clone(),
            message: format!("Failed to parse domain ontology: {}", e),
        })?;

        Ok(store)
    }

    /// Detect RDF format from content and file extension
    fn detect_rdf_format(content: &str, file_path: &str) -> Result<oxigraph::io::RdfFormat, OntologyError> {
        // First, try to detect from content
        let trimmed_content = content.trim();
        
        // Check for Turtle format indicators
        if trimmed_content.starts_with("@prefix") || 
           trimmed_content.starts_with("@base") ||
           content.contains("@prefix") {
            return Ok(oxigraph::io::RdfFormat::Turtle);
        }
        
        // Check for RDF/XML format indicators
        if trimmed_content.starts_with("<?xml") ||
           trimmed_content.starts_with("<rdf:RDF") ||
           content.contains("<rdf:RDF") {
            return Ok(oxigraph::io::RdfFormat::RdfXml);
        }
        
        // Check for N-Triples format indicators
        if content.lines().all(|line| {
            let line = line.trim();
            line.is_empty() || line.starts_with('#') || line.ends_with(" .")
        }) {
            return Ok(oxigraph::io::RdfFormat::NTriples);
        }
        
        // Fall back to file extension detection
        let path = Path::new(file_path);
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            match extension.to_lowercase().as_str() {
                "ttl" | "turtle" => Ok(oxigraph::io::RdfFormat::Turtle),
                "owl" | "rdf" | "xml" => {
                    // For .owl files, default to Turtle since many are actually in Turtle format
                    // but try RDF/XML if content suggests it
                    if content.contains("<?xml") || content.contains("<rdf:RDF") {
                        Ok(oxigraph::io::RdfFormat::RdfXml)
                    } else {
                        Ok(oxigraph::io::RdfFormat::Turtle)
                    }
                },
                "nt" => Ok(oxigraph::io::RdfFormat::NTriples),
                "nq" => Ok(oxigraph::io::RdfFormat::NQuads),
                _ => Ok(oxigraph::io::RdfFormat::Turtle), // Default to Turtle
            }
        } else {
            // No extension, default to Turtle
            Ok(oxigraph::io::RdfFormat::Turtle)
        }
    }

    /// Load domain ontology from file path
    pub fn load_domain_ontology(ontology_path: &str) -> Result<OntologyConfig, OntologyError> {
        // Validate file exists
        if !Path::new(ontology_path).exists() {
            return Err(OntologyError::OntologyNotFound {
                path: ontology_path.to_string(),
            });
        }

        // Create configuration from the ontology path
        let config = crate::config::Config::default();
        OntologyConfig::new(Some(ontology_path.to_string()), &config)
    }

    /// Check ontology consistency across network participants
    pub fn check_ontology_consistency(
        &self,
        network_hash: &str,
    ) -> Result<(), ConsistencyError> {
        if self.config.ontology_hash != network_hash {
            return Err(ConsistencyError::new(
                self.config.ontology_hash.clone(),
                network_hash.to_string(),
                format!(
                    "Local ontology '{}' does not match network ontology. All participants must use the same domain ontology.",
                    self.domain_config.domain_name
                ),
            ));
        }
        Ok(())
    }

    /// Validate transaction data using SHACL
    pub fn validate_transaction(&self, rdf_data: &str) -> Result<crate::ontology::error::ValidationResult, ValidationError> {
        self.validator.validate_transaction(rdf_data)
    }

    /// Get ontology hash for network consistency checking
    pub fn get_ontology_hash(&self) -> &str {
        &self.config.ontology_hash
    }

    /// Get domain name
    pub fn get_domain_name(&self) -> &str {
        &self.domain_config.domain_name
    }

    /// Get supported transaction types
    pub fn get_supported_transaction_types(&self) -> &[String] {
        &self.domain_config.supported_transaction_types
    }

    /// Query the ontology store
    pub fn query_ontology(&self, sparql_query: &str) -> Result<String, OntologyError> {
        use oxigraph::sparql::QueryResults;
        
        let results = self.ontology_store.query(sparql_query)
            .map_err(|e| OntologyError::OntologyLoadError {
                path: "SPARQL query".to_string(),
                source: Box::new(e),
            })?;

        // Convert query results to string representation
        match results {
            QueryResults::Solutions(solutions) => {
                let mut result_string = String::new();
                for solution in solutions {
                    let solution = solution.map_err(|e| OntologyError::OntologyLoadError {
                        path: "SPARQL solution".to_string(),
                        source: Box::new(e),
                    })?;
                    result_string.push_str(&format!("{:?}\n", solution));
                }
                Ok(result_string)
            }
            QueryResults::Graph(quads) => {
                let mut result_string = String::new();
                for quad in quads {
                    let quad = quad.map_err(|e| OntologyError::OntologyLoadError {
                        path: "SPARQL quad".to_string(),
                        source: Box::new(e),
                    })?;
                    result_string.push_str(&format!("{}\n", quad));
                }
                Ok(result_string)
            }
            QueryResults::Boolean(boolean) => {
                Ok(boolean.to_string())
            }
        }
    }

    /// Get ontology statistics
    pub fn get_ontology_stats(&self) -> Result<OntologyStats, OntologyError> {
        let mut stats = OntologyStats::new();

        // Count classes
        let class_query = r#"
            SELECT (COUNT(DISTINCT ?class) AS ?count) WHERE {
                ?class a <http://www.w3.org/2002/07/owl#Class> .
            }
        "#;
        
        if let Ok(result) = self.query_ontology(class_query) {
            // Parse count from result (simplified)
            if let Some(count_str) = result.lines().next() {
                if let Ok(count) = count_str.trim().parse::<u32>() {
                    stats.class_count = count;
                }
            }
        }

        // Count properties
        let property_query = r#"
            SELECT (COUNT(DISTINCT ?property) AS ?count) WHERE {
                { ?property a <http://www.w3.org/2002/07/owl#ObjectProperty> } UNION
                { ?property a <http://www.w3.org/2002/07/owl#DatatypeProperty> }
            }
        "#;
        
        if let Ok(result) = self.query_ontology(property_query) {
            if let Some(count_str) = result.lines().next() {
                if let Ok(count) = count_str.trim().parse::<u32>() {
                    stats.property_count = count;
                }
            }
        }

        // Count individuals
        let individual_query = r#"
            SELECT (COUNT(DISTINCT ?individual) AS ?count) WHERE {
                ?individual a ?class .
                ?class a <http://www.w3.org/2002/07/owl#Class> .
            }
        "#;
        
        if let Ok(result) = self.query_ontology(individual_query) {
            if let Some(count_str) = result.lines().next() {
                if let Ok(count) = count_str.trim().parse::<u32>() {
                    stats.individual_count = count;
                }
            }
        }

        Ok(stats)
    }

    /// Reload ontology configuration
    pub fn reload(&mut self) -> Result<(), OntologyError> {
        // Reload domain configuration
        self.domain_config = Self::load_domain_config(&self.config)?;
        
        // Recreate SHACL validator
        self.validator = ShaclValidator::new(
            &self.config.core_shacl_path,
            &self.config.domain_shacl_path,
            self.config.ontology_hash.clone(),
        ).map_err(|e| OntologyError::OntologyLoadError {
            path: "SHACL validator reload".to_string(),
            source: Box::new(e),
        })?;

        // Reload ontology store
        self.ontology_store = Self::load_ontology_store(&self.config)?;

        Ok(())
    }
}

/// Statistics about the loaded ontology
#[derive(Debug, Clone, Default)]
pub struct OntologyStats {
    /// Number of classes in the ontology
    pub class_count: u32,
    /// Number of properties in the ontology
    pub property_count: u32,
    /// Number of individuals in the ontology
    pub individual_count: u32,
}

impl OntologyStats {
    /// Create new empty statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Get total entity count
    pub fn total_entities(&self) -> u32 {
        self.class_count + self.property_count + self.individual_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_domain_config_creation() {
        let mut config = DomainConfig::new(
            "test_domain".to_string(),
            "Test domain description".to_string(),
        );

        assert_eq!(config.domain_name, "test_domain");
        assert_eq!(config.description, "Test domain description");
        assert!(config.supported_transaction_types.is_empty());

        config.add_transaction_type("Production".to_string());
        assert!(config.supports_transaction_type("Production"));
        assert!(!config.supports_transaction_type("Unknown"));
    }

    #[test]
    fn test_domain_config_validation_rules() {
        let mut config = DomainConfig::new(
            "test_domain".to_string(),
            "Test domain".to_string(),
        );

        config.add_validation_rule("min_temperature".to_string(), "0".to_string());
        config.add_validation_rule("max_temperature".to_string(), "100".to_string());

        assert_eq!(config.validation_rules.get("min_temperature"), Some(&"0".to_string()));
        assert_eq!(config.validation_rules.get("max_temperature"), Some(&"100".to_string()));
    }

    #[test]
    fn test_load_domain_ontology() {
        let temp_dir = TempDir::new().unwrap();
        let ontology_path = temp_dir.path().join("test_ontology.owl");
        
        // Create a minimal OWL ontology file
        let owl_content = r#"<?xml version="1.0"?>
<rdf:RDF xmlns="http://example.org/test#"
         xml:base="http://example.org/test"
         xmlns:owl="http://www.w3.org/2002/07/owl#"
         xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
    <owl:Ontology rdf:about="http://example.org/test">
        <rdfs:comment>Test ontology for domain management</rdfs:comment>
    </owl:Ontology>
</rdf:RDF>"#;
        
        fs::write(&ontology_path, owl_content).unwrap();
        
        let result = OntologyManager::load_domain_ontology(&ontology_path.to_string_lossy());
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq!(config.domain_ontology_path, ontology_path.to_string_lossy());
    }

    #[test]
    fn test_ontology_not_found() {
        let result = OntologyManager::load_domain_ontology("nonexistent/ontology.owl");
        assert!(result.is_err());
        
        match result.unwrap_err() {
            OntologyError::OntologyNotFound { path } => {
                assert_eq!(path, "nonexistent/ontology.owl");
            }
            _ => panic!("Expected OntologyNotFound error"),
        }
    }

    #[test]
    fn test_ontology_stats() {
        let stats = OntologyStats {
            class_count: 10,
            property_count: 20,
            individual_count: 5,
        };

        assert_eq!(stats.total_entities(), 35);
    }
}
