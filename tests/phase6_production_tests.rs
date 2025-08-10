//! Phase 6: Production Deployment Tests
//! 
//! This module contains comprehensive tests for the production deployment features
//! including container orchestration, monitoring, security, compliance, and deployment automation.

use provchain_org::production::{
    self, ProductionConfig, ProductionManager, ProductionError,
    container, monitoring, security, compliance, deployment
};
use std::collections::HashMap;
use tokio;

#[tokio::test]
async fn test_production_manager_initialization() {
    let config = ProductionConfig::default();
    let mut manager = ProductionManager::new(config).expect("Failed to create production manager");
    
    // Test initialization
    let result = manager.initialize().await;
    assert!(result.is_ok(), "Production manager initialization should succeed");
    
    // Test health check
    let health = manager.health_check().await;
    assert!(health.contains_key("environment"));
    assert!(health.contains_key("version"));
    assert!(health.contains_key("monitoring"));
    assert!(health.contains_key("security"));
    assert!(health.contains_key("compliance"));
    
    // Test shutdown
    let shutdown_result = manager.shutdown().await;
    assert!(shutdown_result.is_ok(), "Production manager shutdown should succeed");
}

#[tokio::test]
async fn test_container_manager() {
    let config = container::ContainerConfig::default();
    let manager = container::ContainerManager::new(config);
    
    // Test Dockerfile generation
    let dockerfile = manager.generate_dockerfile();
    assert!(dockerfile.contains("FROM rust:1.75 as builder"));
    assert!(dockerfile.contains("WORKDIR /app"));
    assert!(dockerfile.contains("COPY Cargo.toml Cargo.lock"));
    assert!(dockerfile.contains("EXPOSE"));
    assert!(dockerfile.contains("HEALTHCHECK"));
    
    // Test Docker Compose generation
    let compose = manager.generate_docker_compose();
    assert!(compose.contains("version: '3.8'"));
    assert!(compose.contains("services:"));
    assert!(compose.contains("provchain:"));
    assert!(compose.contains("prometheus:"));
    assert!(compose.contains("grafana:"));
    
    // Test Kubernetes deployment generation
    let k8s = manager.generate_kubernetes_deployment();
    assert!(k8s.contains("apiVersion: apps/v1"));
    assert!(k8s.contains("kind: Deployment"));
    assert!(k8s.contains("replicas: 3"));
    assert!(k8s.contains("livenessProbe"));
    assert!(k8s.contains("readinessProbe"));
    
    // Test Helm values generation
    let helm = manager.generate_helm_values();
    assert!(helm.contains("replicaCount: 3"));
    assert!(helm.contains("image:"));
    assert!(helm.contains("autoscaling:"));
    assert!(helm.contains("monitoring:"));
}

#[tokio::test]
async fn test_monitoring_manager() {
    let config = monitoring::MonitoringConfig::default();
    let mut manager = monitoring::MonitoringManager::new(config).expect("Failed to create monitoring manager");
    
    // Test monitoring start
    let result = manager.start().await;
    assert!(result.is_ok(), "Monitoring should start successfully");
    
    // Test status
    let status = manager.status().await;
    assert!(status.contains("CPU"));
    assert!(status.contains("Memory"));
    assert!(status.contains("Uptime"));
    
    // Test metrics collection
    let (sys_metrics, app_metrics) = manager.get_metrics().await;
    assert_eq!(sys_metrics.cpu_usage_percent, 0.0); // Default value in test
    assert_eq!(app_metrics.blockchain_blocks, 0); // Default value in test
    
    // Test Prometheus config generation
    let prometheus_config = manager.generate_prometheus_config();
    assert!(prometheus_config.contains("global:"));
    assert!(prometheus_config.contains("scrape_configs:"));
    assert!(prometheus_config.contains("alerting_rules:"));
    
    // Test Grafana dashboard generation
    let grafana_dashboard = manager.generate_grafana_dashboard();
    assert!(grafana_dashboard.contains("ProvChain Monitoring Dashboard"));
    assert!(grafana_dashboard.contains("panels"));
    
    // Test stop
    let stop_result = manager.stop().await;
    assert!(stop_result.is_ok(), "Monitoring should stop successfully");
}

#[tokio::test]
async fn test_security_manager() {
    let config = security::SecurityConfig::default();
    let mut manager = security::SecurityManager::new(config).expect("Failed to create security manager");
    
    // Test initialization
    let result = manager.initialize().await;
    assert!(result.is_ok(), "Security manager should initialize successfully");
    
    // Test audit event logging
    let audit_event = security::SecurityAuditEvent {
        timestamp: chrono::Utc::now(),
        event_type: security::AuditEventType::Authentication,
        user_id: Some("test_user".to_string()),
        ip_address: Some("127.0.0.1".to_string()),
        user_agent: Some("test_agent".to_string()),
        resource: "/api/test".to_string(),
        action: "login".to_string(),
        result: security::AuditResult::Success,
        details: HashMap::new(),
    };
    
    let log_result = manager.log_audit_event(audit_event).await;
    assert!(log_result.is_ok(), "Audit event logging should succeed");
    
    // Test status
    let status = manager.status().await;
    assert!(status.contains("TLS"));
    assert!(status.contains("Rate Limiting"));
    assert!(status.contains("Audit Events"));
    
    // Test recent audit events
    let recent_events = manager.get_recent_audit_events(10).await;
    assert_eq!(recent_events.len(), 1);
    
    // Test Nginx config generation
    let nginx_config = manager.generate_nginx_config();
    assert!(nginx_config.contains("server {"));
    assert!(nginx_config.contains("ssl_certificate"));
    assert!(nginx_config.contains("limit_req_zone"));
    
    // Test firewall rules generation
    let firewall_rules = manager.generate_firewall_rules();
    assert!(firewall_rules.contains("iptables"));
    assert!(firewall_rules.contains("INPUT"));
    assert!(firewall_rules.contains("OUTPUT"));
    
    // Test security report generation
    let report = manager.generate_security_report().await;
    assert!(report.contains("ProvChain Security Audit Report"));
    assert!(report.contains("Summary"));
    assert!(report.contains("Recommendations"));
    
    // Test shutdown
    let shutdown_result = manager.shutdown().await;
    assert!(shutdown_result.is_ok(), "Security manager should shutdown successfully");
}

#[tokio::test]
async fn test_security_middleware() {
    let config = security::SecurityConfig::default();
    let middleware = security::SecurityMiddleware::new(config);
    
    // Test request header validation
    let headers = HashMap::new();
    let validation_result = middleware.validate_request_headers(&headers);
    assert!(validation_result.is_ok(), "Header validation should succeed");
    
    // Test rate limiting
    let rate_limit_result = middleware.check_rate_limit("127.0.0.1");
    assert!(rate_limit_result.is_ok(), "Rate limit check should succeed");
    assert!(rate_limit_result.unwrap(), "Rate limit should allow request");
    
    // Test JWT validation
    let jwt_result = middleware.validate_jwt("test_token");
    assert!(jwt_result.is_ok(), "JWT validation should succeed");
    assert!(jwt_result.unwrap(), "JWT should be valid");
    
    // Test empty JWT
    let empty_jwt_result = middleware.validate_jwt("");
    assert!(empty_jwt_result.is_ok(), "Empty JWT validation should succeed");
    assert!(!empty_jwt_result.unwrap(), "Empty JWT should be invalid");
}

#[tokio::test]
async fn test_compliance_manager() {
    let config = compliance::ComplianceConfig::default();
    let mut manager = compliance::ComplianceManager::new(config).expect("Failed to create compliance manager");
    
    // Test initialization
    let result = manager.initialize().await;
    assert!(result.is_ok(), "Compliance manager should initialize successfully");
    
    // Test data item registration
    let data_item = compliance::DataInventoryItem {
        id: "test_item_1".to_string(),
        data_type: compliance::DataClassification::Personal,
        created_at: chrono::Utc::now(),
        last_accessed: chrono::Utc::now(),
        retention_until: chrono::Utc::now() + chrono::Duration::days(365),
        anonymized: false,
        consent_status: Some(compliance::ConsentStatus::Given),
    };
    
    let register_result = manager.register_data_item(data_item).await;
    assert!(register_result.is_ok(), "Data item registration should succeed");
    
    // Test compliance event logging
    let compliance_event = compliance::ComplianceAuditEvent {
        timestamp: chrono::Utc::now(),
        event_type: compliance::ComplianceEventType::DataAccess,
        regulation: compliance::ComplianceRegulation::GDPR,
        data_subject: Some("test_subject".to_string()),
        data_type: compliance::DataClassification::Personal,
        action: "read".to_string(),
        compliance_status: compliance::ComplianceStatus::Compliant,
        details: HashMap::new(),
    };
    
    let log_result = manager.log_compliance_event(compliance_event).await;
    assert!(log_result.is_ok(), "Compliance event logging should succeed");
    
    // Test GDPR requests
    let gdpr_access_result = manager.handle_gdpr_request(
        compliance::GdprRequestType::DataAccess,
        "test_item"
    ).await;
    assert!(gdpr_access_result.is_ok(), "GDPR data access request should succeed");
    
    let gdpr_deletion_result = manager.handle_gdpr_request(
        compliance::GdprRequestType::DataDeletion,
        "test_item"
    ).await;
    assert!(gdpr_deletion_result.is_ok(), "GDPR data deletion request should succeed");
    
    let gdpr_portability_result = manager.handle_gdpr_request(
        compliance::GdprRequestType::DataPortability,
        "test_item"
    ).await;
    assert!(gdpr_portability_result.is_ok(), "GDPR data portability request should succeed");
    
    // Test status
    let status = manager.status().await;
    assert!(status.contains("GDPR"));
    assert!(status.contains("FDA"));
    assert!(status.contains("Events"));
    
    // Test compliance report generation
    let report = manager.generate_compliance_report().await;
    assert!(report.contains("ProvChain Compliance Report"));
    assert!(report.contains("Configuration"));
    assert!(report.contains("Recommendations"));
    
    // Test DPA template generation
    let dpa = manager.generate_dpa_template();
    assert!(dpa.contains("DATA PROCESSING AGREEMENT"));
    assert!(dpa.contains("PARTIES"));
    assert!(dpa.contains("SCOPE AND PURPOSE"));
    
    // Test shutdown
    let shutdown_result = manager.shutdown().await;
    assert!(shutdown_result.is_ok(), "Compliance manager should shutdown successfully");
}

#[tokio::test]
async fn test_compliance_checker() {
    let config = compliance::ComplianceConfig::default();
    let checker = compliance::ComplianceChecker::new(config);
    
    // Test data operation compliance checks
    let personal_read = checker.check_data_operation("read", &compliance::DataClassification::Personal);
    assert!(personal_read.is_ok(), "Personal data read should be allowed");
    assert!(personal_read.unwrap(), "Personal data read should be compliant");
    
    let confidential_export = checker.check_data_operation("export", &compliance::DataClassification::Confidential);
    assert!(confidential_export.is_ok(), "Confidential data export check should succeed");
    assert!(!confidential_export.unwrap(), "Confidential data export should not be allowed");
    
    let public_read = checker.check_data_operation("read", &compliance::DataClassification::Public);
    assert!(public_read.is_ok(), "Public data read should be allowed");
    assert!(public_read.unwrap(), "Public data read should be compliant");
    
    // Test data classification
    let email_classification = checker.classify_data("user email: test@example.com");
    assert!(matches!(email_classification, compliance::DataClassification::Personal));
    
    let product_classification = checker.classify_data("product information: widget");
    assert!(matches!(product_classification, compliance::DataClassification::Business));
    
    let unknown_classification = checker.classify_data("random data");
    assert!(matches!(unknown_classification, compliance::DataClassification::Public));
}

#[tokio::test]
async fn test_deployment_manager() {
    let config = deployment::DeploymentConfig::default();
    let manager = deployment::DeploymentManager::new(config);
    
    // Test deployment
    let deployment_result = manager.deploy("v1.0.0".to_string(), "test_user".to_string()).await;
    assert!(deployment_result.is_ok(), "Deployment should succeed");
    
    let deployment_id = deployment_result.unwrap();
    assert!(!deployment_id.is_empty(), "Deployment ID should not be empty");
    
    // Wait for deployment to complete
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    
    // Test deployment status
    let status = manager.get_deployment_status(&deployment_id).await;
    assert!(status.is_some(), "Deployment status should be available");
    
    let deployment_record = status.unwrap();
    assert_eq!(deployment_record.version, "v1.0.0");
    assert!(matches!(deployment_record.status, deployment::DeploymentStatus::Completed));
    
    // Test deployment history
    let history = manager.get_deployment_history(Some(10)).await;
    assert!(!history.is_empty(), "Deployment history should not be empty");
    
    // Test rollback
    let rollback_result = manager.rollback(None).await;
    assert!(rollback_result.is_ok(), "Rollback should succeed");
    
    // Test deployment report generation
    let report = manager.generate_deployment_report().await;
    assert!(report.contains("ProvChain Deployment Report"));
    assert!(report.contains("Deployment Statistics"));
    assert!(report.contains("Recommendations"));
    
    // Test CI/CD pipeline generation
    let cicd = manager.generate_cicd_pipeline();
    assert!(cicd.contains("name: ProvChain Deployment Pipeline"));
    assert!(cicd.contains("jobs:"));
    assert!(cicd.contains("test:"));
    assert!(cicd.contains("build:"));
    assert!(cicd.contains("deploy-production:"));
}

#[tokio::test]
async fn test_deployment_strategies() {
    // Test Blue-Green deployment
    let mut config = deployment::DeploymentConfig::default();
    config.strategy = deployment::DeploymentStrategy::BlueGreen;
    let manager = deployment::DeploymentManager::new(config);
    
    let deployment_result = manager.deploy("v1.1.0".to_string(), "test_user".to_string()).await;
    assert!(deployment_result.is_ok(), "Blue-Green deployment should succeed");
    
    // Test Rolling Update deployment
    let mut config = deployment::DeploymentConfig::default();
    config.strategy = deployment::DeploymentStrategy::RollingUpdate;
    let manager = deployment::DeploymentManager::new(config);
    
    let deployment_result = manager.deploy("v1.2.0".to_string(), "test_user".to_string()).await;
    assert!(deployment_result.is_ok(), "Rolling Update deployment should succeed");
    
    // Test Canary deployment
    let mut config = deployment::DeploymentConfig::default();
    config.strategy = deployment::DeploymentStrategy::Canary;
    let manager = deployment::DeploymentManager::new(config);
    
    let deployment_result = manager.deploy("v1.3.0".to_string(), "test_user".to_string()).await;
    assert!(deployment_result.is_ok(), "Canary deployment should succeed");
    
    // Test Recreate deployment
    let mut config = deployment::DeploymentConfig::default();
    config.strategy = deployment::DeploymentStrategy::Recreate;
    let manager = deployment::DeploymentManager::new(config);
    
    let deployment_result = manager.deploy("v1.4.0".to_string(), "test_user".to_string()).await;
    assert!(deployment_result.is_ok(), "Recreate deployment should succeed");
}

#[tokio::test]
async fn test_metrics_recorder() {
    let recorder = monitoring::MetricsRecorder::new();
    
    // Test request recording
    recorder.record_request(150.0, false).await;
    recorder.record_request(200.0, true).await;
    recorder.record_request(100.0, false).await;
    
    // Test stats retrieval
    let (request_count, error_count, avg_response_time) = recorder.get_stats().await;
    assert_eq!(request_count, 3);
    assert_eq!(error_count, 1);
    assert!((avg_response_time - 150.0).abs() < 1.0); // Should be approximately 150ms
}

#[tokio::test]
async fn test_production_config_serialization() {
    let config = ProductionConfig::default();
    
    // Test serialization
    let serialized = serde_json::to_string(&config);
    assert!(serialized.is_ok(), "Production config should serialize successfully");
    
    // Test deserialization
    let json_str = serialized.unwrap();
    let deserialized: Result<ProductionConfig, _> = serde_json::from_str(&json_str);
    assert!(deserialized.is_ok(), "Production config should deserialize successfully");
    
    let restored_config = deserialized.unwrap();
    assert_eq!(config.environment, restored_config.environment);
    assert_eq!(config.version, restored_config.version);
}

#[tokio::test]
async fn test_container_file_generation() {
    use std::path::PathBuf;
    use tempfile::TempDir;
    
    let config = container::ContainerConfig::default();
    let manager = container::ContainerManager::new(config);
    
    // Create temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().to_path_buf();
    
    // Test file generation
    let result = manager.write_container_files(&output_path).await;
    assert!(result.is_ok(), "Container file generation should succeed");
    
    // Verify files were created
    assert!(output_path.join("Dockerfile").exists());
    assert!(output_path.join("docker-compose.yml").exists());
    assert!(output_path.join("kubernetes-deployment.yaml").exists());
    assert!(output_path.join("helm-values.yaml").exists());
}

#[tokio::test]
async fn test_health_check_configuration() {
    let mut config = deployment::HealthCheckConfig::default();
    config.endpoint = "/custom-health".to_string();
    config.interval_seconds = 15;
    config.timeout_seconds = 5;
    config.healthy_threshold = 3;
    config.unhealthy_threshold = 2;
    
    assert_eq!(config.endpoint, "/custom-health");
    assert_eq!(config.interval_seconds, 15);
    assert_eq!(config.timeout_seconds, 5);
    assert_eq!(config.healthy_threshold, 3);
    assert_eq!(config.unhealthy_threshold, 2);
}

#[tokio::test]
async fn test_auto_scaling_configuration() {
    let mut config = deployment::AutoScalingConfig::default();
    config.min_replicas = 2;
    config.max_replicas = 20;
    config.target_cpu_utilization = 60.0;
    config.target_memory_utilization = 75.0;
    
    assert_eq!(config.min_replicas, 2);
    assert_eq!(config.max_replicas, 20);
    assert_eq!(config.target_cpu_utilization, 60.0);
    assert_eq!(config.target_memory_utilization, 75.0);
    assert!(config.enabled);
}

#[tokio::test]
async fn test_backup_configuration() {
    let mut config = deployment::BackupConfig::default();
    config.schedule = "0 3 * * *".to_string(); // Daily at 3 AM
    config.retention_days = 60;
    config.backup_location = "/custom/backup/path".to_string();
    
    assert_eq!(config.schedule, "0 3 * * *");
    assert_eq!(config.retention_days, 60);
    assert_eq!(config.backup_location, "/custom/backup/path");
    assert!(config.enabled);
    assert!(config.encryption_enabled);
}

#[tokio::test]
async fn test_load_balancer_configuration() {
    let mut config = deployment::LoadBalancerConfig::default();
    config.algorithm = deployment::LoadBalancingAlgorithm::LeastConnections;
    config.session_affinity = true;
    config.timeout_seconds = 120;
    
    assert!(matches!(config.algorithm, deployment::LoadBalancingAlgorithm::LeastConnections));
    assert!(config.session_affinity);
    assert_eq!(config.timeout_seconds, 120);
    assert!(config.health_check_enabled);
}

#[test]
fn test_production_error_types() {
    let config_error = ProductionError::Configuration("Test config error".to_string());
    assert!(config_error.to_string().contains("Configuration error"));
    
    let monitoring_error = ProductionError::Monitoring("Test monitoring error".to_string());
    assert!(monitoring_error.to_string().contains("Monitoring error"));
    
    let security_error = ProductionError::Security("Test security error".to_string());
    assert!(security_error.to_string().contains("Security error"));
    
    let compliance_error = ProductionError::Compliance("Test compliance error".to_string());
    assert!(compliance_error.to_string().contains("Compliance error"));
    
    let deployment_error = ProductionError::Deployment("Test deployment error".to_string());
    assert!(deployment_error.to_string().contains("Deployment error"));
}

#[test]
fn test_data_classification_levels() {
    use compliance::DataClassification;
    
    let public = DataClassification::Public;
    let internal = DataClassification::Internal;
    let confidential = DataClassification::Confidential;
    let personal = DataClassification::Personal;
    let restricted = DataClassification::Restricted;
    
    // Test serialization
    let public_json = serde_json::to_string(&public).unwrap();
    assert!(public_json.contains("Public"));
    
    let personal_json = serde_json::to_string(&personal).unwrap();
    assert!(personal_json.contains("Personal"));
}

#[test]
fn test_deployment_environments() {
    use deployment::DeploymentEnvironment;
    
    let dev = DeploymentEnvironment::Development;
    let staging = DeploymentEnvironment::Staging;
    let prod = DeploymentEnvironment::Production;
    let test = DeploymentEnvironment::Testing;
    
    // Test serialization
    let prod_json = serde_json::to_string(&prod).unwrap();
    assert!(prod_json.contains("Production"));
    
    let staging_json = serde_json::to_string(&staging).unwrap();
    assert!(staging_json.contains("Staging"));
}

#[test]
fn test_compliance_regulations() {
    use compliance::ComplianceRegulation;
    
    let gdpr = ComplianceRegulation::GDPR;
    let fda = ComplianceRegulation::FDA;
    let eu = ComplianceRegulation::EU;
    let iso27001 = ComplianceRegulation::ISO27001;
    let sox = ComplianceRegulation::SOX;
    let hipaa = ComplianceRegulation::HIPAA;
    
    // Test serialization
    let gdpr_json = serde_json::to_string(&gdpr).unwrap();
    assert!(gdpr_json.contains("GDPR"));
    
    let fda_json = serde_json::to_string(&fda).unwrap();
    assert!(fda_json.contains("FDA"));
}
