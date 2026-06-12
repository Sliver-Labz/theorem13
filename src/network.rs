use serde::{Deserialize, Serialize};
use crate::protocol::ReconfigMessage;

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
