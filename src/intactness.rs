use crate::quorum::QuorumSlice;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntactnessProof {
    pub intact_set: HashSet<String>,
    pub faulty_set: HashSet<String>,
    pub proof_hash: String,
}

impl IntactnessProof {
    pub fn new(intact_set: HashSet<String>, faulty_set: HashSet<String>) -> Self {
        let proof_hash = Self::compute_hash(&intact_set, &faulty_set);
        Self {
            intact_set,
            faulty_set,
            proof_hash,
        }
    }

    fn compute_hash(intact: &HashSet<String>, faulty: &HashSet<String>) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        let mut intact_vec: Vec<_> = intact.iter().cloned().collect();
        intact_vec.sort();
        for node in intact_vec {
            hasher.update(node.as_bytes());
        }
        let mut faulty_vec: Vec<_> = faulty.iter().cloned().collect();
        faulty_vec.sort();
        for node in faulty_vec {
            hasher.update(node.as_bytes());
        }
        hex::encode(hasher.finalize())
    }

    pub fn is_disjoint(&self) -> bool {
        self.intact_set.is_disjoint(&self.faulty_set)
    }
}

pub struct IntactnessValidator;

impl IntactnessValidator {
    pub fn validate_slice_change(
        old_slice: &QuorumSlice,
        new_slice: &QuorumSlice,
        proof: &IntactnessProof,
    ) -> bool {
        if !proof.is_disjoint() {
            return false;
        }

        let removed: HashSet<String> = old_slice
            .validators
            .difference(&new_slice.validators)
            .cloned()
            .collect();

        let added: HashSet<String> = new_slice
            .validators
            .difference(&old_slice.validators)
            .cloned()
            .collect();

        for added_node in &added {
            if proof.faulty_set.contains(added_node) {
                return false;
            }
        }

        true
    }

    pub fn validate_cumulative(
        proofs: &[IntactnessProof],
        all_slices: &HashMap<String, QuorumSlice>,
    ) -> bool {
        let combined_faulty: HashSet<String> = proofs
            .iter()
            .flat_map(|p| p.faulty_set.clone())
            .collect();

        for (_, slice) in all_slices {
            if !combined_faulty.is_disjoint(&slice.validators) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intactness_proof_disjoint() {
        let intact: HashSet<String> = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect();
        let faulty: HashSet<String> = vec!["C".to_string(), "D".to_string()]
            .into_iter()
            .collect();

        let proof = IntactnessProof::new(intact, faulty);
        assert!(proof.is_disjoint());
    }

    #[test]
    fn test_intactness_proof_overlapping() {
        let intact: HashSet<String> = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect();
        let faulty: HashSet<String> = vec!["B".to_string(), "C".to_string()]
            .into_iter()
            .collect();

        let proof = IntactnessProof::new(intact, faulty);
        assert!(!proof.is_disjoint());
    }

    #[test]
    fn test_validate_slice_change_adds_faulty() {
        let old_validators = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect();
        let old_slice = QuorumSlice::new(1, old_validators);

        let new_validators = vec!["A".to_string(), "B".to_string(), "D".to_string()]
            .into_iter()
            .collect();
        let new_slice = QuorumSlice::new(2, new_validators);

        let intact: HashSet<String> = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect();
        let faulty: HashSet<String> = vec!["D".to_string()].into_iter().collect();
        let proof = IntactnessProof::new(intact, faulty);

        assert!(!IntactnessValidator::validate_slice_change(
            &old_slice,
            &new_slice,
            &proof
        ));
    }
}
