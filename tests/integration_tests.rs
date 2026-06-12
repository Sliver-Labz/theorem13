use smr_protocol::*;
use std::collections::{HashMap, HashSet};

fn create_test_quorum_slices(node_ids: Vec<&str>) -> HashMap<String, QuorumSlice> {
    let mut slices = HashMap::new();
    let validators: HashSet<String> = node_ids.iter().map(|id| id.to_string()).collect();
    let threshold = (node_ids.len() / 2) + 1;
    for node_id in node_ids {
        slices.insert(
            node_id.to_string(),
            QuorumSlice::new(threshold, validators.clone()),
        );
    }
    slices
}

fn create_test_config() -> SMRConfig {
    SMRConfig {
        enabled: true,
        max_reconfigs_per_slot: 3,
        failure_detection_threshold_ms: 5000,
        intactness_proof_timeout_ms: 2000,
        checkpoint_ballot_height: 0,
    }
}

#[test]
fn test_e2e_normal_consensus() {
    let slices = create_test_quorum_slices(vec!["A", "B", "C"]);
    let config = create_test_config();

    assert!(config.is_valid());
    assert_eq!(slices.len(), 3);
}

#[test]
fn test_e2e_config_loading() {
    let toml_str = r#"
enabled = true
max_reconfigs_per_slot = 5
failure_detection_threshold_ms = 6000
intactness_proof_timeout_ms = 3000
checkpoint_ballot_height = 100
"#;
    let config = ConfigLoader::from_str(toml_str).unwrap();
    assert_eq!(config.max_reconfigs_per_slot, 5);
    assert!(config.enabled);
}

#[test]
fn test_e2e_message_creation_and_serialization() {
    let msg = NetworkMessage::new(
        MessageType::Heartbeat,
        "sender".to_string(),
        1,
        vec![1, 2, 3],
        0,
        100,
    );

    let serialized = MessageSerializer::serialize(&msg).unwrap();
    let deserialized = MessageSerializer::deserialize(&serialized).unwrap();

    assert_eq!(deserialized.sender_id, "sender");
    assert_eq!(deserialized.slot_seq, 1);
}

#[test]
fn test_e2e_p2p_network_formation() {
    let mut node_a = P2PNode::new("A".to_string());
    let mut node_b = P2PNode::new("B".to_string());

    node_a.add_peer("B".to_string());
    node_b.add_peer("A".to_string());

    assert_eq!(node_a.peer_count(), 1);
    assert_eq!(node_b.peer_count(), 1);
}

#[test]
fn test_e2e_byzantine_filter() {
    let mut filter = ByzantineMessageFilter::new(
        vec!["A".to_string(), "B".to_string()],
        10,
    );

    let msg = NetworkMessage::new(
        MessageType::ReconfigProposal,
        "A".to_string(),
        1,
        vec![1, 2, 3],
        0,
        100,
    );

    assert!(filter.validate_message(&msg, 100, 5000));
}

#[test]
fn test_e2e_metrics_collection() {
    let mut tracker = StatisticsTracker::new(100);
    
    tracker.record_slot_start(100);
    tracker.record_reconfig_proposed(105, 50.0);
    tracker.record_reconfig_committed(155, 75.0);

    let stats = tracker.get_statistics();
    assert_eq!(stats.total_slots, 1);
    assert_eq!(stats.successful_reconfigs, 1);
}
