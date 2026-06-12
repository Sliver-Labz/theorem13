use smr_protocol::*;

#[test]
fn scenario_5_node_network_nominal() {
    let mut node_a = P2PNode::new("A".to_string());
    for id in &["B", "C", "D", "E"] {
        node_a.add_peer(id.to_string());
    }
    assert_eq!(node_a.connected_peers(), 4);
}

#[test]
fn scenario_detect_and_filter_byzantine() {
    let mut filter = ByzantineMessageFilter::new(
        vec!["A".to_string(), "B".to_string(), "C".to_string()],
        50,
    );

    let msg = NetworkMessage::new(
        MessageType::Heartbeat,
        "A".to_string(),
        1,
        vec![],
        0,
        1000,
    );
    
    assert!(filter.validate_message(&msg, 1000, 100));
    
    let untrusted_msg = NetworkMessage::new(
        MessageType::Heartbeat,
        "Attacker".to_string(),
        1,
        vec![],
        0,
        1000,
    );
    assert!(!filter.validate_message(&untrusted_msg, 1000, 100));
}

#[test]
fn scenario_message_queue_overflow() {
    let mut queue = MessageQueue::new(2);
    
    let msg1 = NetworkMessage::new(
        MessageType::Heartbeat,
        "A".to_string(),
        1,
        vec![],
        0,
        0,
    );
    
    assert!(queue.enqueue(msg1.clone()));
    assert!(queue.enqueue(msg1.clone()));
    assert!(!queue.enqueue(msg1.clone()));
    assert_eq!(queue.size(), 2);
}

#[test]
fn scenario_quorum_formation_3_of_5() {
    use std::collections::HashSet;
    
    let validators: HashSet<String> = 
        vec!["A", "B", "C", "D", "E"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    
    let slice = QuorumSlice::new(3, validators);
    
    let quorum: HashSet<String> = 
        vec!["A", "B", "C"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    
    assert!(slice.is_quorum(&quorum));
}
