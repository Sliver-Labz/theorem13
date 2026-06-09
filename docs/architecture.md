# SMR Architecture

## Component Overview

### Protocol State Machine
- **Normal**: Standard SCP operation
- **ReconfigProposed**: Reconfig message nominated, intactness proof validation in progress
- **ReconfigCommitted**: Majority votes obtained, awaiting checkpoint ballot
- **Reconfiguring**: Transitioning slice configuration at nominated ballot height
- **PostReconfig**: Slot continues with new quorum slices

### Intactness Validator
Validates that Theorem 13's cumulative intactness condition is preserved:
1. Checks that intact set and faulty set are disjoint
2. Verifies no faulty nodes are added to any well-behaved node's slice
3. Confirms historical votes remain valid under new configuration

### Failure Detector
Decentralized failure detection with configurable timeout:
- Records heartbeats from all nodes
- Detects timeouts, malformed messages, missed votes
- Emits failure events without requiring Byzantine consensus

### Reconfig Coordinator
Orchestrates the entire reconfiguration process:
1. Proposes new quorum slices with intactness proof
2. Collects votes from all nodes
3. Commits when majority approval reached
4. Applies changes at checkpoint ballot atomically

## Data Flow

```
[Failure Detected]
       ↓
[Failure Detector]
       ↓
[Propose Reconfig] → [Intactness Proof Validation]
       ↓
[Vote Collection]
       ↓
[Majority Reached] → [Commit Reconfig]
       ↓
[Checkpoint Ballot] → [Apply Slices Atomically]
       ↓
[Resume Ballot Phase]
```

## Protocol Guarantees

### Safety
- Theorem 13 intactness preserved across all reconfigurations
- No faulty nodes added retroactively to well-behaved paths
- All well-behaved nodes transition at the same logical point

### Liveness
- Minority failures do not block consensus
- Slot continues immediately after reconfiguration applies
- Quorum becomes smaller and more responsive

### Atomicity
- All nodes apply slice changes simultaneously at checkpoint ballot
- No transient configuration mismatches
- Failed nodes cleanly removed from consideration
