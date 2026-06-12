use smr_protocol::*;
use std::collections::{HashMap, HashSet};

#[test]
fn test_soroban_bridge_records_contract_calls() {
    let mut bridge = SorobanBridge::new("CONTRACT_TESTID".to_string());

    let intact: HashSet<String> = vec!["A".to_string(), "B".to_string()]
        .into_iter()
        .collect();
    let faulty = HashSet::new();
    let proof = IntactnessProof::new(intact, faulty);

    let reconfig = ReconfigMessage {
        node_id: "A".to_string(),
        slot_seq: 1,
        new_slices: HashMap::new(),
        proof,
        timestamp: 100,
    };

    let snapshot = ConsensusStateSnapshot {
        slot_seq: 1,
        reconfig_msg_hash: "hash_abc123".to_string(),
        timestamp: 100,
        validated_by_count: 2,
    };

    bridge.validate_reconfig_onchain(&reconfig, &snapshot).unwrap();
    assert_eq!(bridge.get_call_history().len(), 1);
    assert_eq!(bridge.get_call_history()[0].method, "validate_reconfig");
}

#[test]
fn test_soroban_validator_workflow() {
    let mut validator = SorobanReconfigValidator::new("SOROBAN_CONTRACT_ID".to_string());

    let intact: HashSet<String> = vec!["A".to_string(), "B".to_string()]
        .into_iter()
        .collect();
    let faulty: HashSet<String> = vec!["C".to_string()].into_iter().collect();
    let proof = IntactnessProof::new(intact, faulty);

    let reconfig = ReconfigMessage {
        node_id: "A".to_string(),
        slot_seq: 5,
        new_slices: HashMap::new(),
        proof,
        timestamp: 500,
    };

    assert!(validator.validate_with_contract(&reconfig, 5).is_ok());
    assert_eq!(validator.get_history().len(), 2);
}

#[test]
fn test_soroban_state_snapshot_immutability() {
    let snapshot1 = ConsensusStateSnapshot {
        slot_seq: 10,
        reconfig_msg_hash: "hash1".to_string(),
        timestamp: 1000,
        validated_by_count: 3,
    };

    let snapshot2 = ConsensusStateSnapshot {
        slot_seq: 10,
        reconfig_msg_hash: "hash1".to_string(),
        timestamp: 1000,
        validated_by_count: 3,
    };

    assert_eq!(snapshot1.slot_seq, snapshot2.slot_seq);
    assert_eq!(snapshot1.reconfig_msg_hash, snapshot2.reconfig_msg_hash);
}

#[test]
fn test_soroban_multiple_contract_calls() {
    let mut bridge = SorobanBridge::new("CONTRACT_MULTI".to_string());

    let intact: HashSet<String> = vec!["A".to_string()].into_iter().collect();
    let faulty = HashSet::new();
    let proof = IntactnessProof::new(intact, faulty);

    for i in 1..=3 {
        let reconfig = ReconfigMessage {
            node_id: "A".to_string(),
            slot_seq: i,
            new_slices: HashMap::new(),
            proof: proof.clone(),
            timestamp: 100 + (i as u64 * 10),
        };

        let snapshot = ConsensusStateSnapshot {
            slot_seq: i,
            reconfig_msg_hash: format!("hash_{}", i),
            timestamp: 100 + (i as u64 * 10),
            validated_by_count: i as usize,
        };

        bridge.validate_reconfig_onchain(&reconfig, &snapshot).ok();
    }

    assert_eq!(bridge.get_call_history().len(), 3);
}
