# SMR Protocol Public API Reference

## Configuration Module

### `SMRConfig`
```rust
pub struct SMRConfig {
    pub enabled: bool,
    pub max_reconfigs_per_slot: usize,
    pub failure_detection_threshold_ms: u64,
    pub intactness_proof_timeout_ms: u64,
    pub checkpoint_ballot_height: u64,
}
```

### `ConfigBuilder`
```rust
let config = ConfigBuilder::default()
    .enabled(true)
    .max_reconfigs_per_slot(3)
    .failure_detection_threshold_ms(5000)
    .intactness_proof_timeout_ms(2000)
    .checkpoint_ballot_height(0)
    .build()?;
```

### `ConfigLoader`
```rust
let config = ConfigLoader::from_file("smr.toml")?;
ConfigLoader::to_file(&config, "backup.toml")?;
let config = ConfigLoader::from_str(toml_string)?;
```

---

## Network Module

### `NetworkMessage`
```rust
let msg = NetworkMessage::new(
    MessageType::ReconfigProposal,
    "node-id".to_string(),
    slot_seq,
    payload,
    sequence_num,
    timestamp,
);

if msg.is_reconfig() {
    // Handle reconfig-related messages
}
```

### `MessageQueue`
```rust
let mut queue = MessageQueue::new(max_size);
queue.enqueue(msg)?;
if let Some(msg) = queue.dequeue() {
    // Process message
}
```

### `MessageSerializer`
```rust
let bytes = MessageSerializer::serialize(&msg)?;
let msg = MessageSerializer::deserialize(&bytes)?;
```

### `P2PNode`
```rust
let mut node = P2PNode::new("node-1".to_string());
node.add_peer("peer-1".to_string());

let broadcasts = node.broadcast(msg);
let unicast = node.unicast("peer-1", msg)?;
```

---

## Validation Module

### `MessageValidator`
```rust
MessageValidator::validate_sender_id(sender_id)?;
MessageValidator::validate_slot_sequence(slot_seq)?;
MessageValidator::validate_timestamp(ts, current_ts, tolerance)?;

let hash = MessageValidator::compute_message_hash(data);
MessageValidator::verify_message_integrity(data, &hash)?;
```

### `SignatureValidator`
```rust
let mut validator = SignatureValidator::new(trusted_keys);
if validator.is_trusted_sender(sender_id) {
    validator.add_trusted_key(key);
    validator.revoke_trusted_key(key)?;
}
```

---

## Byzantine Module

### `ByzantineMessageFilter`
```rust
let mut filter = ByzantineMessageFilter::new(trusted_keys, max_history);
if filter.validate_message(&msg, current_time, time_tolerance_ms) {
    filter.add_trusted_sender(sender_id);
    filter.remove_trusted_sender(sender_id)?;
}
```

---

## Metrics Module

### `MetricEvent`
```rust
let event = MetricEvent::new("latency".to_string(), timestamp, 45.5)
    .with_metadata("node".to_string(), "A".to_string());
```

### `MetricsCollector`
```rust
let mut collector = MetricsCollector::new(max_events);
collector.record(event);
let avg = collector.average_value("latency");
let count = collector.count_events("latency");
```

---

## Statistics Module

### `StatisticsTracker`
```rust
let mut tracker = StatisticsTracker::new(max_events);
tracker.record_slot_start(ts);
tracker.record_reconfig_proposed(ts, latency_ms);
tracker.record_reconfig_committed(ts, latency_ms);

let stats = tracker.get_statistics();
println!("Success rate: {}/{}", 
    stats.successful_reconfigs, 
    stats.total_slots);
```

---

## Complete Example

```rust
use smr_protocol::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config
    let config = ConfigLoader::from_file("smr.toml")?;
    
    // Create P2P node
    let mut node = P2PNode::new("node-1".to_string());
    node.add_peer("node-2".to_string());
    
    // Setup validation
    let mut filter = ByzantineMessageFilter::new(
        vec!["node-1".to_string(), "node-2".to_string()],
        100,
    );
    
    // Track metrics
    let mut tracker = StatisticsTracker::new(1000);
    tracker.record_slot_start(0);
    
    // Create reconfig message
    let msg = NetworkMessage::new(
        MessageType::ReconfigProposal,
        "node-1".to_string(),
        1,
        vec![],
        0,
        0,
    );
    
    // Validate and broadcast
    if filter.validate_message(&msg, 0, 5000) {
        let broadcasts = node.broadcast(msg);
        tracker.record_reconfig_proposed(0, 50.0);
    }
    
    // Report statistics
    let stats = tracker.get_statistics();
    println!("{:?}", stats);
    
    Ok(())
}
```
