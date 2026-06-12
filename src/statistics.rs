use crate::metrics::MetricsCollector;

pub struct ConsensusStatistics {
    pub total_slots: u64,
    pub successful_reconfigs: u64,
    pub failed_reconfigs: u64,
    pub avg_reconfig_latency_ms: f64,
    pub avg_consensus_latency_ms: f64,
}

pub struct StatisticsTracker {
    metrics: MetricsCollector,
}

impl StatisticsTracker {
    pub fn new(max_events: usize) -> Self {
        Self {
            metrics: MetricsCollector::new(max_events),
        }
    }

    pub fn record_slot_start(&mut self, timestamp: u64) {
        let event = crate::metrics::MetricEvent::new(
            "slot_start".to_string(),
            timestamp,
            1.0,
        );
        self.metrics.record(event);
    }

    pub fn record_reconfig_proposed(&mut self, timestamp: u64, latency_ms: f64) {
        let event = crate::metrics::MetricEvent::new(
            "reconfig_proposed".to_string(),
            timestamp,
            latency_ms,
        );
        self.metrics.record(event);
    }

    pub fn record_reconfig_committed(&mut self, timestamp: u64, latency_ms: f64) {
        let event = crate::metrics::MetricEvent::new(
            "reconfig_committed".to_string(),
            timestamp,
            latency_ms,
        );
        self.metrics.record(event);
    }

    pub fn get_statistics(&self) -> ConsensusStatistics {
        let total_slots = self.metrics.count_events("slot_start") as u64;
        let successful_reconfigs = self.metrics.count_events("reconfig_committed") as u64;
        let failed_reconfigs = self.metrics.count_events("reconfig_proposed") as u64
            - successful_reconfigs;

        ConsensusStatistics {
            total_slots,
            successful_reconfigs,
            failed_reconfigs,
            avg_reconfig_latency_ms: self.metrics.average_value("reconfig_committed").unwrap_or(0.0),
            avg_consensus_latency_ms: self.metrics.average_value("slot_start").unwrap_or(0.0),
        }
    }
}
