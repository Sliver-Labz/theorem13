use crate::quorum::QuorumSlice;
use crate::intactness::IntactnessProof;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SMRState {
    Normal,
    ReconfigProposed,
    ReconfigCommitted,
    Reconfiguring,
    PostReconfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReconfigMessage {
    pub node_id: String,
    pub slot_seq: u64,
    pub new_slices: HashMap<String, QuorumSlice>,
    pub proof: IntactnessProof,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckpointBallot {
    pub ballot_height: u64,
    pub applied_slices: HashMap<String, QuorumSlice>,
}

pub struct ProtocolState {
    pub state: SMRState,
    pub slot_seq: u64,
    pub current_slices: HashMap<String, QuorumSlice>,
    pub pending_reconfig: Option<ReconfigMessage>,
    pub votes: HashMap<String, bool>,
    pub checkpoint: Option<CheckpointBallot>,
}

impl ProtocolState {
    pub fn new(slot_seq: u64, initial_slices: HashMap<String, QuorumSlice>) -> Self {
        Self {
            state: SMRState::Normal,
            slot_seq,
            current_slices: initial_slices,
            pending_reconfig: None,
            votes: HashMap::new(),
            checkpoint: None,
        }
    }

    pub fn propose_reconfig(&mut self, msg: ReconfigMessage) -> bool {
        if self.state != SMRState::Normal {
            return false;
        }
        self.pending_reconfig = Some(msg);
        self.state = SMRState::ReconfigProposed;
        self.votes.clear();
        true
    }

    pub fn vote_on_reconfig(&mut self, voter_id: String, approve: bool) -> bool {
        if self.state != SMRState::ReconfigProposed && self.state != SMRState::ReconfigCommitted {
            return false;
        }
        self.votes.insert(voter_id, approve);
        true
    }

    pub fn commit_reconfig(&mut self, total_nodes: usize) -> bool {
        if self.state != SMRState::ReconfigProposed {
            return false;
        }
        let approvals = self.votes.values().filter(|&&v| v).count();
        if approvals > total_nodes / 2 {
            self.state = SMRState::ReconfigCommitted;
            return true;
        }
        false
    }

    pub fn apply_reconfig(&mut self, checkpoint: CheckpointBallot) -> bool {
        if self.state != SMRState::ReconfigCommitted {
            return false;
        }
        self.current_slices = checkpoint.applied_slices.clone();
        self.checkpoint = Some(checkpoint);
        self.state = SMRState::PostReconfig;
        self.pending_reconfig = None;
        true
    }

    pub fn reset_to_normal(&mut self) {
        self.state = SMRState::Normal;
        self.votes.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_protocol_state_transitions() {
        let initial_slices = HashMap::new();
        let mut state = ProtocolState::new(1, initial_slices);

        assert_eq!(state.state, SMRState::Normal);

        let new_slices = HashMap::new();
        let intact: HashSet<String> = vec!["A".to_string()].into_iter().collect();
        let faulty = HashSet::new();
        let proof = IntactnessProof::new(intact, faulty);

        let msg = ReconfigMessage {
            node_id: "A".to_string(),
            slot_seq: 1,
            new_slices,
            proof,
            timestamp: 0,
        };

        assert!(state.propose_reconfig(msg));
        assert_eq!(state.state, SMRState::ReconfigProposed);
    }

    #[test]
    fn test_voting_and_commit() {
        let initial_slices = HashMap::new();
        let mut state = ProtocolState::new(1, initial_slices);

        let new_slices = HashMap::new();
        let intact: HashSet<String> = vec!["A".to_string()].into_iter().collect();
        let faulty = HashSet::new();
        let proof = IntactnessProof::new(intact, faulty);

        let msg = ReconfigMessage {
            node_id: "A".to_string(),
            slot_seq: 1,
            new_slices,
            proof,
            timestamp: 0,
        };

        state.propose_reconfig(msg);
        state.vote_on_reconfig("A".to_string(), true);
        state.vote_on_reconfig("B".to_string(), true);

        assert!(state.commit_reconfig(3));
        assert_eq!(state.state, SMRState::ReconfigCommitted);
    }
}
