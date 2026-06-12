// Full end-to-end SMR reconfiguration workflow example
use smr_protocol::*;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("=== SMR Protocol Full Workflow Example ===\n");

    // 1. Initialize configuration
    println!("[1] Initializing SMR configuration...");
    let config = ConfigBuilder::default()
        .enabled(true)
        .max_reconfigs_per_slot(3)
        .failure_detection_threshold_ms(5000)
        .intactness_proof_timeout_ms(2000)
        .build()
        .expect("Failed to build config");
    println!("    Config valid: {}\n", config.is_valid());

    // 2. Set up initial quorum slices
    println!("[2] Setting up 5-node network with quorum slices...");
    let node_ids = vec!["A", "B", "C", "D", "E"];
    let mut slices = HashMap::new();
    let validators: HashSet<String> = node_ids.iter().map(|id| id.to_string()).collect();
    let threshold = 3;

    for id in &node_ids {
        slices.insert(id.to_string(), QuorumSlice::new(threshold, validators.clone()));
    }
    println!("    Created {} quorum slices with threshold {}\n", slices.len(), threshold);

    // 3. Create protocol state machine
    println!("[3] Creating protocol state machine...");
    let mut protocol = protocol::ProtocolState::new(1, slices.clone());
    println!("    Initial state: {:?}\n", protocol.state);

    // 4. Detect failures
    println!("[4] Failure detection: nodes C and E go offline...");
    let mut detector = FailureDetector::new(5000);
    for id in &node_ids {
        detector.heartbeat(id.to_string());
    }
    let failed = detector.check_timeouts(10000);
    println!("    Failed nodes: {:?}\n", failed);

    // 5. Create reconfig proposal
    println!("[5] Creating reconfiguration proposal...");
    let intact_set: HashSet<String> = vec!["A".to_string(), "B".to_string(), "D".to_string()]
        .into_iter()
        .collect();
    let faulty_set: HashSet<String> = vec!["C".to_string(), "E".to_string()].into_iter().collect();
    let proof = IntactnessProof::new(intact_set, faulty_set);

    let mut new_slices = HashMap::new();
    let healthy_validators: HashSet<String> =
        vec!["A".to_string(), "B".to_string(), "D".to_string()]
            .into_iter()
            .collect();
    for id in vec!["A", "B", "D"] {
        new_slices.insert(
            id.to_string(),
            QuorumSlice::new(2, healthy_validators.clone()),
        );
    }

    let reconfig_msg = ReconfigMessage {
        node_id: "A".to_string(),
        slot_seq: 1,
        new_slices,
        proof,
        timestamp: 100,
    };

    println!("    Reconfig proposal created");
    println!("    Intactness proof valid: {}\n", reconfig_msg.proof.is_disjoint());

    // 6. Propose reconfiguration
    println!("[6] Proposing reconfiguration...");
    if protocol.propose_reconfig(reconfig_msg) {
        println!("    State transitioned to: {:?}\n", protocol.state);
    }

    // 7. Voting phase
    println!("[7] Voting on reconfiguration...");
    for node_id in vec!["A", "B", "D"] {
        protocol.vote_on_reconfig(node_id.to_string(), true);
    }
    println!("    Votes recorded: {}\n", protocol.votes.len());

    // 8. Commitment
    println!("[8] Committing reconfiguration...");
    if protocol.commit_reconfig(5) {
        println!("    State transitioned to: {:?}\n", protocol.state);
    }

    // 9. Apply at checkpoint
    println!("[9] Applying reconfiguration at checkpoint...");
    let checkpoint = CheckpointBallot {
        ballot_height: 100,
        applied_slices: HashMap::new(),
    };
    if protocol.apply_reconfig(checkpoint) {
        println!("    State transitioned to: {:?}\n", protocol.state);
    }

    // 10. Metrics reporting
    println!("[10] Recording metrics...");
    let mut tracker = StatisticsTracker::new(100);
    tracker.record_slot_start(0);
    tracker.record_reconfig_proposed(5, 50.0);
    tracker.record_reconfig_committed(155, 75.0);

    let stats = tracker.get_statistics();
    println!(
        "    Total slots: {}, Successful reconfigs: {}, Failed: {}\n",
        stats.total_slots, stats.successful_reconfigs, stats.failed_reconfigs
    );

    println!("=== Workflow Complete ===");
}
