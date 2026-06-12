use smr_protocol::*;
use std::collections::HashSet;

#[test]
fn test_network_message_serialization_roundtrip() {
    let msg = NetworkMessage::new(
        MessageType::ReconfigProposal,
        "node-1".to_string(),
        42,
        vec![0xFF, 0xAA, 0x55],
        10,
        12345,
    );

    let serialized = MessageSerializer::serialize(&msg).unwrap();
    let deserialized = MessageSerializer::deserialize(&serialized).unwrap();

    assert_eq!(deserialized.sender_id, msg.sender_id);
    assert_eq!(deserialized.slot_seq, msg.slot_seq);
    assert_eq!(deserialized.sequence_num, msg.sequence_num);
    assert_eq!(deserialized.timestamp, msg.timestamp);
    assert_eq!(deserialized.payload, msg.payload);
}

#[test]
fn test_config_toml_serialization() {
    let config = ConfigBuilder::default()
        .max_reconfigs_per_slot(7)
        .failure_detection_threshold_ms(8000)
        .build()
        .unwrap();

    let toml_str = toml::to_string_pretty(&config).unwrap();
    let loaded = ConfigLoader::from_str(&toml_str).unwrap();

    assert_eq!(loaded.max_reconfigs_per_slot, 7);
    assert_eq!(loaded.failure_detection_threshold_ms, 8000);
}

#[test]
fn test_intactness_proof_serialization() {
    let intact: HashSet<String> = vec!["X".to_string(), "Y".to_string()]
        .into_iter()
        .collect();
    let faulty: HashSet<String> = vec!["Z".to_string()].into_iter().collect();

    let proof = IntactnessProof::new(intact, faulty);
    let json = serde_json::to_string(&proof).unwrap();
    let deserialized: IntactnessProof = serde_json::from_str(&json).unwrap();

    assert!(deserialized.is_disjoint());
}
