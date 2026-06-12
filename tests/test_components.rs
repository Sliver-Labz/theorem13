use smr_protocol::*;

#[test]
fn test_failure_detection_timeout() {
    let mut detector = FailureDetector::new(5000);

    detector.heartbeat("A".to_string());
    detector.heartbeat("B".to_string());

    let failed = detector.check_timeouts(15000);
    assert_eq!(failed.len(), 2);
}

#[test]
fn test_failure_detection_liveness() {
    let mut detector = FailureDetector::new(5000);

    detector.heartbeat("A".to_string());
    detector.heartbeat("B".to_string());

    let failed = detector.check_timeouts(6000);
    assert_eq!(failed.len(), 0);
}

#[test]
fn test_message_failure_tracking() {
    let mut detector = FailureDetector::new(5000);
    
    detector.record_message_failure("C".to_string());
    let failed = detector.check_timeouts(0);
    assert!(failed.contains(&"C".to_string()));
}

#[test]
fn test_config_builder_validation() {
    let config = ConfigBuilder::default()
        .enabled(true)
        .max_reconfigs_per_slot(5)
        .failure_detection_threshold_ms(7000)
        .intactness_proof_timeout_ms(2500)
        .build()
        .unwrap();

    assert!(config.enabled);
    assert_eq!(config.max_reconfigs_per_slot, 5);
    assert_eq!(config.failure_detection_threshold_ms, 7000);
}

#[test]
fn test_network_broadcast() {
    let mut node = P2PNode::new("A".to_string());
    node.add_peer("B".to_string());
    node.add_peer("C".to_string());
    node.add_peer("D".to_string());

    let msg = NetworkMessage::new(
        MessageType::ReconfigProposal,
        "A".to_string(),
        1,
        vec![1, 2, 3],
        0,
        100,
    );

    let broadcasts = node.broadcast(msg);
    assert_eq!(broadcasts.len(), 3);
}

#[test]
fn test_intactness_proof_validation() {
    use std::collections::HashSet;

    let intact: HashSet<String> = vec!["A".to_string(), "B".to_string()]
        .into_iter()
        .collect();
    let faulty: HashSet<String> = vec!["C".to_string()].into_iter().collect();

    let proof = IntactnessProof::new(intact, faulty);
    assert!(proof.is_disjoint());
}
