// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::network::{send_message, LocalMessage, NetworkClient};
use crate::node_manager::NodesManagerClient;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::routing_key;
use libproto::{Message, TryFrom};
use pubsub::channel::{unbounded, Receiver, Sender};
use pubsub::start_pubsub;
use std::thread;

/// MqAgent
pub struct MqAgent {
    client: MqAgentClient,
    nodes_manager_client: Option<NodesManagerClient>,
    network_client: Option<NetworkClient>,

    sub_auth: Receiver<(String, Vec<u8>)>,
    sub_consensus: Receiver<(String, Vec<u8>)>,
    sub_other_modules: Receiver<(String, Vec<u8>)>,
}

impl MqAgent {
    pub fn new() -> Self {
        // New transactions use a special channel, all new transactions come from:
        // JSON-RPC -> Auth -> Network,
        // So the channel subscribe 'Auth' Request from MQ
        let (ctx_sub_auth, crx_sub_auth) = unbounded();
        let (ctx_pub_auth, crx_pub_auth) = unbounded();
        start_pubsub(
            "network_auth",
            routing_key!([Auth >> Request, Auth >> GetBlockTxn, Auth >> BlockTxn]),
            ctx_sub_auth,
            crx_pub_auth,
        );

        // Consensus use a special channel
        let (ctx_sub_consensus, crx_sub_consensus) = unbounded();
        let (ctx_pub_consensus, crx_pub_consensus) = unbounded();
        start_pubsub(
            "network_consensus",
            routing_key!([Consensus >> CompactSignedProposal, Consensus >> RawBytes]),
            ctx_sub_consensus,
            crx_pub_consensus,
        );

        // Chain, JSON-RPC and Snapshot use a common channel
        let (ctx_sub_other_modules, crx_sub_other_modules) = unbounded();
        let (ctx_pub_other_modules, crx_pub_other_modules) = unbounded();
        start_pubsub(
            "network",
            routing_key!([
                Chain >> Status,
                Chain >> RichStatus,
                Chain >> SyncResponse,
                Jsonrpc >> RequestNet,
                Jsonrpc >> RequestPeersInfo,
                Snapshot >> SnapshotReq,
                Executor >> GetCrlResp,
            ]),
            ctx_sub_other_modules,
            crx_pub_other_modules,
        );
        let client = MqAgentClient::new(ctx_pub_auth, ctx_pub_consensus, ctx_pub_other_modules);

        MqAgent {
            client,
            nodes_manager_client: None,
            network_client: None,
            sub_auth: crx_sub_auth,
            sub_consensus: crx_sub_consensus,
            sub_other_modules: crx_sub_other_modules,
        }
    }

    pub fn set_nodes_mgr_client(&mut self, client: NodesManagerClient) {
        self.nodes_manager_client = Some(client);
    }

    pub fn set_network_client(&mut self, client: NetworkClient) {
        self.network_client = Some(client);
    }

    pub fn client(&self) -> MqAgentClient {
        self.client.clone()
    }

    pub fn run(&self) {
        if let Some(ref client) = self.nodes_manager_client {
            // Thread for handle new transactions from MQ
            let nodes_mgr_client = client.clone();
            let sub_auth = self.sub_auth.clone();
            thread::spawn(move || loop {
                let (key, body) = sub_auth.recv().unwrap();
                let msg = Message::try_from(&body).unwrap();

                send_message(&nodes_mgr_client, key, msg);
            });

            // Thread for handle consensus message
            let nodes_mgr_client = client.clone();
            let sub_consensus = self.sub_consensus.clone();
            thread::spawn(move || loop {
                let (key, body) = sub_consensus.recv().unwrap();
                let msg = Message::try_from(&body).unwrap();

                send_message(&nodes_mgr_client, key, msg);
            });
        }

        if let Some(ref client) = self.network_client {
            let network_client = client.clone();
            let sub_other_modules = self.sub_other_modules.clone();
            thread::spawn(move || loop {
                let (key, body) = sub_other_modules.recv().unwrap();
                trace!("[MqAgent] Handle delivery from {} payload {:?}", key, body);

                let msg = LocalMessage::new(key, body);
                network_client.handle_local_message(msg);
            });
        }
    }
}

impl Default for MqAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct MqAgentClient {
    pub_auth: Sender<(String, Vec<u8>)>,
    pub_consensus: Sender<(String, Vec<u8>)>,
    pub_other_modules: Sender<(String, Vec<u8>)>,
}

impl MqAgentClient {
    pub fn new(
        pub_auth: Sender<(String, Vec<u8>)>,
        pub_consensus: Sender<(String, Vec<u8>)>,
        pub_other_modules: Sender<(String, Vec<u8>)>,
    ) -> Self {
        MqAgentClient {
            pub_auth,
            pub_consensus,
            pub_other_modules,
        }
    }

    pub fn forward_msg_to_auth(&self, msg: PubMessage) {
        if let Err(e) = self.pub_auth.send((msg.key, msg.data)) {
            warn!("[MqAgent] Forward message to auth failed: {:?}", e);
        }
    }

    pub fn forward_msg_to_consensus(&self, msg: PubMessage) {
        if let Err(e) = self.pub_consensus.send((msg.key, msg.data)) {
            warn!("[MqAgent] Forward message to consensus failed: {:?}", e);
        }
    }

    pub fn send_peer_count(&self, msg: PubMessage) {
        if let Err(e) = self.pub_other_modules.send((msg.key, msg.data)) {
            warn!("[MqAgent] Send peer count failed: {:?}", e);
        }
    }

    pub fn send_snapshot_resp(&self, msg: PubMessage) {
        if let Err(e) = self.pub_other_modules.send((msg.key, msg.data)) {
            warn!("[MqAgent] Send snapshot response failed: {:?}", e);
        }
    }

    // Publish a synchronize request, to start synchronize operation in this node
    pub fn pub_sync_request(&self, msg: PubMessage) {
        if let Err(e) = self.pub_other_modules.send((msg.key, msg.data)) {
            warn!("[MqAgent] Publish synchronize request failed: {:?}", e);
        }
    }

    pub fn pub_sync_blocks(&self, msg: PubMessage) {
        if let Err(e) = self.pub_other_modules.send((msg.key, msg.data)) {
            warn!("[MqAgent] Publish synchronize blocks failed: {:?}", e);
        }
    }

    pub fn get_crl(&self, msg: PubMessage) {
        if let Err(e) = self.pub_other_modules.send((msg.key, msg.data)) {
            warn!("[MqAgent] Get certificat revoke list failed: {:?}", e);
        }
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
