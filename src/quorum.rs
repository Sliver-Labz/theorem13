use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub is_well_behaved: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuorumSlice {
    pub threshold: usize,
    pub validators: HashSet<String>,
}

impl QuorumSlice {
    pub fn new(threshold: usize, validators: HashSet<String>) -> Self {
        Self {
            threshold,
            validators,
        }
    }

    pub fn is_quorum(&self, nodes: &HashSet<String>) -> bool {
        let intersection: HashSet<_> = self.validators.intersection(nodes).cloned().collect();
        intersection.len() >= self.threshold
    }

    pub fn remove_node(&mut self, node_id: &str) {
        self.validators.remove(node_id);
    }

    pub fn add_node(&mut self, node_id: String) {
        self.validators.insert(node_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quorum_slice_formation() {
        let validators = vec!["A".to_string(), "B".to_string(), "C".to_string()]
            .into_iter()
            .collect();
        let slice = QuorumSlice::new(2, validators);
        assert_eq!(slice.validators.len(), 3);
        assert_eq!(slice.threshold, 2);
    }

    #[test]
    fn test_is_quorum() {
        let validators = vec!["A".to_string(), "B".to_string(), "C".to_string()]
            .into_iter()
            .collect();
        let slice = QuorumSlice::new(2, validators);

        let present: HashSet<_> = vec!["A".to_string(), "B".to_string()].into_iter().collect();
        assert!(slice.is_quorum(&present));

        let insufficient: HashSet<_> = vec!["A".to_string()].into_iter().collect();
        assert!(!slice.is_quorum(&insufficient));
    }

    #[test]
    fn test_remove_node() {
        let validators = vec!["A".to_string(), "B".to_string(), "C".to_string()]
            .into_iter()
            .collect();
        let mut slice = QuorumSlice::new(2, validators);
        slice.remove_node("C");
        assert!(!slice.validators.contains("C"));
    }
}
