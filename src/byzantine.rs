use crate::network::NetworkMessage;
use crate::validation::{MessageValidator, SignatureValidator};
use std::collections::HashSet;

pub struct ByzantineMessageFilter {
    validator: SignatureValidator,
    message_history: HashSet<String>,
    max_history_size: usize,
}

impl ByzantineMessageFilter {
    pub fn new(trusted_keys: Vec<String>, max_history_size: usize) -> Self {
        Self {
            validator: SignatureValidator::new(trusted_keys),
            message_history: HashSet::new(),
            max_history_size,
        }
    }

    pub fn validate_message(
        &mut self,
        msg: &NetworkMessage,
        current_time: u64,
        time_tolerance_ms: u64,
    ) -> bool {
        if !self.validator.is_trusted_sender(&msg.sender_id) {
            return false;
        }

        if !MessageValidator::validate_sender_id(&msg.sender_id) {
            return false;
        }

        if !MessageValidator::validate_slot_sequence(msg.slot_seq) {
            return false;
        }

        if !MessageValidator::validate_timestamp(msg.timestamp, current_time, time_tolerance_ms) {
            return false;
        }

        let msg_hash = MessageValidator::compute_message_hash(&msg.payload);
        if self.message_history.contains(&msg_hash) {
            return false;
        }

        if self.message_history.len() >= self.max_history_size {
            self.message_history.clear();
        }

        self.message_history.insert(msg_hash);
        true
    }

    pub fn add_trusted_sender(&mut self, sender_id: String) {
        self.validator.add_trusted_key(sender_id);
    }

    pub fn remove_trusted_sender(&mut self, sender_id: &str) -> bool {
        self.validator.revoke_trusted_key(sender_id)
    }
}
