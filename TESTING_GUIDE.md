# SMR Protocol Testing Guide

## Test Organization

The test suite is organized into focused test files covering different aspects:

```
tests/
├── test_utils.rs           # Shared test helpers
├── integration_tests.rs    # Cross-module smoke tests
├── test_reconfig_flow.rs   # State machine workflows
├── test_components.rs      # Individual component tests
├── test_scenarios.rs       # Realistic network scenarios
├── test_serialization.rs   # Message format stability
├── test_failure_scenarios.rs # Failure handling
└── test_metrics_e2e.rs     # Statistics collection
```

## Test Categories

### 1. Unit Tests
**Focus**: Individual component functionality in isolation

**Examples**:
- `test_quorum_slice_formation` - Quorum slice creation
- `test_message_hash_consistency` - Hash determinism
- `test_config_builder_validation` - Configuration validation
- `test_metrics_collector_average` - Metric aggregation

**Run**:
```bash
cargo test --lib
```

### 2. Integration Tests
**Focus**: Multiple components working together

**Examples**:
- `test_e2e_normal_consensus` - Normal path consensus
- `test_e2e_message_creation_and_serialization` - Message roundtrip
- `test_reconfig_proposal_flow` - State machine transitions
- `test_network_broadcast` - P2P message dissemination

**Run**:
```bash
cargo test --test integration_tests
```

### 3. Scenario Tests
**Focus**: Realistic consensus scenarios

**Examples**:
- `scenario_5_node_network_nominal` - 5-node stable network
- `scenario_detect_and_filter_byzantine` - Byzantine message filtering
- `scenario_message_queue_overflow` - Queue capacity handling
- `scenario_quorum_formation_3_of_5` - Threshold quorum validation

**Run**:
```bash
cargo test --test test_scenarios
```

### 4. Failure Tests
**Focus**: Failure detection and recovery

**Examples**:
- `test_failure_detection_timeout` - Node timeout detection
- `test_failure_detection_multiple_nodes_different_times` - Multi-node timeouts
- `test_byzantine_filter_timestamp_tolerance` - Timestamp validation
- `scenario_message_queue_overflow` - Queue saturation handling

**Run**:
```bash
cargo test --test test_failure_scenarios
```

### 5. Metrics Tests
**Focus**: Event tracking and statistics

**Examples**:
- `test_metrics_single_reconfig_cycle` - Single reconfig cycle
- `test_metrics_multiple_slots` - Multi-slot aggregation
- `test_metrics_averaging` - Statistical averaging
- `test_metrics_event_with_metadata` - Event metadata tracking

**Run**:
```bash
cargo test --test test_metrics_e2e
```

## Running Tests

### All tests
```bash
cargo test --all
```

### Specific test file
```bash
cargo test --test integration_tests
cargo test --test test_reconfig_flow
```

### Specific test function
```bash
cargo test test_e2e_config_loading
cargo test scenario_5_node_network
```

### With output
```bash
cargo test -- --nocapture
cargo test --test integration_tests -- --nocapture --test-threads=1
```

### Verbose mode
```bash
cargo test --all -- --nocapture --test-threads=1 2>&1 | less
```

## Test Scenarios

### Scenario 1: Normal Consensus
**Setup**: 3 nodes, all well-behaved
**Flow**: Config load → Quorum validation → Metrics recording
**Expected**: All components interact successfully

```rust
#[test]
fn test_e2e_normal_consensus() {
    let slices = test_utils::create_test_quorum_slices(vec!["A", "B", "C"]);
    let config = test_utils::create_config();
    assert!(config.is_valid());
}
```

### Scenario 2: Failure Detection
**Setup**: 5 nodes, 2 go offline
**Flow**: Heartbeat → Timeout → Failure event
**Expected**: Failed nodes detected, healthy nodes continue

```rust
#[test]
fn test_failure_detection_timeout() {
    let mut detector = FailureDetector::new(1000);
    detector.heartbeat("Node-A".to_string());
    let failed = detector.check_timeouts(2000);
    assert!(!failed.is_empty());
}
```

### Scenario 3: Reconfig Workflow
**Setup**: Initial 5-node quorum, 2 nodes fail
**Flow**: Propose → Vote → Commit → Apply
**Expected**: State transitions correctly, new quorum formed

```rust
#[test]
fn test_reconfig_checkpoint_application() {
    let mut protocol = protocol::ProtocolState::new(1, HashMap::new());
    // ... propose, vote, commit ...
    protocol.apply_reconfig(checkpoint);
    assert_eq!(protocol.state, SMRState::PostReconfig);
}
```

### Scenario 4: Byzantine Filtering
**Setup**: Untrusted sender, invalid message
**Flow**: Validate sender → Check timestamp → Verify hash
**Expected**: Malicious messages rejected

```rust
#[test]
fn test_byzantine_filter_untrusted_sender() {
    let filter = ByzantineMessageFilter::new(
        vec!["A".to_string()],
        10,
    );
    let msg = NetworkMessage::new(..., "Attacker".to_string(), ...);
    assert!(!filter.validate_message(&msg, 1000, 5000));
}
```

### Scenario 5: Metrics Collection
**Setup**: Multiple slots and reconfigs
**Flow**: Record events → Aggregate → Compute statistics
**Expected**: Accurate metrics over reporting window

```rust
#[test]
fn test_metrics_multiple_slots() {
    let mut tracker = StatisticsTracker::new(100);
    for i in 0..5 {
        tracker.record_slot_start(i * 100);
        tracker.record_reconfig_committed(i * 100 + 50, 45.0);
    }
    let stats = tracker.get_statistics();
    assert_eq!(stats.total_slots, 5);
}
```

## Test Utilities

### Helper Functions in `test_utils.rs`

```rust
// Create pre-configured quorum slices for testing
pub fn create_test_quorum_slices(node_ids: Vec<&str>) 
    -> HashMap<String, QuorumSlice>

// Create default SMR configuration
pub fn create_test_config() -> SMRConfig

// Create test nodes
pub fn create_nodes(count: usize) -> Vec<Node>
```

## Debugging Failed Tests

### Option 1: Run with output
```bash
cargo test --all -- --nocapture
```

### Option 2: Run single-threaded
```bash
cargo test test_name -- --test-threads=1
```

### Option 3: Run with Rust backtrace
```bash
RUST_BACKTRACE=1 cargo test
```

### Option 4: Filter by test name
```bash
cargo test failure_detection
cargo test metrics
```

## Adding New Tests

### Pattern for new test file:
```rust
use smr_protocol::*;

#[test]
fn test_new_feature() {
    // Arrange: Set up test data
    let mut component = Component::new();
    
    // Act: Perform operation
    let result = component.operation();
    
    // Assert: Verify outcome
    assert_eq!(result, expected);
}
```

### Naming conventions:
- Unit test: `test_component_operation`
- Integration: `test_e2e_feature_flow`
- Scenario: `scenario_description`
- Failure: `test_failure_condition`

## Performance Testing

### Running benchmarks:
```bash
cargo bench --all
```

### Benchmarks measure:
- Intactness proof generation
- Message serialization
- State machine transitions
- Byzantine filtering overhead

---

## CI/CD Integration

Tests run automatically on:
- Push to any branch
- Pull requests

Configuration in `.github/workflows/rust-ci.yml`:
```yaml
- name: Run tests
  run: cargo test --all -- --nocapture
```

## Test Coverage Goals

| Component | Target Coverage | Current |
|-----------|-----------------|---------|
| Core logic | 85%+ | ✓ |
| Network | 80%+ | ✓ |
| Validation | 90%+ | ✓ |
| Metrics | 85%+ | ✓ |
| Overall | 85%+ | ✓ |
