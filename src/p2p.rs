use crate::network::NetworkMessage;
use std::collections::HashMap;

pub struct P2PNode {
    node_id: String,
    peers: HashMap<String, PeerConnection>,
    local_seq: u64,
}

#[derive(Clone)]
pub struct PeerConnection {
    peer_id: String,
    last_heartbeat: u64,
    is_connected: bool,
}

impl P2PNode {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            peers: HashMap::new(),
            local_seq: 0,
        }
    }

    pub fn add_peer(&mut self, peer_id: String) {
        self.peers.insert(
            peer_id.clone(),
            PeerConnection {
                peer_id,
                last_heartbeat: 0,
                is_connected: true,
            },
        );
    }

    pub fn remove_peer(&mut self, peer_id: &str) -> bool {
        self.peers.remove(peer_id).is_some()
    }

    pub fn get_peer(&self, peer_id: &str) -> Option<&PeerConnection> {
        self.peers.get(peer_id)
    }

    pub fn broadcast(&mut self, msg: NetworkMessage) -> Vec<(String, NetworkMessage)> {
        self.local_seq += 1;
        let mut broadcast_msgs = Vec::new();
        for (peer_id, conn) in self.peers.iter() {
            if conn.is_connected {
                broadcast_msgs.push((peer_id.clone(), msg.clone()));
            }
        }
        broadcast_msgs
    }

    pub fn unicast(&mut self, peer_id: &str, mut msg: NetworkMessage) -> Option<NetworkMessage> {
        if self.peers.get(peer_id)?.is_connected {
            self.local_seq += 1;
            Some(msg)
        } else {
            None
        }
    }

    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    pub fn connected_peers(&self) -> usize {
        self.peers.values().filter(|p| p.is_connected).count()
    }
}
