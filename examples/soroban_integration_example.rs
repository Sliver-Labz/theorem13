use smr_protocol::*;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("=== Soroban + SMR Protocol Integration Example ===\n");

    // Initialize Soroban contract bridge
    let mut validator = SorobanReconfigValidator::new("CABC123DEF456".to_string());
    println!("[1] Initialized Soroban contract validator\n");

    // Create reconfig message
    let intact: HashSet<String> = vec!["A".to_string(), "B".to_string(), "D".to_string()]
        .into_iter()
        .collect();
    let faulty: HashSet<String> = vec!["C".to_string(), "E".to_string()].into_iter().collect();
    let proof = IntactnessProof::new(intact, faulty);

    let reconfig = ReconfigMessage {
        node_id: "A".to_string(),
        slot_seq: 1,
        new_slices: HashMap::new(),
        proof,
        timestamp: 1000,
    };
    println!("[2] Created reconfig message for slot 1\n");

    // Validate with Soroban contract
    match validator.validate_with_contract(&reconfig, 1) {
        Ok(_) => println!("[3] ✓ Reconfig validated by Soroban contract\n"),
        Err(e) => println!("[3] ✗ Validation failed: {}\n", e),
    }

    // Inspect contract calls
    let history = validator.get_history();
    println!("[4] Contract call history ({} calls):", history.len());
    for (i, call) in history.iter().enumerate() {
        println!(
            "    Call {}: {} on {}",
            i + 1,
            call.method,
            call.contract_id
        );
    }

    println!("\n=== Integration Complete ===");
}
