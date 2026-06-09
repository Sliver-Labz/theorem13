use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smr_protocol::{QuorumSlice, IntactnessProof, ProtocolState, ReconfigMessage};
use std::collections::{HashMap, HashSet};

fn benchmark_intactness_validation(c: &mut Criterion) {
    c.bench_function("intactness_proof_generation", |b| {
        b.iter(|| {
            let intact: HashSet<String> = vec!["A".to_string(), "B".to_string()]
                .into_iter()
                .collect();
            let faulty = HashSet::new();
            IntactnessProof::new(black_box(intact), black_box(faulty))
        })
    });
}

fn benchmark_reconfig_proposal(c: &mut Criterion) {
    c.bench_function("reconfig_proposal_creation", |b| {
        b.iter(|| {
            let new_slices = HashMap::new();
            let intact: HashSet<String> = vec!["A".to_string()].into_iter().collect();
            let faulty = HashSet::new();
            let proof = IntactnessProof::new(intact, faulty);

            ReconfigMessage {
                node_id: "A".to_string(),
                slot_seq: 1,
                new_slices,
                proof,
                timestamp: 0,
            }
        })
    });
}

fn benchmark_protocol_transitions(c: &mut Criterion) {
    c.bench_function("state_transitions", |b| {
        b.iter(|| {
            let mut state = ProtocolState::new(black_box(1), HashMap::new());
            let _ = state.propose_reconfig(ReconfigMessage {
                node_id: "A".to_string(),
                slot_seq: 1,
                new_slices: HashMap::new(),
                proof: IntactnessProof::new(
                    vec!["A".to_string()].into_iter().collect(),
                    HashSet::new(),
                ),
                timestamp: 0,
            });
        })
    });
}

criterion_group!(
    benches,
    benchmark_intactness_validation,
    benchmark_reconfig_proposal,
    benchmark_protocol_transitions
);
criterion_main!(benches);
