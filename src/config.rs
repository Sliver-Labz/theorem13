use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<SMRConfig, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;
        toml::from_str(&content).map_err(|e| format!("Failed to parse TOML: {}", e))
    }

    pub fn from_str(content: &str) -> Result<SMRConfig, String> {
        toml::from_str(content).map_err(|e| format!("Failed to parse TOML: {}", e))
    }

    pub fn to_file<P: AsRef<Path>>(config: &SMRConfig, path: P) -> Result<(), String> {
        let content = toml::to_string_pretty(config).map_err(|e| format!("Failed to serialize config: {}", e))?;
        fs::write(path, content).map_err(|e| format!("Failed to write config file: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::default()
            .enabled(true)
            .max_reconfigs_per_slot(5)
            .build()
            .unwrap();

        assert!(config.enabled);
        assert_eq!(config.max_reconfigs_per_slot, 5);
    }

    #[test]
    fn test_config_invalid() {
        let result = ConfigBuilder::default()
            .max_reconfigs_per_slot(0)
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_config_from_str() {
        let toml_str = r#"
enabled = true
max_reconfigs_per_slot = 4
failure_detection_threshold_ms = 6000
intactness_proof_timeout_ms = 3000
checkpoint_ballot_height = 100
"#;
        let config = ConfigLoader::from_str(toml_str).unwrap();
        assert_eq!(config.max_reconfigs_per_slot, 4);
        assert_eq!(config.checkpoint_ballot_height, 100);
    }
}
