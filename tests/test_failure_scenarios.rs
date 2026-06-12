use smr_protocol::*;

#[test]
fn test_failure_detector_single_node_timeout() {
    let mut detector = FailureDetector::new(1000);
    detector.heartbeat("Node-A".to_string());
    
    let failed_at_500ms = detector.check_timeouts(500);
    assert!(failed_at_500ms.is_empty());
    
    let failed_at_2000ms = detector.check_timeouts(2000);
    assert!(failed_at_2000ms.contains(&"Node-A".to_string()));
}

#[test]
fn test_failure_detector_multiple_nodes_different_times() {
    let mut detector = FailureDetector::new(1000);
    
    detector.heartbeat("A".to_string());
    detector.heartbeat("B".to_string());
    
    let failed = detector.check_timeouts(2500);
    assert_eq!(failed.len(), 2);
}

#[test]
fn test_byzantine_filter_timestamp_tolerance() {
    let mut filter = ByzantineMessageFilter::new(
        vec!["A".to_string()],
        10,
    );

    let msg = NetworkMessage::new(
        MessageType::Heartbeat,
        "A".to_string(),
        1,
        vec![],
        0,
        1000,
    );

    assert!(filter.validate_message(&msg, 1050, 100));
    assert!(!filter.validate_message(&msg, 1200, 100));
}

#[test]
fn test_coordinator_state_recovery() {
    use std::collections::HashMap;
    
    let initial_slices = HashMap::new();
    let nodes = vec!["A".to_string(), "B".to_string(), "C".to_string()];

    let coordinator = ReconfigCoordinator::new("A".to_string(), initial_slices, nodes);
    assert_eq!(coordinator.node_id, "A");
}
