use smr_protocol::*;

#[test]
fn test_metrics_single_reconfig_cycle() {
    let mut tracker = StatisticsTracker::new(100);

    tracker.record_slot_start(100);
    tracker.record_reconfig_proposed(105, 25.0);
    tracker.record_reconfig_committed(155, 50.0);

    let stats = tracker.get_statistics();
    assert_eq!(stats.total_slots, 1);
    assert_eq!(stats.successful_reconfigs, 1);
    assert_eq!(stats.failed_reconfigs, 0);
}

#[test]
fn test_metrics_multiple_slots() {
    let mut tracker = StatisticsTracker::new(100);

    for i in 0..5 {
        let base = i * 100;
        tracker.record_slot_start(base);
        if i % 2 == 0 {
            tracker.record_reconfig_proposed(base + 5, 20.0);
            tracker.record_reconfig_committed(base + 50, 45.0);
        }
    }

    let stats = tracker.get_statistics();
    assert_eq!(stats.total_slots, 5);
    assert_eq!(stats.successful_reconfigs, 3);
}

#[test]
fn test_metrics_averaging() {
    let mut tracker = StatisticsTracker::new(100);

    tracker.record_reconfig_committed(100, 40.0);
    tracker.record_reconfig_committed(200, 60.0);

    let stats = tracker.get_statistics();
    assert_eq!(stats.avg_reconfig_latency_ms, 50.0);
}

#[test]
fn test_metrics_event_with_metadata() {
    let event = MetricEvent::new("reconfig".to_string(), 100, 42.0)
        .with_metadata("node".to_string(), "node-1".to_string())
        .with_metadata("slot".to_string(), "5".to_string());

    assert_eq!(event.metadata.len(), 2);
    assert_eq!(event.metadata.get("node").unwrap(), "node-1");
}
