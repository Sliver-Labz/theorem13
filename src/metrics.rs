use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MetricEvent {
    pub event_type: String,
    pub timestamp: u64,
    pub value: f64,
    pub metadata: HashMap<String, String>,
}

impl MetricEvent {
    pub fn new(event_type: String, timestamp: u64, value: f64) -> Self {
        Self {
            event_type,
            timestamp,
            value,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

pub struct MetricsCollector {
    events: Vec<MetricEvent>,
    max_events: usize,
}

impl MetricsCollector {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Vec::new(),
            max_events,
        }
    }

    pub fn record(&mut self, event: MetricEvent) {
        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }
        self.events.push(event);
    }

    pub fn get_events(&self) -> &[MetricEvent] {
        &self.events
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn count_events(&self, event_type: &str) -> usize {
        self.events.iter().filter(|e| e.event_type == event_type).count()
    }

    pub fn average_value(&self, event_type: &str) -> Option<f64> {
        let values: Vec<f64> = self.events
            .iter()
            .filter(|e| e.event_type == event_type)
            .map(|e| e.value)
            .collect();

        if values.is_empty() {
            return None;
        }

        let sum: f64 = values.iter().sum();
        Some(sum / values.len() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_event_creation() {
        let event = MetricEvent::new("latency".to_string(), 100, 45.5);
        assert_eq!(event.event_type, "latency");
        assert_eq!(event.value, 45.5);
    }

    #[test]
    fn test_metrics_collector_recording() {
        let mut collector = MetricsCollector::new(10);
        let event = MetricEvent::new("latency".to_string(), 100, 50.0);
        collector.record(event);
        assert_eq!(collector.get_events().len(), 1);
    }

    #[test]
    fn test_metrics_collector_count() {
        let mut collector = MetricsCollector::new(10);
        collector.record(MetricEvent::new("latency".to_string(), 100, 50.0));
        collector.record(MetricEvent::new("latency".to_string(), 101, 55.0));
        collector.record(MetricEvent::new("throughput".to_string(), 102, 100.0));
        
        assert_eq!(collector.count_events("latency"), 2);
        assert_eq!(collector.count_events("throughput"), 1);
    }

    #[test]
    fn test_metrics_collector_average() {
        let mut collector = MetricsCollector::new(10);
        collector.record(MetricEvent::new("latency".to_string(), 100, 40.0));
        collector.record(MetricEvent::new("latency".to_string(), 101, 60.0));
        
        assert_eq!(collector.average_value("latency"), Some(50.0));
    }
}
