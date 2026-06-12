use sha2::{Sha256, Digest};

pub struct MessageValidator;

impl MessageValidator {
    pub fn compute_message_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    pub fn verify_message_integrity(data: &[u8], provided_hash: &str) -> bool {
        Self::compute_message_hash(data) == provided_hash
    }

    pub fn validate_sender_id(sender_id: &str) -> bool {
        !sender_id.is_empty() && sender_id.len() <= 64
    }

    pub fn validate_slot_sequence(slot_seq: u64) -> bool {
        slot_seq > 0
    }

    pub fn validate_timestamp(timestamp: u64, current_time: u64, tolerance_ms: u64) -> bool {
        let diff = if current_time > timestamp {
            current_time - timestamp
        } else {
            timestamp - current_time
        };
        diff <= tolerance_ms
    }
}

pub struct SignatureValidator {
    pub trusted_keys: Vec<String>,
}

impl SignatureValidator {
    pub fn new(trusted_keys: Vec<String>) -> Self {
        Self { trusted_keys }
    }

    pub fn is_trusted_sender(&self, sender_id: &str) -> bool {
        self.trusted_keys.contains(&sender_id.to_string())
    }

    pub fn add_trusted_key(&mut self, key: String) {
        if !self.trusted_keys.contains(&key) {
            self.trusted_keys.push(key);
        }
    }

    pub fn revoke_trusted_key(&mut self, key: &str) -> bool {
        if let Some(pos) = self.trusted_keys.iter().position(|k| k == key) {
            self.trusted_keys.remove(pos);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_hash_consistency() {
        let data = b"test message";
        let hash1 = MessageValidator::compute_message_hash(data);
        let hash2 = MessageValidator::compute_message_hash(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_verify_message_integrity() {
        let data = b"test data";
        let hash = MessageValidator::compute_message_hash(data);
        assert!(MessageValidator::verify_message_integrity(data, &hash));
        assert!(!MessageValidator::verify_message_integrity(b"wrong data", &hash));
    }

    #[test]
    fn test_validate_sender_id() {
        assert!(MessageValidator::validate_sender_id("sender1"));
        assert!(!MessageValidator::validate_sender_id(""));
    }

    #[test]
    fn test_signature_validator() {
        let mut validator = SignatureValidator::new(vec!["A".to_string(), "B".to_string()]);
        assert!(validator.is_trusted_sender("A"));
        assert!(!validator.is_trusted_sender("C"));
        
        validator.add_trusted_key("C".to_string());
        assert!(validator.is_trusted_sender("C"));
        
        assert!(validator.revoke_trusted_key("C"));
        assert!(!validator.is_trusted_sender("C"));
    }
}
