use smr_protocol::*;
use std::collections::{HashMap, HashSet};

#[test]
fn test_reconfig_proposal_flow() {
    let mut protocol = {
        let initial_slices = HashMap::new();
        protocol::ProtocolState::new(1, initial_slices)
    };

    let new_slices = HashMap::new();
    let intact: HashSet<String> = vec!["A".to_string(), "B".to_string()]
        .into_iter()
        .collect();
    let faulty = HashSet::new();
    let proof = IntactnessProof::new(intact, faulty);

    let msg = ReconfigMessage {
        node_id: "A".to_string(),
        slot_seq: 1,
        new_slices,
        proof,
        timestamp: 0,
    };

    assert!(protocol.propose_reconfig(msg));
    assert_eq!(protocol.state, SMRState::ReconfigProposed);
}

#[test]
fn test_reconfig_voting_and_commitment() {
    let mut protocol = {
        let initial_slices = HashMap::new();
        protocol::ProtocolState::new(1, initial_slices)
    };

    let new_slices = HashMap::new();
    let intact: HashSet<String> = vec!["A".to_string(), "B".to_string()]
        .into_iter()
        .collect();
    let faulty = HashSet::new();
    let proof = IntactnessProof::new(intact, faulty);

    let msg = ReconfigMessage {
        node_id: "A".to_string(),
        slot_seq: 1,
        new_slices,
        proof,
        timestamp: 0,
    };

    protocol.propose_reconfig(msg);
    protocol.vote_on_reconfig("A".to_string(), true);
    protocol.vote_on_reconfig("B".to_string(), true);
    assert!(protocol.commit_reconfig(3));
}

#[test]
fn test_reconfig_checkpoint_application() {
    let mut protocol = {
        let initial_slices = HashMap::new();
        protocol::ProtocolState::new(1, initial_slices)
    };

    let new_slices = HashMap::new();
    let intact: HashSet<String> = vec!["A".to_string()].into_iter().collect();
    let faulty = HashSet::new();
    let proof = IntactnessProof::new(intact, faulty);

    let msg = ReconfigMessage {
        node_id: "A".to_string(),
        slot_seq: 1,
        new_slices,
        proof,
        timestamp: 0,
    };

    protocol.propose_reconfig(msg);
    protocol.vote_on_reconfig("A".to_string(), true);
    protocol.commit_reconfig(1);

    let checkpoint = CheckpointBallot {
        ballot_height: 100,
        applied_slices: HashMap::new(),
    };

    assert!(protocol.apply_reconfig(checkpoint));
    assert_eq!(protocol.state, SMRState::PostReconfig);
}

#[test]
fn test_quorum_slice_validation_after_reconfig() {
    let mut validators = HashSet::new();
    validators.insert("A".to_string());
    validators.insert("B".to_string());
    validators.insert("C".to_string());

    let slice = QuorumSlice::new(2, validators);

    let present: HashSet<String> = vec!["A".to_string(), "B".to_string()]
        .into_iter()
        .collect();
    assert!(slice.is_quorum(&present));

    let insufficient: HashSet<String> = vec!["A".to_string()].into_iter().collect();
    assert!(!slice.is_quorum(&insufficient));
}
