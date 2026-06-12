use serde::{Deserialize, Serialize};
use crate::protocol::ReconfigMessage;
use std::collections::VecDeque;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageType {
    ReconfigProposal,
    ReconfigVote,
    Heartbeat,
    Acknowledgment,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub msg_type: MessageType,
    pub sender_id: String,
    pub slot_seq: u64,
    pub payload: Vec<u8>,
    pub sequence_num: u64,
    pub timestamp: u64,
}

impl NetworkMessage {
    pub fn new(
        msg_type: MessageType,
        sender_id: String,
        slot_seq: u64,
        payload: Vec<u8>,
        sequence_num: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            msg_type,
            sender_id,
            slot_seq,
            payload,
            sequence_num,
            timestamp,
        }
    }

    pub fn is_reconfig(&self) -> bool {
        matches!(self.msg_type, MessageType::ReconfigProposal | MessageType::ReconfigVote)
    }
}

pub struct MessageQueue {
    messages: VecDeque<NetworkMessage>,
    max_size: usize,
}

impl MessageQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            messages: VecDeque::new(),
            max_size,
        }
    }

    pub fn enqueue(&mut self, msg: NetworkMessage) -> bool {
        if self.messages.len() >= self.max_size {
            return false;
        }
        self.messages.push_back(msg);
        true
    }

    pub fn dequeue(&mut self) -> Option<NetworkMessage> {
        self.messages.pop_front()
    }

    pub fn size(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

pub struct MessageSerializer;

impl MessageSerializer {
    pub fn serialize(msg: &NetworkMessage) -> Result<Vec<u8>, String> {
        serde_json::to_vec(msg).map_err(|e| format!("Serialization failed: {}", e))
    }

    pub fn deserialize(data: &[u8]) -> Result<NetworkMessage, String> {
        serde_json::from_slice(data).map_err(|e| format!("Deserialization failed: {}", e))
    }
}
