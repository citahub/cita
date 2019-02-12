use std::sync::mpsc::Sender;

#[derive(Clone)]
pub struct MqClient {
    auth_sender: Sender<(String, Vec<u8>)>,
    consensus_sender: Sender<(String, Vec<u8>)>,
    mq_sender: Sender<(String, Vec<u8>)>,
}

impl MqClient {
    pub fn new(
        auth_sender: Sender<(String, Vec<u8>)>,
        consensus_sender: Sender<(String, Vec<u8>)>,
        mq_sender: Sender<(String, Vec<u8>)>,
    ) -> Self {
        MqClient {
            auth_sender,
            consensus_sender,
            mq_sender,
        }
    }

    pub fn forward_msg_to_auth(&self, msg: PubMessage) {
        let _ = self.auth_sender.send((msg.key, msg.data));
    }

    pub fn forward_msg_to_consensus(&self, msg: PubMessage) {
        let _ = self.consensus_sender.send((msg.key, msg.data));
    }

    pub fn send_peer_count(&self, msg: PubMessage) {
        let _ = self.mq_sender.send((msg.key, msg.data));
    }

    pub fn send_snapshot_resp(&self, msg: PubMessage) {
        let _ = self.mq_sender.send((msg.key, msg.data));
    }

    // Publish a synchronize request, to start synchronize operation in this node
    pub fn pub_sync_request(&self, msg: PubMessage) {
        let _ = self.mq_sender.send((msg.key, msg.data));
    }

    pub fn pub_sync_blocks(&self, msg: PubMessage) {
        let _ = self.mq_sender.send((msg.key, msg.data));
    }
}

pub struct PubMessage {
    key: String,
    data: Vec<u8>,
}

impl PubMessage {
    pub fn new(key: String, data: Vec<u8>) -> Self {
        PubMessage { key, data }
    }
}
