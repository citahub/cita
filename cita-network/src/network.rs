use crate::mq_client::{MqClient, PubMessage};
use crate::node_manager::{BroadcastReq, GetPeerCountReq, NodesManagerClient};
use crate::synchronizer::{SynchronizerClient, SynchronizerMessage};
use crossbeam_channel::{self, unbounded};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::routing_key;
use libproto::snapshot::{Cmd, Resp, SnapshotResp};
use libproto::{Message as ProtoMessage, Response};
use libproto::{TryFrom, TryInto};
use log::{debug, error, info, trace, warn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Network {
    is_pause: Arc<AtomicBool>,
    mq_client: MqClient,
    network_client: NetworkClient,
    nodes_mgr_client: NodesManagerClient,
    sync_client: SynchronizerClient,
    msg_receiver: crossbeam_channel::Receiver<NetworkMessage>,
}

impl Network {
    pub fn new(
        mq_client: MqClient,
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
    sender: crossbeam_channel::Sender<NetworkMessage>,
}

impl NetworkClient {
    pub fn new(sender: crossbeam_channel::Sender<NetworkMessage>) -> Self {
        NetworkClient { sender }
    }

    pub fn handle_local_message(&self, msg: LocalMessage) {
        self.send_msg(NetworkMessage::LocalMessage(msg));
    }

    pub fn handle_remote_message(&self, msg: RemoteMessage) {
        self.send_msg(NetworkMessage::RemoteMessage(msg));
    }

    fn send_msg(&self, msg: NetworkMessage) {
        self.sender.try_send(msg).unwrap_or_else(|err| {
            warn!("Send message to network failed : {:?}", err);
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
        trace!("Network receive Message from Local/{}", self.key);

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
                service.nodes_mgr_client.broadcast(BroadcastReq::new(
                    routing_key!(Synchronizer >> SyncResponse).into(),
                    msg,
                ));
            }
            routing_key!(Jsonrpc >> RequestNet) => {
                self.reply_rpc(&self.data, service);
            }
            routing_key!(Snapshot >> SnapshotReq) => {
                info!("Set disconnect and response");
                self.snapshot_req(&self.data, service);
            }
            _ => {
                error!("Unexpected key {} from Local", self.key);
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
                warn!("[reply_rpc] Receive unexpected rpc data");
            }
        }
    }

    fn snapshot_req(&self, data: &[u8], service: &mut Network) {
        let mut msg = ProtoMessage::try_from(data).unwrap();
        let req = msg.take_snapshot_req().unwrap();
        let mut resp = SnapshotResp::new();
        let mut send = false;

        match req.cmd {
            Cmd::Snapshot => {
                info!("[snapshot] receive cmd: Snapshot");
            }
            Cmd::Begin => {
                info!("[snapshot] receive cmd: Begin");
                service.is_pause.store(true, Ordering::SeqCst);
                resp.set_resp(Resp::BeginAck);
                resp.set_flag(true);
                send = true;
            }
            Cmd::Restore => {
                info!("[snapshot] receive cmd: Restore");
            }
            Cmd::Clear => {
                info!("[snapshot] receive cmd: Clear");
                resp.set_resp(Resp::ClearAck);
                resp.set_flag(true);
                send = true;
            }
            Cmd::End => {
                info!("[snapshot] receive cmd: End");
                service.is_pause.store(false, Ordering::SeqCst);
                resp.set_resp(Resp::EndAck);
                resp.set_flag(true);
                send = true;
            }
        }

        if send {
            let msg: ProtoMessage = resp.into();
            service.mq_client.send_snapshot_resp(PubMessage::new(
                routing_key!(Net >> SnapshotResp).into(),
                (&msg).try_into().unwrap(),
            ));
        }
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
        trace!("Network receive Message from Remote/{}", self.key);

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
                error!("Unexpected key {} from Remote", self.key);
            }
        }
    }
}
