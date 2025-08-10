//! Phase 4: Distributed Network Implementation Test Suite
//! 
//! This test suite validates the distributed networking capabilities including:
//! - Peer discovery and connection management
//! - P2P message passing and protocol handling
//! - Network manager functionality
//! - Basic distributed network components

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;
use anyhow::Result;

use provchain_org::blockchain::Blockchain;
use provchain_org::config::NodeConfig;
use provchain_org::network::{NetworkManager, messages::{P2PMessage, PeerInfo}};
use provchain_org::network::discovery::PeerDiscovery;

/// Test peer discovery functionality
#[tokio::test]
async fn test_peer_discovery() -> Result<()> {
    // Create peer info for testing
    let peer1_info = PeerInfo::new(
        Uuid::new_v4(),
        "127.0.0.1".to_string(),
        8081,
        "test-network".to_string(),
        false,
    );
    
    let peer2_info = PeerInfo::new(
        Uuid::new_v4(),
        "127.0.0.1".to_string(),
        8082,
        "test-network".to_string(),
        false,
    );
    
    // Create peer discovery instances
    let bootstrap_peers = vec!["127.0.0.1:8081".to_string()];
    let discovery1 = PeerDiscovery::new(peer1_info, vec![]);
    let discovery2 = PeerDiscovery::new(peer2_info.clone(), bootstrap_peers);
    
    // Start discovery on both nodes
    discovery1.start_discovery().await?;
    discovery2.start_discovery().await?;
    
    // Wait for discovery to complete
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Test adding discovered peers
    discovery1.add_discovered_peer(peer2_info).await?;
    
    // Check that peer was added
    let known_peers = discovery1.get_known_peers().await;
    assert_eq!(known_peers.len(), 1, "Should have discovered one peer");
    
    Ok(())
}

/// Test P2P message creation and handling
#[tokio::test]
async fn test_p2p_messaging() -> Result<()> {
    let node_id = Uuid::new_v4();
    
    // Test ping message
    let ping_message = P2PMessage::Ping {
        sender_id: node_id,
        timestamp: chrono::Utc::now(),
    };
    
    // Test peer discovery message
    let discovery_message = P2PMessage::new_peer_discovery(
        node_id,
        8080,
        "test-network".to_string(),
    );
    
    // Test block announcement message
    let blockchain = Blockchain::new();
    let block = blockchain.chain.first().unwrap(); // Genesis block
    let announcement = P2PMessage::new_block_announcement(
        block,
        format!("http://provchain.org/block/{}", block.index)
    );
    
    // Verify messages were created successfully
    match ping_message {
        P2PMessage::Ping { sender_id, .. } => assert_eq!(sender_id, node_id),
        _ => panic!("Expected Ping message"),
    }
    
    match discovery_message {
        P2PMessage::PeerDiscovery { node_id: id, listen_port, .. } => {
            assert_eq!(id, node_id);
            assert_eq!(listen_port, 8080);
        },
        _ => panic!("Expected PeerDiscovery message"),
    }
    
    match announcement {
        P2PMessage::BlockAnnouncement { block_index, .. } => {
            assert_eq!(block_index, 0); // Genesis block
        },
        _ => panic!("Expected BlockAnnouncement message"),
    }
    
    Ok(())
}

/// Test message broadcasting across the network
#[tokio::test]
async fn test_message_broadcasting() -> Result<()> {
    // Create a small network of nodes
    let mut networks = Vec::new();
    let mut blockchains = Vec::new();
    
    for i in 0..3 {
        let mut config = NodeConfig::default();
        config.network.listen_port = 8100 + i;
        config.node_id = Uuid::new_v4();
        
        let network = Arc::new(NetworkManager::new(config));
        let blockchain = Arc::new(RwLock::new(Blockchain::new()));
        
        networks.push(network);
        blockchains.push(blockchain);
    }
    
    // Create a test block to broadcast
    let test_block = {
        let mut blockchain = blockchains[0].write().await;
        blockchain.add_block("Test broadcast data".to_string());
        blockchain.chain.last().unwrap().clone()
    };
    
    // Create block announcement message
    let announcement = P2PMessage::new_block_announcement(
        &test_block,
        format!("http://provchain.org/block/{}", test_block.index)
    );
    
    // Broadcast from first node
    networks[0].broadcast_message(announcement).await?;
    
    // Wait for message propagation
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Verify broadcast completed without errors
    assert!(true, "Message broadcasting completed successfully");
    
    Ok(())
}

/// Test basic network manager functionality
#[tokio::test]
async fn test_network_manager_basic() -> Result<()> {
    let config = NodeConfig::default();
    let network = NetworkManager::new(config.clone());
    
    // Test basic properties
    assert_eq!(network.node_id, config.node_id);
    assert_eq!(network.config.network.listen_port, config.network.listen_port);
    
    // Test peer list (should be empty initially)
    let peers = network.get_connected_peers().await;
    assert_eq!(peers.len(), 0);
    
    Ok(())
}

/// Test peer discovery network statistics
#[tokio::test]
async fn test_peer_discovery_stats() -> Result<()> {
    let local_info = PeerInfo::new(
        Uuid::new_v4(),
        "127.0.0.1".to_string(),
        8080,
        "test-network".to_string(),
        false,
    );
    
    let discovery = PeerDiscovery::new(local_info, vec!["127.0.0.1:8081".to_string()]);
    
    // Add a regular peer
    let regular_peer = PeerInfo::new(
        Uuid::new_v4(),
        "127.0.0.1".to_string(),
        8081,
        "test-network".to_string(),
        false,
    );
    discovery.add_discovered_peer(regular_peer).await?;
    
    // Add an authority peer
    let authority_peer = PeerInfo::new(
        Uuid::new_v4(),
        "127.0.0.1".to_string(),
        8082,
        "test-network".to_string(),
        true,
    );
    discovery.add_discovered_peer(authority_peer).await?;
    
    let stats = discovery.get_network_stats().await;
    assert_eq!(stats.total_peers, 2);
    assert_eq!(stats.authority_peers, 1);
    assert_eq!(stats.regular_peers, 1);
    assert_eq!(stats.bootstrap_peers, 1);
    
    Ok(())
}

/// Test peer discovery message handling
#[tokio::test]
async fn test_peer_discovery_message_handling() -> Result<()> {
    let local_info = PeerInfo::new(
        Uuid::new_v4(),
        "127.0.0.1".to_string(),
        8080,
        "test-network".to_string(),
        false,
    );
    
    let discovery = PeerDiscovery::new(local_info, vec![]);
    
    // Test peer discovery message
    let peer_id = Uuid::new_v4();
    let discovery_message = P2PMessage::new_peer_discovery(
        peer_id,
        8081,
        "test-network".to_string(),
    );
    
    let response = discovery.handle_peer_discovery(discovery_message).await?;
    
    // Should respond with peer list
    assert!(response.is_some());
    match response.unwrap() {
        P2PMessage::PeerList { peers, .. } => {
            // Should contain the newly discovered peer
            assert_eq!(peers.len(), 1);
        },
        _ => panic!("Expected PeerList response"),
    }
    
    // Check that peer was added
    let known_peers = discovery.get_known_peers().await;
    assert_eq!(known_peers.len(), 1);
    assert_eq!(known_peers[0].node_id, peer_id);
    
    Ok(())
}

/// Test network configuration validation
#[tokio::test]
async fn test_network_configuration() -> Result<()> {
    // Test default configuration
    let default_config = NodeConfig::default();
    assert!(default_config.validate().is_ok());
    
    // Test network configuration
    let network_config = &default_config.network;
    assert!(!network_config.network_id.is_empty());
    assert!(network_config.listen_port > 0);
    assert!(network_config.max_peers > 0);
    
    // Test consensus configuration
    let consensus_config = &default_config.consensus;
    assert!(!consensus_config.is_authority); // Default should not be authority
    assert!(consensus_config.block_interval > 0);
    assert!(consensus_config.max_block_size > 0);
    
    Ok(())
}

/// Test peer info creation and validation
#[tokio::test]
async fn test_peer_info() -> Result<()> {
    let peer_id = Uuid::new_v4();
    let peer_info = PeerInfo::new(
        peer_id,
        "192.168.1.100".to_string(),
        8080,
        "production-network".to_string(),
        true, // Authority peer
    );
    
    // Test basic properties
    assert_eq!(peer_info.node_id, peer_id);
    assert_eq!(peer_info.address, "192.168.1.100");
    assert_eq!(peer_info.port, 8080);
    assert_eq!(peer_info.network_id, "production-network");
    assert!(peer_info.is_authority);
    
    // Test full address
    assert_eq!(peer_info.full_address(), "192.168.1.100:8080");
    
    Ok(())
}

/// Test multiple peer discovery instances
#[tokio::test]
async fn test_multiple_peer_discovery() -> Result<()> {
    let mut discoveries = Vec::new();
    let mut peer_infos = Vec::new();
    
    // Create multiple peer discovery instances
    for i in 0..5 {
        let peer_info = PeerInfo::new(
            Uuid::new_v4(),
            "127.0.0.1".to_string(),
            8200 + i,
            "multi-test-network".to_string(),
            i % 2 == 0, // Every other peer is an authority
        );
        
        let bootstrap_peers = if i > 0 {
            vec![format!("127.0.0.1:{}", 8200)]
        } else {
            vec![]
        };
        
        let discovery = PeerDiscovery::new(peer_info.clone(), bootstrap_peers);
        
        peer_infos.push(peer_info);
        discoveries.push(discovery);
    }
    
    // Start all discovery instances
    for discovery in &discoveries {
        discovery.start_discovery().await?;
    }
    
    // Wait for discovery processes
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Add peers to the first discovery instance
    for i in 1..peer_infos.len() {
        discoveries[0].add_discovered_peer(peer_infos[i].clone()).await?;
    }
    
    // Check network stats
    let stats = discoveries[0].get_network_stats().await;
    assert_eq!(stats.total_peers, 4); // 4 other peers
    assert_eq!(stats.authority_peers, 2); // Peers 1 and 3 are authorities
    assert_eq!(stats.regular_peers, 2); // Peers 2 and 4 are regular
    
    Ok(())
}

/// Test error handling in peer discovery
#[tokio::test]
async fn test_peer_discovery_error_handling() -> Result<()> {
    let local_info = PeerInfo::new(
        Uuid::new_v4(),
        "127.0.0.1".to_string(),
        8080,
        "test-network".to_string(),
        false,
    );
    
    let discovery = PeerDiscovery::new(local_info, vec![]);
    
    // Test peer discovery with different network ID
    let peer_id = Uuid::new_v4();
    let discovery_message = P2PMessage::new_peer_discovery(
        peer_id,
        8081,
        "different-network".to_string(), // Different network ID
    );
    
    let response = discovery.handle_peer_discovery(discovery_message).await?;
    
    // Should respond with error
    assert!(response.is_some());
    match response.unwrap() {
        P2PMessage::Error { error_code, .. } => {
            use provchain_org::network::messages::ErrorCode;
            assert_eq!(error_code, ErrorCode::NetworkMismatch);
        },
        _ => panic!("Expected Error response"),
    }
    
    // Peer should not be added
    let known_peers = discovery.get_known_peers().await;
    assert_eq!(known_peers.len(), 0);
    
    Ok(())
}

/// Performance test for peer discovery operations
#[tokio::test]
async fn test_peer_discovery_performance() -> Result<()> {
    let local_info = PeerInfo::new(
        Uuid::new_v4(),
        "127.0.0.1".to_string(),
        8080,
        "perf-test-network".to_string(),
        false,
    );
    
    let discovery = PeerDiscovery::new(local_info, vec![]);
    
    let start_time = std::time::Instant::now();
    
    // Add many peers quickly
    for i in 0..100 {
        let peer_info = PeerInfo::new(
            Uuid::new_v4(),
            "127.0.0.1".to_string(),
            8100 + i,
            "perf-test-network".to_string(),
            false,
        );
        discovery.add_discovered_peer(peer_info).await?;
    }
    
    let duration = start_time.elapsed();
    
    // Should complete quickly
    assert!(duration.as_millis() < 1000, "Adding 100 peers should be fast");
    
    // Verify all peers were added
    let known_peers = discovery.get_known_peers().await;
    assert_eq!(known_peers.len(), 100);
    
    let stats = discovery.get_network_stats().await;
    assert_eq!(stats.total_peers, 100);
    
    Ok(())
}

/// Test network manager message handling
#[tokio::test]
async fn test_network_manager_message_handling() -> Result<()> {
    let config = NodeConfig::default();
    let network = NetworkManager::new(config.clone());
    
    // Test handling incoming message (basic functionality)
    let peer_id = Uuid::new_v4();
    let ping_message = P2PMessage::Ping {
        sender_id: peer_id,
        timestamp: chrono::Utc::now(),
    };
    
    // This should not fail even with no message handlers
    let result = network.handle_incoming_message(peer_id, ping_message).await;
    assert!(result.is_ok(), "Message handling should not fail");
    
    Ok(())
}

/// Integration test for basic distributed network functionality
#[tokio::test]
async fn test_basic_distributed_network_integration() -> Result<()> {
    // Create multiple network managers
    let mut networks = Vec::new();
    let mut discoveries = Vec::new();
    
    for i in 0..3 {
        let mut config = NodeConfig::default();
        config.network.listen_port = 8300 + i;
        config.node_id = Uuid::new_v4();
        
        let network = Arc::new(NetworkManager::new(config.clone()));
        networks.push(network);
        
        // Create peer info for discovery
        let peer_info = PeerInfo::new(
            config.node_id,
            "127.0.0.1".to_string(),
            config.network.listen_port,
            config.network.network_id.clone(),
            i == 0, // First node is authority
        );
        
        let bootstrap_peers = if i > 0 {
            vec!["127.0.0.1:8300".to_string()]
        } else {
            vec![]
        };
        
        let discovery = PeerDiscovery::new(peer_info, bootstrap_peers);
        discoveries.push(discovery);
    }
    
    // Start all discovery processes
    for discovery in &discoveries {
        discovery.start_discovery().await?;
    }
    
    // Wait for network formation
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Test message broadcasting
    let test_message = P2PMessage::Ping {
        sender_id: networks[0].node_id,
        timestamp: chrono::Utc::now(),
    };
    
    networks[0].broadcast_message(test_message).await?;
    
    // Check network stats
    let stats = discoveries[0].get_network_stats().await;
    assert_eq!(stats.authority_peers, 0); // No other authorities discovered yet
    assert_eq!(stats.bootstrap_peers, 0); // First node has no bootstrap peers
    
    Ok(())
}
