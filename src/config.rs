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

    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

pub struct ConfigBuilder {
    config: SMRConfig,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            config: SMRConfig::default(),
        }
    }
}

impl ConfigBuilder {
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.config.enabled = enabled;
        self
    }

    pub fn max_reconfigs_per_slot(mut self, max: usize) -> Self {
        self.config.max_reconfigs_per_slot = max;
        self
    }

    pub fn failure_detection_threshold_ms(mut self, ms: u64) -> Self {
        self.config.failure_detection_threshold_ms = ms;
        self
    }

    pub fn intactness_proof_timeout_ms(mut self, ms: u64) -> Self {
        self.config.intactness_proof_timeout_ms = ms;
        self
    }

    pub fn checkpoint_ballot_height(mut self, height: u64) -> Self {
        self.config.checkpoint_ballot_height = height;
        self
    }

    pub fn build(self) -> Result<SMRConfig, String> {
        if !self.config.is_valid() {
            return Err("Invalid SMR configuration".to_string());
        }
        Ok(self.config)
    }
}
