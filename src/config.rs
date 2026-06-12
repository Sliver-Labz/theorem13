use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMRConfig {
    pub enabled: bool,
    pub max_reconfigs_per_slot: usize,
    pub failure_detection_threshold_ms: u64,
    pub intactness_proof_timeout_ms: u64,
    pub checkpoint_ballot_height: u64,
}

impl Default for SMRConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_reconfigs_per_slot: 3,
            failure_detection_threshold_ms: 5000,
            intactness_proof_timeout_ms: 2000,
            checkpoint_ballot_height: 0,
        }
    }
}

impl SMRConfig {
    pub fn is_valid(&self) -> bool {
        self.max_reconfigs_per_slot > 0
            && self.failure_detection_threshold_ms > 0
            && self.intactness_proof_timeout_ms > 0
    }
}
