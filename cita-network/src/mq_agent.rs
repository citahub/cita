// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::network::{LocalMessage, NetworkClient};
use crate::node_manager::{BroadcastReq, NodesManagerClient};
use crossbeam_channel::{unbounded, Receiver, Sender};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::routing_key;
use libproto::{Message, TryFrom};
use logger::trace;
use pubsub::start_pubsub;
use std::thread;

pub struct MqAgent {
    client: MqAgentClient,
    nodes_manager_client: Option<NodesManagerClient>,
    network_client: Option<NetworkClient>,

    crx_sub_auth: Receiver<(String, Vec<u8>)>,
    crx_sub_consensus: Receiver<(String, Vec<u8>)>,
    crx_sub: Receiver<(String, Vec<u8>)>,
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
        let (ctx_sub, crx_sub) = unbounded();
        let (ctx_pub, crx_pub) = unbounded();
        start_pubsub(
            "network",
            routing_key!([
                Chain >> Status,
                Chain >> SyncResponse,
                Jsonrpc >> RequestNet,
                Snapshot >> SnapshotReq
            ]),
            ctx_sub,
            crx_pub,
        );
        let client = MqAgentClient::new(ctx_pub_auth, ctx_pub_consensus, ctx_pub);

        MqAgent {
            client,
            nodes_manager_client: None,
            network_client: None,
            crx_sub_auth,
            crx_sub_consensus,
            crx_sub,
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
            let nodes_manager_client = client.clone();
            let crx_sub_auth = self.crx_sub_auth.clone();
            thread::spawn(move || loop {
                let (key, body) = crx_sub_auth.recv().unwrap();
                let msg = Message::try_from(&body).unwrap();

                // Broadcast the message to other nodes
                nodes_manager_client.broadcast(BroadcastReq::new(key, msg));
            });

            // Thread for handle consensus message
            let nodes_manager_client = client.clone();
            let crx_sub_consensus = self.crx_sub_consensus.clone();
            thread::spawn(move || loop {
                let (key, body) = crx_sub_consensus.recv().unwrap();
                let msg = Message::try_from(&body).unwrap();

                // Broadcast the message to other nodes
                nodes_manager_client.broadcast(BroadcastReq::new(key, msg));
            });
        }

        if let Some(ref client) = self.network_client {
            let network_client = client.clone();
            let crx_sub = self.crx_sub.clone();
            thread::spawn(move || loop {
                let (key, body) = crx_sub.recv().unwrap();
                trace!("[main] Handle delivery from {} payload {:?}", key, body);

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
    auth_sender: Sender<(String, Vec<u8>)>,
    consensus_sender: Sender<(String, Vec<u8>)>,
    other_modules_sender: Sender<(String, Vec<u8>)>,
}

impl MqAgentClient {
    pub fn new(
        auth_sender: Sender<(String, Vec<u8>)>,
        consensus_sender: Sender<(String, Vec<u8>)>,
        other_modules_sender: Sender<(String, Vec<u8>)>,
    ) -> Self {
        MqAgentClient {
            auth_sender,
            consensus_sender,
            other_modules_sender,
        }
    }

    pub fn forward_msg_to_auth(&self, msg: PubMessage) {
        let _ = self.auth_sender.send((msg.key, msg.data));
    }

    pub fn forward_msg_to_consensus(&self, msg: PubMessage) {
        let _ = self.consensus_sender.send((msg.key, msg.data));
    }

    pub fn send_peer_count(&self, msg: PubMessage) {
        let _ = self.other_modules_sender.send((msg.key, msg.data));
    }

    pub fn send_snapshot_resp(&self, msg: PubMessage) {
        let _ = self.other_modules_sender.send((msg.key, msg.data));
    }

    // Publish a synchronize request, to start synchronize operation in this node
    pub fn pub_sync_request(&self, msg: PubMessage) {
        let _ = self.other_modules_sender.send((msg.key, msg.data));
    }

    pub fn pub_sync_blocks(&self, msg: PubMessage) {
        let _ = self.other_modules_sender.send((msg.key, msg.data));
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
