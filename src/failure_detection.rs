use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FailureEvent {
    pub node_id: String,
    pub timestamp: u64,
    pub reason: FailureReason,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FailureReason {
    Timeout,
    MalformedMessage,
    MissedVote,
    NetworkPartition,
    Other(String),
}

pub struct FailureDetector {
    timeout_ms: u64,
    last_seen: HashMap<String, u64>,
    detected_failures: Vec<FailureEvent>,
}

impl FailureDetector {
    pub fn new(timeout_ms: u64) -> Self {
        Self {
            timeout_ms,
            last_seen: HashMap::new(),
            detected_failures: Vec::new(),
        }
    }

    pub fn heartbeat(&mut self, node_id: &str) {
        let now = Self::now_ms();
        self.last_seen.insert(node_id.to_string(), now);
    }

    pub fn check_timeouts(&mut self) -> Vec<String> {
        let now = Self::now_ms();
        let mut failed = Vec::new();

        for (node_id, last_seen) in &self.last_seen {
            if now - last_seen > self.timeout_ms {
                failed.push(node_id.clone());
                self.record_failure(node_id.clone(), FailureReason::Timeout);
            }
        }

        failed
    }

    pub fn record_message_failure(&mut self, node_id: String, malformed: bool) {
        let reason = if malformed {
            FailureReason::MalformedMessage
        } else {
            FailureReason::MissedVote
        };
        self.record_failure(node_id, reason);
    }

    fn record_failure(&mut self, node_id: String, reason: FailureReason) {
        let event = FailureEvent {
            node_id,
            timestamp: Self::now_ms(),
            reason,
        };
        self.detected_failures.push(event);
    }

    pub fn get_failures(&self) -> &[FailureEvent] {
        &self.detected_failures
    }

    pub fn clear_failures(&mut self) {
        self.detected_failures.clear();
    }

    fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heartbeat_recording() {
        let mut detector = FailureDetector::new(5000);
        detector.heartbeat("A");
        assert!(detector.last_seen.contains_key("A"));
    }

    #[test]
    fn test_multiple_nodes() {
        let mut detector = FailureDetector::new(5000);
        detector.heartbeat("A");
        detector.heartbeat("B");
        detector.heartbeat("C");
        assert_eq!(detector.last_seen.len(), 3);
    }

    #[test]
    fn test_record_failure() {
        let mut detector = FailureDetector::new(5000);
        detector.record_message_failure("A".to_string(), true);
        assert_eq!(detector.get_failures().len(), 1);
        assert_eq!(detector.get_failures()[0].reason, FailureReason::MalformedMessage);
    }

    #[test]
    fn test_clear_failures() {
        let mut detector = FailureDetector::new(5000);
        detector.record_message_failure("A".to_string(), false);
        detector.clear_failures();
        assert_eq!(detector.get_failures().len(), 0);
    }
}
