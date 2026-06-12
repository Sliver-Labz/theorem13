use crate::protocol::ReconfigMessage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SorobanContractCall {
    pub contract_id: String,
    pub method: String,
    pub args: Vec<Vec<u8>>,
    pub result: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsensusStateSnapshot {
    pub slot_seq: u64,
    pub reconfig_msg_hash: String,
    pub timestamp: u64,
    pub validated_by_count: usize,
}

pub struct SorobanBridge {
    contract_id: String,
    call_history: Vec<SorobanContractCall>,
}

impl SorobanBridge {
    pub fn new(contract_id: String) -> Self {
        Self {
            contract_id,
            call_history: Vec::new(),
        }
    }

    pub fn validate_reconfig_onchain(
        &mut self,
        reconfig: &ReconfigMessage,
        state_snapshot: &ConsensusStateSnapshot,
    ) -> Result<bool, String> {
        let call = SorobanContractCall {
            contract_id: self.contract_id.clone(),
            method: "validate_reconfig".to_string(),
            args: vec![
                reconfig.node_id.as_bytes().to_vec(),
                state_snapshot.reconfig_msg_hash.as_bytes().to_vec(),
            ],
            result: None,
        };

        self.call_history.push(call);
        Ok(true)
    }

    pub fn record_consensus_state(
        &mut self,
        state: &ConsensusStateSnapshot,
    ) -> Result<(), String> {
        let call = SorobanContractCall {
            contract_id: self.contract_id.clone(),
            method: "record_state".to_string(),
            args: vec![state.slot_seq.to_le_bytes().to_vec()],
            result: None,
        };

        self.call_history.push(call);
        Ok(())
    }

    pub fn get_call_history(&self) -> &[SorobanContractCall] {
        &self.call_history
    }

    pub fn clear_history(&mut self) {
        self.call_history.clear();
    }
}

pub struct SorobanReconfigValidator {
    bridge: SorobanBridge,
}

impl SorobanReconfigValidator {
    pub fn new(contract_id: String) -> Self {
        Self {
            bridge: SorobanBridge::new(contract_id),
        }
    }

    pub fn validate_with_contract(
        &mut self,
        reconfig: &ReconfigMessage,
        slot_seq: u64,
    ) -> Result<bool, String> {
        let hash = crate::intactness::IntactnessProof::compute_hash(
            &reconfig.proof.intact_set,
            &reconfig.proof.faulty_set,
        );

        let snapshot = ConsensusStateSnapshot {
            slot_seq,
            reconfig_msg_hash: hash,
            timestamp: reconfig.timestamp,
            validated_by_count: 0,
        };

        self.bridge.validate_reconfig_onchain(reconfig, &snapshot)?;
        self.bridge.record_consensus_state(&snapshot)?;

        Ok(true)
    }

    pub fn get_history(&self) -> &[SorobanContractCall] {
        self.bridge.get_call_history()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_soroban_bridge_creation() {
        let bridge = SorobanBridge::new("CBRIDGEID123".to_string());
        assert_eq!(bridge.contract_id, "CBRIDGEID123");
    }

    #[test]
    fn test_validate_reconfig_onchain() {
        let mut bridge = SorobanBridge::new("CBRIDGEID123".to_string());

        let intact: HashSet<String> = vec!["A".to_string()].into_iter().collect();
        let faulty = HashSet::new();
        let proof = crate::IntactnessProof::new(intact, faulty);

        let reconfig = ReconfigMessage {
            node_id: "A".to_string(),
            slot_seq: 1,
            new_slices: HashMap::new(),
            proof,
            timestamp: 100,
        };

        let snapshot = ConsensusStateSnapshot {
            slot_seq: 1,
            reconfig_msg_hash: "hash123".to_string(),
            timestamp: 100,
            validated_by_count: 1,
        };

        assert!(bridge.validate_reconfig_onchain(&reconfig, &snapshot).is_ok());
        assert_eq!(bridge.get_call_history().len(), 1);
    }

    #[test]
    fn test_soroban_validator() {
        let mut validator = SorobanReconfigValidator::new("CONTRACT123".to_string());

        let intact: HashSet<String> = vec!["A".to_string()].into_iter().collect();
        let faulty = HashSet::new();
        let proof = crate::IntactnessProof::new(intact, faulty);

        let reconfig = ReconfigMessage {
            node_id: "A".to_string(),
            slot_seq: 1,
            new_slices: HashMap::new(),
            proof,
            timestamp: 100,
        };

        assert!(validator.validate_with_contract(&reconfig, 1).is_ok());
        assert_eq!(validator.get_history().len(), 2);
    }
}
