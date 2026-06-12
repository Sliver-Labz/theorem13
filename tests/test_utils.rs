use smr_protocol::{QuorumSlice, Node, SMRConfig};
use std::collections::{HashMap, HashSet};

pub fn create_test_quorum_slices(node_ids: Vec<&str>) -> HashMap<String, QuorumSlice> {
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

pub fn create_test_config() -> SMRConfig {
    SMRConfig {
        enabled: true,
        max_reconfigs_per_slot: 3,
        failure_detection_threshold_ms: 5000,
        intactness_proof_timeout_ms: 2000,
        checkpoint_ballot_height: 0,
    }
}

pub fn create_nodes(count: usize) -> Vec<Node> {
    (0..count)
        .map(|i| Node {
            id: format!("node-{}", i),
            is_well_behaved: true,
        })
        .collect()
}
