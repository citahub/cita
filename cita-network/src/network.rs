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

use crate::mq_agent::{MqAgentClient, PubMessage};
use crate::node_manager::{BroadcastReq, GetPeerCountReq, NodesManagerClient, SingleTxReq};
use crate::synchronizer::{SynchronizerClient, SynchronizerMessage};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::routing_key;
use libproto::snapshot::{Cmd, Resp, SnapshotResp};
use libproto::{Message as ProtoMessage, OperateType, Response};
use libproto::{TryFrom, TryInto};
use logger::{error, info, trace, warn};
use pubsub::channel::{unbounded, Receiver, Sender};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Network {
    is_pause: Arc<AtomicBool>,
    mq_client: MqAgentClient,
    network_client: NetworkClient,
    nodes_mgr_client: NodesManagerClient,
    sync_client: SynchronizerClient,
    msg_receiver: Receiver<NetworkMessage>,
}

impl Network {
    pub fn new(
        mq_client: MqAgentClient,
        nodes_mgr_client: NodesManagerClient,
        sync_client: SynchronizerClient,
    ) -> Self {
        let (tx, rx) = unbounded();
        let client = NetworkClient { sender: tx };
        Network {
            is_pause: Arc::new(AtomicBool::new(false)),
            mq_client,
            network_client: client,
            nodes_mgr_client,
            sync_client,
            msg_receiver: rx,
        }
    }

    pub fn client(&self) -> NetworkClient {
        self.network_client.clone()
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(msg) = self.msg_receiver.recv() {
                msg.handle(self);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct NetworkClient {
    sender: Sender<NetworkMessage>,
}

impl NetworkClient {
    pub fn new(sender: Sender<NetworkMessage>) -> Self {
        NetworkClient { sender }
    }

    pub fn handle_local_message(&self, msg: LocalMessage) {
        self.send_message(NetworkMessage::LocalMessage(msg));
    }

    pub fn handle_remote_message(&self, msg: RemoteMessage) {
        self.send_message(NetworkMessage::RemoteMessage(msg));
    }

    fn send_message(&self, msg: NetworkMessage) {
        self.sender.try_send(msg).unwrap_or_else(|err| {
            warn!("[Network] Send message to network failed : {:?}", err);
        });
    }
}

pub enum NetworkMessage {
    LocalMessage(LocalMessage),
    RemoteMessage(RemoteMessage),
}

impl NetworkMessage {
    pub fn handle(self, service: &mut Network) {
        match self {
            NetworkMessage::LocalMessage(msg) => msg.handle(service),
            NetworkMessage::RemoteMessage(msg) => msg.handle(service),
        }
    }
}
pub struct LocalMessage {
    key: String,
    data: Vec<u8>,
}

impl LocalMessage {
    pub fn new(key: String, data: Vec<u8>) -> Self {
        LocalMessage { key, data }
    }

    pub fn handle(self, service: &mut Network) {
        let rt_key = RoutingKey::from(&self.key);
        trace!("[Network] Receive Message from Local/{}", self.key);

        if service.is_pause.load(Ordering::SeqCst)
            && rt_key.get_sub_module() != SubModules::Snapshot
        {
            return;
        }

        match rt_key {
            routing_key!(Chain >> Status) => {
                service
                    .sync_client
                    .handle_local_status(SynchronizerMessage::new(self.key, self.data));
            }
            routing_key!(Chain >> SyncResponse) => {
                let msg = ProtoMessage::try_from(&self.data).unwrap();
                send_message(
                    &service.nodes_mgr_client,
                    routing_key!(Synchronizer >> SyncResponse).into(),
                    msg,
                );
            }
            routing_key!(Jsonrpc >> RequestNet) => {
                self.reply_rpc(&self.data, service);
            }
            routing_key!(Snapshot >> SnapshotReq) => {
                info!("[Network] Set disconnect and response");
                self.snapshot_req(&self.data, service);
            }
            _ => {
                error!("[Network] Unexpected key {} from Local", self.key);
            }
        }
    }

    fn reply_rpc(&self, data: &[u8], service: &mut Network) {
        let mut msg = ProtoMessage::try_from(data).unwrap();

        let req_opt = msg.take_request();
        {
            if let Some(mut req) = req_opt {
                // Get peer count and send back to JsonRpc from MQ
                if req.has_peercount() {
                    let mut response = Response::new();
                    response.set_request_id(req.take_request_id());

                    let (tx, rx) = unbounded();
                    service
                        .nodes_mgr_client
                        .get_peer_count(GetPeerCountReq::new(tx));

                    // Get peer count from rx channel
                    // FIXME: This is a block receive, double check about this
                    let peer_count = rx.recv().unwrap();
                    response.set_peercount(peer_count as u32);
                    let msg: ProtoMessage = response.into();
                    service.mq_client.send_peer_count(PubMessage::new(
                        routing_key!(Net >> Response).into(),
                        msg.try_into().unwrap(),
                    ));
                }
            } else {
                warn!("[Network] Receive unexpected rpc data");
            }
        }
    }

    fn snapshot_req(&self, data: &[u8], service: &mut Network) {
        let mut msg = ProtoMessage::try_from(data).unwrap();
        let req = msg.take_snapshot_req().unwrap();
        let mut resp = SnapshotResp::new();

        match req.cmd {
            Cmd::Snapshot => {
                info!("[Network] Snapshot receive cmd::Snapshot: {:?}", req);
                resp.set_resp(Resp::SnapshotAck);
                resp.set_flag(true);
            }
            Cmd::Begin => {
                info!("[Network] Snapshot receive cmd::Begin: {:?}", req);
                service.is_pause.store(true, Ordering::SeqCst);
                resp.set_resp(Resp::BeginAck);
                resp.set_flag(true);
            }
            Cmd::Restore => {
                info!("[Network] Snapshot receive cmd::Restore: {:?}", req);
                resp.set_resp(Resp::RestoreAck);
                resp.set_flag(true);
            }
            Cmd::Clear => {
                info!("[Network] Snapshot receive cmd::Clear: {:?}", req);
                resp.set_resp(Resp::ClearAck);
                resp.set_flag(true);
            }
            Cmd::End => {
                info!("[Network] Snapshot receive cmd::End: {:?}", req);
                service.is_pause.store(false, Ordering::SeqCst);
                resp.set_resp(Resp::EndAck);
                resp.set_flag(true);
            }
        }

        let msg: ProtoMessage = resp.into();
        service.mq_client.send_snapshot_resp(PubMessage::new(
            routing_key!(Net >> SnapshotResp).into(),
            (&msg).try_into().unwrap(),
        ));
    }
}

pub struct RemoteMessage {
    key: String,
    data: Vec<u8>,
}

impl RemoteMessage {
    pub fn new(key: String, data: Vec<u8>) -> Self {
        RemoteMessage { key, data }
    }

    pub fn handle(self, service: &mut Network) {
        let rt_key = RoutingKey::from(&self.key);
        trace!("[Network] Receive Message from Remote/{}", self.key);

        if service.is_pause.load(Ordering::SeqCst)
            && rt_key.get_sub_module() != SubModules::Snapshot
        {
            return;
        }

        match rt_key {
            routing_key!(Synchronizer >> Status) => {
                service
                    .sync_client
                    .handle_remote_status(SynchronizerMessage::new(self.key, self.data));
            }
            routing_key!(Synchronizer >> SyncResponse) => {
                service
                    .sync_client
                    .handle_remote_response(SynchronizerMessage::new(self.key, self.data));
            }
            routing_key!(Synchronizer >> SyncRequest) => {
                service.mq_client.pub_sync_request(PubMessage::new(
                    routing_key!(Net >> SyncRequest).into(),
                    self.data,
                ));
            }
            routing_key!(Consensus >> CompactSignedProposal) => {
                let msg =
                    PubMessage::new(routing_key!(Net >> CompactSignedProposal).into(), self.data);
                service.mq_client.forward_msg_to_consensus(msg);
            }
            routing_key!(Consensus >> RawBytes) => {
                let msg = PubMessage::new(routing_key!(Net >> RawBytes).into(), self.data);
                service.mq_client.forward_msg_to_consensus(msg);
            }
            routing_key!(Auth >> Request) => {
                let msg = PubMessage::new(routing_key!(Net >> Request).into(), self.data);
                service.mq_client.forward_msg_to_auth(msg);
            }
            routing_key!(Auth >> GetBlockTxn) => {
                let msg = PubMessage::new(routing_key!(Net >> GetBlockTxn).into(), self.data);
                service.mq_client.forward_msg_to_auth(msg);
            }
            routing_key!(Auth >> BlockTxn) => {
                let msg = PubMessage::new(routing_key!(Net >> BlockTxn).into(), self.data);
                service.mq_client.forward_msg_to_auth(msg);
            }
            _ => {
                error!("[Network] Unexpected key {} from Remote", self.key);
            }
        }
    }
}

pub fn send_message(nodes_mgr_client: &NodesManagerClient, key: String, msg: ProtoMessage) {
    let operate = msg.get_operate();

    match operate {
        OperateType::Broadcast => {
            nodes_mgr_client.broadcast(BroadcastReq::new(key, msg));
        }
        OperateType::Single => {
            let dst = msg.get_origin();
            nodes_mgr_client.send_message(SingleTxReq::new(dst as usize, key, msg));
        }
        OperateType::Subtract => {
            // FIXME: Support subtract broadcast if necessary, just use broadcast instead.
            warn!("[MqAgent] Subtract broadcast does not support yet, use broadcast instead!");
            nodes_mgr_client.broadcast(BroadcastReq::new(key, msg));
        }
    }
}
