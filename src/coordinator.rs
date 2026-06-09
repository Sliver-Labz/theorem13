use crate::protocol::{ProtocolState, ReconfigMessage, CheckpointBallot, SMRState};
use crate::intactness::IntactnessValidator;
use crate::quorum::QuorumSlice;
use crate::failure_detection::FailureDetector;
use std::collections::{HashMap, HashSet};

pub struct ReconfigCoordinator {
    node_id: String,
    protocol_state: ProtocolState,
    failure_detector: FailureDetector,
    all_nodes: HashSet<String>,
}

impl ReconfigCoordinator {
    pub fn new(
        node_id: String,
        initial_slices: HashMap<String, QuorumSlice>,
        all_nodes: HashSet<String>,
    ) -> Self {
        let protocol_state = ProtocolState::new(1, initial_slices);
        let failure_detector = FailureDetector::new(5000);

        Self {
            node_id,
            protocol_state,
            failure_detector,
            all_nodes,
        }
    }

    pub fn heartbeat(&mut self, from_node: &str) {
        self.failure_detector.heartbeat(from_node);
    }

    pub fn check_for_failures(&mut self) -> Vec<String> {
        self.failure_detector.check_timeouts()
    }

    pub fn propose_reconfiguration(
        &mut self,
        new_slices: HashMap<String, QuorumSlice>,
        proof_intact: HashSet<String>,
        proof_faulty: HashSet<String>,
    ) -> bool {
        let proof = crate::intactness::IntactnessProof::new(proof_intact, proof_faulty);

        let msg = ReconfigMessage {
            node_id: self.node_id.clone(),
            slot_seq: self.protocol_state.slot_seq,
            new_slices,
            proof,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        };

        self.protocol_state.propose_reconfig(msg)
    }

    pub fn vote_on_reconfig(&mut self, approve: bool) -> bool {
        self.protocol_state
            .vote_on_reconfig(self.node_id.clone(), approve)
    }

    pub fn commit_reconfig(&mut self) -> bool {
        self.protocol_state.commit_reconfig(self.all_nodes.len())
    }

    pub fn apply_checkpoint(&mut self, new_slices: HashMap<String, QuorumSlice>) -> bool {
        let checkpoint = CheckpointBallot {
            ballot_height: 1,
            applied_slices: new_slices,
        };
        self.protocol_state.apply_reconfig(checkpoint)
    }

    pub fn get_current_state(&self) -> &ProtocolState {
        &self.protocol_state
    }

    pub fn get_state_enum(&self) -> SMRState {
        self.protocol_state.state.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinator_initialization() {
        let mut slices = HashMap::new();
        let validators = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect();
        slices.insert("A".to_string(), QuorumSlice::new(1, validators));

        let all_nodes = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect();
        let coordinator = ReconfigCoordinator::new("A".to_string(), slices, all_nodes);

        assert_eq!(coordinator.node_id, "A");
        assert_eq!(coordinator.get_state_enum(), SMRState::Normal);
    }

    #[test]
    fn test_heartbeat_and_failure_detection() {
        let mut slices = HashMap::new();
        let validators = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect();
        slices.insert("A".to_string(), QuorumSlice::new(1, validators));

        let all_nodes = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect();
        let mut coordinator = ReconfigCoordinator::new("A".to_string(), slices, all_nodes);

        coordinator.heartbeat("B");
        let failures = coordinator.check_for_failures();
        assert_eq!(failures.len(), 0);
    }

    #[test]
    fn test_propose_and_vote() {
        let mut slices = HashMap::new();
        let validators = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect();
        slices.insert("A".to_string(), QuorumSlice::new(1, validators));

        let all_nodes = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect();
        let mut coordinator = ReconfigCoordinator::new("A".to_string(), slices, all_nodes);

        let new_slices = HashMap::new();
        let intact: HashSet<String> = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect();
        let faulty = HashSet::new();

        assert!(coordinator.propose_reconfiguration(new_slices, intact, faulty));
        assert_eq!(coordinator.get_state_enum(), SMRState::ReconfigProposed);

        assert!(coordinator.vote_on_reconfig(true));
    }
}
