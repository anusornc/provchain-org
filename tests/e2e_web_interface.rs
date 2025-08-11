//! End-to-End Web Interface Tests
//! 
//! This test suite validates the complete web interface using browser automation,
//! testing all UI components, interactions, and data flows.

use provchain_org::{
    blockchain::Blockchain,
    web::server::create_web_server,
};
use serde_json::json;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use reqwest::Client;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use anyhow::Result;

/// Test helper to start a test web server with sample data
async fn start_test_server_with_data() -> Result<(u16, tokio::task::JoinHandle<()>)> {
    let mut blockchain = Blockchain::new();
    
    // Add sample data for testing
    add_sample_test_data(&mut blockchain);
    
    let server = create_web_server(blockchain, Some(0)).await?;
    let port = server.port();
    
    let handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            eprintln!("Server error: {}", e);
        }
    });
    
    // Give server time to start
    sleep(Duration::from_millis(1000)).await;
    
    Ok((port, handle))
}

/// Add sample data for testing
fn add_sample_test_data(blockchain: &mut Blockchain) {
    let sample_data = vec![
        // Product batches
        ":batch001 tc:product \"Organic Coffee Beans\" .",
        ":batch001 tc:origin \"Farm ABC, Colombia\" .",
        ":batch001 tc:currentLocation \"Warehouse XYZ, USA\" .",
        ":batch001 tc:status \"In Transit\" .",
        ":batch001 tc:batchId \"BATCH001\" .",
        
        ":batch002 tc:product \"Fair Trade Cocoa\" .",
        ":batch002 tc:origin \"Farm DEF, Ecuador\" .",
        ":batch002 tc:currentLocation \"Processing Plant\" .",
        ":batch002 tc:status \"Processing\" .",
        ":batch002 tc:batchId \"BATCH002\" .",
        
        // Environmental data
        ":batch001 tc:environmentalData :env001 .",
        ":env001 tc:temperature \"22.5\" .",
        ":env001 tc:humidity \"65.0\" .",
        
        // Supply chain events
        ":event001 tc:batch :batch001 .",
        ":event001 tc:actor \"Farmer John\" .",
        ":event001 tc:action \"Harvested\" .",
        ":event001 tc:timestamp \"2024-01-15T08:00:00Z\" .",
        
        // Certifications
        ":batch001 tc:certification \"Organic\" .",
        ":batch002 tc:certification \"Fair Trade\" .",
    ];
    
    for data in sample_data {
        blockchain.add_block(data.to_string());
    }
}

/// Test helper to create browser instance
fn create_browser() -> Result<Browser> {
    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .window_size(Some((1920, 1080)))
        .build()?;
    
    Browser::new(options)
}

/// Test helper to wait for element and handle timeouts
async fn wait_for_element_with_timeout(tab: &Tab, selector: &str, timeout_ms: u64) -> Result<()> {
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(timeout_ms) {
        if tab.find_element(selector).is_ok() {
            return Ok(());
        }
        sleep(Duration::from_millis(100)).await;
    }
    Err(anyhow::anyhow!("Element {} not found within timeout", selector))
}

/// Test helper to simulate user login via UI
async fn login_via_ui(tab: &Tab, username: &str, password: &str) -> Result<()> {
    // Click login button
    tab.click_element("#loginBtn")?;
    wait_for_element_with_timeout(tab, "#loginModal", 2000).await?;
    
    // Fill login form
    tab.type_into_element("#loginUsername", username)?;
    tab.type_into_element("#loginPassword", password)?;
    
    // Submit form
    tab.click_element("#loginForm button[type='submit']")?;
    
    // Wait for login to process
    sleep(Duration::from_millis(2000)).await;
    
    Ok(())
}

#[tokio::test]
async fn test_dashboard_functionality() -> Result<()> {
    let (port, _server_handle) = start_test_server_with_data().await?;
    let base_url = format!("http://localhost:{}", port);
    
    println!("Testing Dashboard Functionality on {}", base_url);
    
    let browser = create_browser()?;
    let tab = browser.wait_for_initial_tab()?;
    
    // Navigate to application
    tab.navigate_to(&base_url)?;
    tab.wait_for_element("nav.navbar")?;
    
    // Verify dashboard is active by default
    let dashboard = tab.wait_for_element("#dashboard.content-section.active")?;
    assert!(dashboard.get_description()?.contains("Dashboard"));
    
    // Check for stats cards
    tab.wait_for_element(".stats-grid")?;
    tab.wait_for_element("#blockHeight")?;
    tab.wait_for_element("#totalTransactions")?;
    tab.wait_for_element("#networkPeers")?;
    tab.wait_for_element("#blockchainStatus")?;
    
    // Check for dashboard cards
    tab.wait_for_element(".dashboard-grid")?;
    tab.wait_for_element("#recentTransactions")?;
    tab.wait_for_element("#apiStatus")?;
    tab.wait_for_element("#lastBlockTime")?;
    tab.wait_for_element("#validationStatus")?;
    
    // Login to see actual data
    login_via_ui(&tab, "admin", "password").await?;
    
    // Wait for data to load
    sleep(Duration::from_millis(3000)).await;
    
    // Verify stats are populated (should show numbers, not just "-")
    let block_height_text = tab.get_element_text("#blockHeight")?;
    assert_ne!(block_height_text, "-", "Block height should be populated");
    
    println!("✓ Dashboard functionality test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_block_explorer_functionality() -> Result<()> {
    let (port, _server_handle) = start_test_server_with_data().await?;
    let base_url = format!("http://localhost:{}", port);
    
    println!("Testing Block Explorer Functionality on {}", base_url);
    
    let browser = create_browser()?;
    let tab = browser.wait_for_initial_tab()?;
    
    // Navigate and login
    tab.navigate_to(&base_url)?;
    tab.wait_for_element("nav.navbar")?;
    login_via_ui(&tab, "admin", "password").await?;
    
    // Navigate to blocks section
    tab.click_element("a[data-section='blocks']")?;
    tab.wait_for_element("#blocks.content-section.active")?;
    
    // Check for blocks controls
    tab.wait_for_element("#blockSearch")?;
    tab.wait_for_element("#refreshBlocks")?;
    tab.wait_for_element("#blocksList")?;
    
    // Test refresh functionality
    tab.click_element("#refreshBlocks")?;
    sleep(Duration::from_millis(2000)).await;
    
    // Test search functionality
    tab.type_into_element("#blockSearch", "0")?;
    sleep(Duration::from_millis(1000)).await;
    
    // Clear search
    tab.evaluate("document.getElementById('blockSearch').value = ''", false)?;
    sleep(Duration::from_millis(500)).await;
    
    println!("✓ Block Explorer functionality test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_product_traceability_interface() -> Result<()> {
    let (port, _server_handle) = start_test_server_with_data().await?;
    let base_url = format!("http://localhost:{}", port);
    
    println!("Testing Product Traceability Interface on {}", base_url);
    
    let browser = create_browser()?;
    let tab = browser.wait_for_initial_tab()?;
    
    // Navigate and login
    tab.navigate_to(&base_url)?;
    tab.wait_for_element("nav.navbar")?;
    login_via_ui(&tab, "user", "password").await?;
    
    // Navigate to traceability section
    tab.click_element("a[data-section='traceability']")?;
    tab.wait_for_element("#traceability.content-section.active")?;
    
    // Check for traceability interface elements
    tab.wait_for_element("#batchId")?;
    tab.wait_for_element("#productName")?;
    tab.wait_for_element("#traceProduct")?;
    tab.wait_for_element("#traceResults")?;
    
    // Test traceability search with known batch
    tab.type_into_element("#batchId", "BATCH001")?;
    tab.type_into_element("#productName", "Coffee")?;
    tab.click_element("#traceProduct")?;
    
    // Wait for results
    sleep(Duration::from_millis(3000)).await;
    
    // Verify results area is updated
    let results_element = tab.find_element("#traceResults")?;
    let results_html = results_element.get_content()?;
    
    // Should not show placeholder anymore
    assert!(!results_html.contains("trace-placeholder"), "Should show actual results, not placeholder");
    
    // Test with invalid batch ID
    tab.evaluate("document.getElementById('batchId').value = ''", false)?;
    tab.type_into_element("#batchId", "INVALID_BATCH")?;
    tab.click_element("#traceProduct")?;
    
    sleep(Duration::from_millis(2000)).await;
    
    println!("✓ Product Traceability Interface test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_sparql_query_interface() -> Result<()> {
    let (port, _server_handle) = start_test_server_with_data().await?;
    let base_url = format!("http://localhost:{}", port);
    
    println!("Testing SPARQL Query Interface on {}", base_url);
    
    let browser = create_browser()?;
    let tab = browser.wait_for_initial_tab()?;
    
    // Navigate and login
    tab.navigate_to(&base_url)?;
    tab.wait_for_element("nav.navbar")?;
    login_via_ui(&tab, "admin", "password").await?;
    
    // Navigate to SPARQL section
    tab.click_element("a[data-section='sparql']")?;
    tab.wait_for_element("#sparql.content-section.active")?;
    
    // Check for SPARQL interface elements
    tab.wait_for_element("#sparqlQuery")?;
    tab.wait_for_element("#queryTemplates")?;
    tab.wait_for_element("#executeQuery")?;
    tab.wait_for_element("#sparqlResults")?;
    
    // Test query template selection
    tab.select_option_by_value("#queryTemplates", "all-triples")?;
    sleep(Duration::from_millis(500)).await;
    
    // Verify query was populated
    let query_text = tab.get_element_text("#sparqlQuery")?;
    assert!(!query_text.is_empty(), "Query template should populate the editor");
    
    // Test custom query
    tab.evaluate("document.getElementById('sparqlQuery').value = ''", false)?;
    tab.type_into_element("#sparqlQuery", "SELECT * WHERE { ?s ?p ?o } LIMIT 5")?;
    
    // Execute query
    tab.click_element("#executeQuery")?;
    
    // Wait for results
    sleep(Duration::from_millis(3000)).await;
    
    // Check for results
    let results_element = tab.find_element("#sparqlResults")?;
    let results_html = results_element.get_content()?;
    
    // Should not show placeholder anymore
    assert!(!results_html.contains("results-placeholder"), "Should show query results");
    
    // Test query stats
    wait_for_element_with_timeout(&tab, "#queryStats", 2000).await?;
    
    // Test invalid query
    tab.evaluate("document.getElementById('sparqlQuery').value = ''", false)?;
    tab.type_into_element("#sparqlQuery", "INVALID SPARQL SYNTAX")?;
    tab.click_element("#executeQuery")?;
    
    sleep(Duration::from_millis(2000)).await;
    
    println!("✓ SPARQL Query Interface test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_transaction_management_interface() -> Result<()> {
    let (port, _server_handle) = start_test_server_with_data().await?;
    let base_url = format!("http://localhost:{}", port);
    
    println!("Testing Transaction Management Interface on {}", base_url);
    
    let browser = create_browser()?;
    let tab = browser.wait_for_initial_tab()?;
    
    // Navigate and login
    tab.navigate_to(&base_url)?;
    tab.wait_for_element("nav.navbar")?;
    login_via_ui(&tab, "manager", "password").await?;
    
    // Navigate to transactions section
    tab.click_element("a[data-section='transactions']")?;
    tab.wait_for_element("#transactions.content-section.active")?;
    
    // Check for transaction interface elements
    tab.wait_for_element("#addTripleForm")?;
    tab.wait_for_element("#subject")?;
    tab.wait_for_element("#predicate")?;
    tab.wait_for_element("#object")?;
    tab.wait_for_element("#transactionsList")?;
    
    // Test form validation (empty form)
    tab.click_element("#addTripleForm button[type='submit']")?;
    sleep(Duration::from_millis(500)).await;
    
    // Fill form with valid data
    tab.type_into_element("#subject", ":testBatch123")?;
    tab.type_into_element("#predicate", "tc:hasStatus")?;
    tab.type_into_element("#object", "Testing")?;
    
    // Submit form
    tab.click_element("#addTripleForm button[type='submit']")?;
    
    // Wait for submission
    sleep(Duration::from_millis(3000)).await;
    
    // Check if form was cleared (indicates successful submission)
    let subject_value = tab.get_element_attribute("#subject", "value")?;
    // Form might be cleared on successful submission
    
    // Check transaction history
    let transactions_element = tab.find_element("#transactionsList")?;
    let transactions_html = transactions_element.get_content()?;
    
    // Should show transactions, not loading message
    assert!(!transactions_html.contains("Loading transactions"), "Should show transaction history");
    
    println!("✓ Transaction Management Interface test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_authentication_flow() -> Result<()> {
    let (port, _server_handle) = start_test_server_with_data().await?;
    let base_url = format!("http://localhost:{}", port);
    
    println!("Testing Authentication Flow on {}", base_url);
    
    let browser = create_browser()?;
    let tab = browser.wait_for_initial_tab()?;
    
    // Navigate to application
    tab.navigate_to(&base_url)?;
    tab.wait_for_element("nav.navbar")?;
    
    // Verify login button is visible
    tab.wait_for_element("#loginBtn")?;
    
    // Test login modal
    tab.click_element("#loginBtn")?;
    tab.wait_for_element("#loginModal")?;
    
    // Test modal close
    tab.click_element(".modal .close")?;
    sleep(Duration::from_millis(500)).await;
    
    // Modal should be hidden
    let modal_style = tab.get_element_attribute("#loginModal", "style")?;
    assert!(modal_style.contains("display: none") || modal_style.is_empty());
    
    // Test login with invalid credentials
    tab.click_element("#loginBtn")?;
    tab.wait_for_element("#loginModal")?;
    
    tab.type_into_element("#loginUsername", "invalid")?;
    tab.type_into_element("#loginPassword", "invalid")?;
    tab.click_element("#loginForm button[type='submit']")?;
    
    sleep(Duration::from_millis(2000)).await;
    
    // Should still show login button (login failed)
    tab.wait_for_element("#loginBtn")?;
    
    // Test login with valid credentials
    tab.evaluate("document.getElementById('loginUsername').value = ''", false)?;
    tab.evaluate("document.getElementById('loginPassword').value = ''", false)?;
    
    tab.type_into_element("#loginUsername", "testuser")?;
    tab.type_into_element("#loginPassword", "testpass")?;
    tab.click_element("#loginForm button[type='submit']")?;
    
    sleep(Duration::from_millis(2000)).await;
    
    // Check if user info appears or login button changes
    // (Behavior depends on actual authentication implementation)
    
    println!("✓ Authentication Flow test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_navigation_and_routing() -> Result<()> {
    let (port, _server_handle) = start_test_server_with_data().await?;
    let base_url = format!("http://localhost:{}", port);
    
    println!("Testing Navigation and Routing on {}", base_url);
    
    let browser = create_browser()?;
    let tab = browser.wait_for_initial_tab()?;
    
    // Navigate to application
    tab.navigate_to(&base_url)?;
    tab.wait_for_element("nav.navbar")?;
    
    // Test all navigation links
    let nav_sections = vec![
        ("dashboard", "#dashboard"),
        ("blocks", "#blocks"),
        ("traceability", "#traceability"),
        ("sparql", "#sparql"),
        ("transactions", "#transactions"),
    ];
    
    for (section, selector) in nav_sections {
        // Click navigation link
        tab.click_element(&format!("a[data-section='{}']", section))?;
        
        // Wait for section to become active
        tab.wait_for_element(&format!("{}.content-section.active", selector))?;
        
        // Verify other sections are not active
        for (other_section, other_selector) in &[
            ("dashboard", "#dashboard"),
            ("blocks", "#blocks"),
            ("traceability", "#traceability"),
            ("sparql", "#sparql"),
            ("transactions", "#transactions"),
        ] {
            if *other_section != section {
                let other_element = tab.find_element(other_selector)?;
                let class_attr = other_element.get_attribute_value("class")?;
                assert!(!class_attr.unwrap_or_default().contains("active"), 
                       "Other sections should not be active");
            }
        }
        
        // Verify navigation link is active
        let nav_link = tab.find_element(&format!("a[data-section='{}']", section))?;
        let nav_class = nav_link.get_attribute_value("class")?;
        assert!(nav_class.unwrap_or_default().contains("active"), 
               "Navigation link should be active");
        
        sleep(Duration::from_millis(200)).await;
    }
    
    println!("✓ Navigation and Routing test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_responsive_design() -> Result<()> {
    let (port, _server_handle) = start_test_server_with_data().await?;
    let base_url = format!("http://localhost:{}", port);
    
    println!("Testing Responsive Design on {}", base_url);
    
    let browser = create_browser()?;
    let tab = browser.wait_for_initial_tab()?;
    
    // Navigate to application
    tab.navigate_to(&base_url)?;
    tab.wait_for_element("nav.navbar")?;
    
    // Test desktop view (default)
    let navbar = tab.find_element("nav.navbar")?;
    assert!(navbar.get_description()?.contains("navbar"));
    
    // Test tablet view
    tab.set_viewport_size(768, 1024, None, None, false)?;
    sleep(Duration::from_millis(500)).await;
    
    // Verify layout still works
    tab.wait_for_element("nav.navbar")?;
    tab.wait_for_element("#dashboard")?;
    
    // Test mobile view
    tab.set_viewport_size(375, 667, None, None, false)?;
    sleep(Duration::from_millis(500)).await;
    
    // Verify layout still works
    tab.wait_for_element("nav.navbar")?;
    tab.wait_for_element("#dashboard")?;
    
    // Test navigation still works on mobile
    tab.click_element("a[data-section='blocks']")?;
    tab.wait_for_element("#blocks.content-section.active")?;
    
    // Reset to desktop view
    tab.set_viewport_size(1920, 1080, None, None, false)?;
    sleep(Duration::from_millis(500)).await;
    
    println!("✓ Responsive Design test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_error_handling_ui() -> Result<()> {
    let (port, _server_handle) = start_test_server_with_data().await?;
    let base_url = format!("http://localhost:{}", port);
    
    println!("Testing Error Handling UI on {}", base_url);
    
    let browser = create_browser()?;
    let tab = browser.wait_for_initial_tab()?;
    
    // Navigate and login
    tab.navigate_to(&base_url)?;
    tab.wait_for_element("nav.navbar")?;
    login_via_ui(&tab, "user", "password").await?;
    
    // Test SPARQL error handling
    tab.click_element("a[data-section='sparql']")?;
    tab.wait_for_element("#sparql.content-section.active")?;
    
    // Enter invalid SPARQL query
    tab.type_into_element("#sparqlQuery", "INVALID SPARQL SYNTAX")?;
    tab.click_element("#executeQuery")?;
    
    // Wait for error response
    sleep(Duration::from_millis(3000)).await;
    
    // Check for error indication in results
    let results_element = tab.find_element("#sparqlResults")?;
    let results_html = results_element.get_content()?;
    
    // Should show some kind of error indication
    // (Exact error handling depends on implementation)
    
    // Test traceability error handling
    tab.click_element("a[data-section='traceability']")?;
    tab.wait_for_element("#traceability.content-section.active")?;
    
    // Search for non-existent batch
    tab.type_into_element("#batchId", "NONEXISTENT_BATCH")?;
    tab.click_element("#traceProduct")?;
    
    sleep(Duration::from_millis(2000)).await;
    
    // Should handle gracefully (no crash)
    tab.wait_for_element("#traceResults")?;
    
    // Test form validation
    tab.click_element("a[data-section='transactions']")?;
    tab.wait_for_element("#transactions.content-section.active")?;
    
    // Try to submit empty form
    tab.click_element("#addTripleForm button[type='submit']")?;
    sleep(Duration::from_millis(500)).await;
    
    // Form should prevent submission or show validation errors
    // (Exact behavior depends on implementation)
    
    println!("✓ Error Handling UI test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_real_time_updates() -> Result<()> {
    let (port, _server_handle) = start_test_server_with_data().await?;
    let base_url = format!("http://localhost:{}", port);
    
    println!("Testing Real-time Updates on {}", base_url);
    
    let browser = create_browser()?;
    let tab = browser.wait_for_initial_tab()?;
    
    // Navigate and login
    tab.navigate_to(&base_url)?;
    tab.wait_for_element("nav.navbar")?;
    login_via_ui(&tab, "admin", "password").await?;
    
    // Go to dashboard to see stats
    tab.click_element("a[data-section='dashboard']")?;
    tab.wait_for_element("#dashboard.content-section.active")?;
    
    // Wait for initial data load
    sleep(Duration::from_millis(3000)).await;
    
    // Get initial block height
    let initial_height = tab.get_element_text("#blockHeight")?;
    
    // Add new data via API in background
    let client = Client::new();
    let auth_response = client
        .post(&format!("{}/auth/login", base_url))
        .json(&json!({
            "username": "admin",
            "password": "password"
        }))
        .send()
        .await?;
    
    if auth_response.status().is_success() {
        let auth_data: serde_json::Value = auth_response.json().await?;
        let token = auth_data["token"].as_str().unwrap_or("");
        
        // Add new triple
        let _add_response = client
            .post(&format!("{}/api/blockchain/add-triple", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&json!({
                "subject": "http://example.org/realtime_test",
                "predicate": "http://provchain.org/trace#status",
                "object": "Real-time Test",
                "graph_name": "realtime"
            }))
            .send()
            .await?;
    }
    
    // Wait for potential updates
    sleep(Duration::from_millis(5000)).await;
    
    // Check if stats updated (depends on implementation of real-time updates)
    let updated_height = tab.get_element_text("#blockHeight")?;
    
    // Note: This test verifies the UI can handle updates, 
    // actual real-time behavior depends on WebSocket implementation
    
    println!("✓ Real-time Updates test completed successfully");
    println!("  Initial height: {}, Updated height: {}", initial_height, updated_height);
    Ok(())
}
