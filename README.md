# Safe Mid-Slot Reconfiguration (SMR) Protocol

A protocol for changing quorum slices mid-slot that preserves the cumulative intactness conditions of Theorem 13 — allowing failed nodes to be safely dropped without poisoning well-behaved peers in Stellar Consensus Protocol (SCP).

---

## Problem

In Stellar Consensus Protocol, once a slot begins, the quorum configuration is fixed. When nodes fail mid-slot, the consensus set becomes bloated with failed peers, risking:

- **Consensus blocking** — if too many well-behaved nodes depend on failed peers in their quorum slices, nomination and ballot phases can deadlock
- **Intactness violation** — adding faulty nodes to quorum slices retroactively contaminates the cumulative intactness guarantees that Theorem 13 provides
- **No safe recovery path** — existing protocols require aborting the slot or waiting for it to complete before reconfiguring

---

## Solution

SMR enables **safe mid-slot quorum reconfigurations** by:

1. **Theorem 13 preservation** — tracks cumulative intactness conditions throughout the slot and proves that slice changes don't retroactively add faulty nodes to well-behaved paths
2. **Coordinated updates** — nodes update their quorum slices atomically at a nominated checkpoint, preventing transient configuration mismatches
3. **Failure isolation** — failed nodes are dropped from well-behaved nodes' slices without requiring global consensus on who failed
4. **Consensus continuity** — the slot progresses immediately after reconfig with smaller, more responsive quorum sets

---

## Architecture

```
SCP Slot Execution
├── Nomination Phase (rounds 1–n)
│   └── Normal nomination with initial quorum slices
├── [Mid-slot Failure Detected]
│   └── Proposed reconfiguration message
├── SMR Reconfig Window
│   ├── Validity check (Theorem 13 intactness proof)
│   ├── Commitment-phase votes on new slices
│   └── Atomic slice update at checkpoint ballot
└── Ballot Phase (resumed with new slices)
    └── Continue consensus with smaller, healthier quorum sets
```

---

## Key Components

### 1. Intactness Proof

Verifies that Theorem 13's cumulative intactness holds across all nodes' proposed slice changes. Guarantees:

- No faulty node is added to any well-behaved node's slice
- The union of all faulty nodes stays disjoint from the intact set
- Historical votes remain valid under the new configuration

### 2. Coordinated Update Protocol

Ensures all well-behaved nodes transition quorum slices at the same logical point in the slot:

- Nomination of a reconfig message containing new slice definitions
- Commitment phase where nodes validate the intactness proof
- Atomic application at a nominated ballot height

### 3. Failure Detection & Propagation

Decentralized detection of failed nodes (timeouts, malformed messages, missed votes) without requiring Byzantine agreement on the failure itself.

---

## Prerequisites

| Component | Version | Installation |
|-----------|---------|--------------|
| Rust | 1.70+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| SCP Node | Latest | [Stellar Core](https://github.com/stellar/stellar-core) |

---

## Getting Started

### 1. Clone

```bash
git clone https://github.com/theorem13/smr-protocol.git
cd smr-protocol
```

### 2. Build

```bash
cargo build --release
```

### 3. Run tests

```bash
cargo test --all
```

### 4. Integration with SCP Node

Compile the SMR module and link it into your Stellar Core build:

```bash
./scripts/integrate-stellar-core.sh /path/to/stellar-core
```

This patches the SCP ballot and nomination logic to invoke SMR reconfig checks.

---

## How It Works

### Example Scenario

**Initial state:** 5 nodes (A, B, C, D, E) all well-behaved. Each trusts all 4 others in their quorum slices.

**Mid-slot:** Nodes C and E fail (network partition, crash).

**Without SMR:** A, B, and D are stuck waiting for votes from C and E. No safe way to remove them mid-slot.

**With SMR:**

1. A, B, D each detect C and E are unresponsive and nominate a reconfig message
2. All nodes run the intactness proof — confirms removing C and E doesn't violate Theorem 13
3. Nodes vote on the reconfig and commit at a agreed ballot
4. At the checkpoint ballot, A, B, D atomically update slices: `{A, B, D}` instead of `{A, B, C, D, E}`
5. Nomination and ballot phases resume with a 3-node quorum instead of 5, making progress immediately

---

## Protocol States

| State | Behavior |
|-------|----------|
| `Normal` | Standard SCP nomination and ballot phases with current slices |
| `ReconfigProposed` | Reconfig message nominated; nodes validating intactness proof |
| `ReconfigCommitted` | Sufficient votes on reconfig; awaiting checkpoint ballot to apply |
| `Reconfiguring` | Transitioning slices at the nominated ballot height |
| `PostReconfig` | Slot continues with updated quorum slices |

---

## Safety Properties

### Theorem 13 Intactness

The cumulative intactness condition is preserved across all mid-slot reconfigurations. Formally:

> After reconfig, the well-behaved nodes and their cumulative intact sets remain disjoint from the set of faulty nodes.

This is proven via the intactness proof subsystem before any slice changes are applied.

### Consensus Liveness

If a minority of nodes fail, the protocol guarantees that the remaining well-behaved majority can form a quorum and continue the slot to completion.

### Atomicity

All well-behaved nodes apply slice changes at the same logical ballot height, preventing configuration inconsistency windows.

---

## Configuration

SMR behavior is controlled via SCP node parameters:

```toml
[smr]
enabled = true
max_reconfigs_per_slot = 3
failure_detection_threshold_ms = 5000
intactness_proof_timeout_ms = 2000
checkpoint_ballot_height = 0  # 0 = auto-select
```

---

## Testing

Run the full test suite:

```bash
cargo test --all -- --nocapture
```

Run specific tests:

```bash
cargo test smr::tests::test_intactness_preservation -- --nocapture
cargo test smr::tests::test_atomic_slice_update -- --nocapture
```

---

## Benchmarks

Profile consensus latency with and without mid-slot reconfigs:

```bash
cargo bench --all
```

Reports include:
- Nomination latency improvement (post-reconfig vs. pre-reconfig)
- Ballot phase throughput
- Intactness proof verification time
- Message overhead

---

## Contributing

1. Fork the repo
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Commit with conventional commits: `git commit -m "feat: add X"`
4. Open a pull request

---

## License

MIT — see [LICENSE](./LICENSE).

---

## References

- **Theorem 13** — Mazieres, D. "The Stellar Consensus Protocol." SCP White Paper.
- **SCP Specification** — [stellar-protocol/core/cap-0005](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0005.md)
- **Stellar Docs** — [developers.stellar.org](https://developers.stellar.org)
