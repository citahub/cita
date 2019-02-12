use crate::citaprotocol::pubsub_message_to_network_message;
use crate::config::NetConfig;
use bytes::BytesMut;
use crossbeam_channel;
use crossbeam_channel::{select, tick, unbounded};
use discovery::RawAddr;
use fnv::FnvHashMap;
use libproto::{Message as ProtoMessage, TryInto};
use log::{debug, trace, warn};
use p2p::{context::ServiceControl, multiaddr::ToMultiaddr, SessionId};
use std::{
    collections::HashMap,
    net::SocketAddr,
    str::FromStr,
    time::{Duration, Instant},
};

pub const DEFAULT_MAX_CONNECTS: usize = 4;
pub const DEFAULT_PORT: usize = 4000;
pub const CHECK_CONNECTED_NODES: Duration = Duration::from_secs(3);

pub struct NodesManager {
    check_connected_nodes: crossbeam_channel::Receiver<Instant>,
    known_addrs: FnvHashMap<RawAddr, i32>,
    connected_addrs: HashMap<SessionId, RawAddr>,
    max_connects: usize,
    nodes_manager_client: NodesManagerClient,
    nodes_manager_service_receiver: crossbeam_channel::Receiver<NodesManagerMessage>,
    service_ctrl: Option<ServiceControl>,
}

impl NodesManager {
    pub fn new(known_addrs: FnvHashMap<RawAddr, i32>) -> Self {
        let mut node_mgr = NodesManager::default();
        node_mgr.known_addrs = known_addrs;
        node_mgr
    }

    // The [[peers]] 'MUST' be config, Otherwise leads a panic,
    // and cannot setup network service.
    pub fn from_config(cfg: NetConfig) -> Self {
        let mut node_mgr = NodesManager::default();

        let cfg_addrs = cfg
            .peers
            .expect("[NodesManager] peers MUST be config in a network config file.");

        let max_connects = cfg.max_connects.unwrap_or(DEFAULT_MAX_CONNECTS);
        node_mgr.max_connects = max_connects;

        for addr in cfg_addrs {
            let ip = addr
                .ip
                .expect("[NodeManager] ip 'MUST' be set in known_nodes.");
            let port = addr
                .port
                .expect("[NodeManager] port 'MUST' be set in known_nodes.");
            let addr_str = format!("{}:{}", ip, port);
            let socket_addr = SocketAddr::from_str(&addr_str).unwrap();
            let raw_addr = RawAddr::from(socket_addr);
            node_mgr.known_addrs.insert(raw_addr, 100);
        }

        node_mgr
    }

    pub fn run(&mut self) {
        loop {
            select! {
                recv(self.nodes_manager_service_receiver) -> msg => {
                    match msg {
                        Ok(data) => {
                            data.handle(self);
                        },
                        Err(err) => debug!("Error in {:?}", err),
                    }
                }
                recv(self.check_connected_nodes) -> _ => {
                    self.dial_nodes();
                }
            }
        }
    }

    pub fn client(&self) -> NodesManagerClient {
        self.nodes_manager_client.clone()
    }

    pub fn dial_nodes(&mut self) {
        debug!("=============================");
        for raw_addr in self.known_addrs.keys() {
            debug!("Node in known: {:?}", raw_addr.socket_addr());
        }
        debug!("-----------------------------");
        for raw_addr in self.connected_addrs.values() {
            debug!("Node in connected: {:?}", raw_addr.socket_addr());
        }
        debug!("=============================");

        if self.connected_addrs.len() < self.max_connects {
            for key in self.known_addrs.keys() {
                if !self.connected_addrs.values().any(|value| *value == *key) {
                    debug!("[dial_nodes] Connect to {:?}", key.socket_addr());

                    if let Some(ref mut ctrl) = self.service_ctrl {
                        match ctrl.dial(key.socket_addr().to_multiaddr().unwrap()) {
                            Ok(_) => {
                                debug!("[dial_nodes] Dail success");
                            }
                            Err(err) => {
                                warn!("[dial_nodes] Dail failed : {:?}", err);
                            }
                        }
                    }
                    break;
                }
            }
        }
    }

    pub fn set_service_task_sender(&mut self, ctrl: ServiceControl) {
        self.service_ctrl = Some(ctrl);
    }
}

impl Default for NodesManager {
    fn default() -> NodesManager {
        let (tx, rx) = unbounded();
        let ticker = tick(CHECK_CONNECTED_NODES);
        let client = NodesManagerClient { sender: tx };

        NodesManager {
            check_connected_nodes: ticker,
            known_addrs: FnvHashMap::default(),
            connected_addrs: HashMap::default(),
            max_connects: DEFAULT_MAX_CONNECTS,
            nodes_manager_client: client,
            nodes_manager_service_receiver: rx,
            service_ctrl: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NodesManagerClient {
    sender: crossbeam_channel::Sender<NodesManagerMessage>,
}

impl NodesManagerClient {
    pub fn new(sender: crossbeam_channel::Sender<NodesManagerMessage>) -> Self {
        NodesManagerClient { sender }
    }

    pub fn add_node(&self, req: AddNodeReq) {
        self.send_req(NodesManagerMessage::AddNodeReq(req));
    }

    pub fn del_node(&self, req: DelNodeReq) {
        self.send_req(NodesManagerMessage::DelNodeReq(req));
    }

    pub fn get_random_nodes(&self, req: GetRandomNodesReq) {
        self.send_req(NodesManagerMessage::GetRandomNodesReq(req));
    }

    pub fn add_connected_node(&self, req: AddConnectedNodeReq) {
        self.send_req(NodesManagerMessage::AddConnectedNodeReq(req));
    }

    pub fn del_connected_node(&self, req: DelConnectedNodeReq) {
        self.send_req(NodesManagerMessage::DelConnectedNodeReq(req));
    }

    pub fn broadcast(&self, req: BroadcastReq) {
        self.send_req(NodesManagerMessage::Broadcast(req));
    }

    pub fn send_message(&self, req: SingleTxReq) {
        self.send_req(NodesManagerMessage::SingleTxReq(req));
    }

    pub fn get_peer_count(&self, req: GetPeerCountReq) {
        self.send_req(NodesManagerMessage::GetPeerCount(req));
    }

    fn send_req(&self, req: NodesManagerMessage) {
        match self.sender.try_send(req) {
            Ok(_) => {
                debug!("Send message to node manager Success");
            }
            Err(err) => {
                warn!("Send message to node manager failed : {:?}", err);
            }
        }
    }
}

// Define messages for NodesManager
pub enum NodesManagerMessage {
    AddNodeReq(AddNodeReq),
    DelNodeReq(DelNodeReq),
    GetRandomNodesReq(GetRandomNodesReq),
    AddConnectedNodeReq(AddConnectedNodeReq),
    DelConnectedNodeReq(DelConnectedNodeReq),
    Broadcast(BroadcastReq),
    SingleTxReq(SingleTxReq),
    GetPeerCount(GetPeerCountReq),
}

impl NodesManagerMessage {
    pub fn handle(self, service: &mut NodesManager) {
        match self {
            NodesManagerMessage::AddNodeReq(req) => req.handle(service),
            NodesManagerMessage::DelNodeReq(req) => req.handle(service),
            NodesManagerMessage::GetRandomNodesReq(req) => req.handle(service),
            NodesManagerMessage::AddConnectedNodeReq(req) => req.handle(service),
            NodesManagerMessage::DelConnectedNodeReq(req) => req.handle(service),
            NodesManagerMessage::Broadcast(req) => req.handle(service),
            NodesManagerMessage::SingleTxReq(req) => req.handle(service),
            NodesManagerMessage::GetPeerCount(req) => req.handle(service),
        }
    }
}

pub struct AddNodeReq {
    addr: SocketAddr,
}

impl AddNodeReq {
    pub fn new(addr: SocketAddr) -> Self {
        AddNodeReq { addr }
    }

    pub fn handle(self, service: &mut NodesManager) {
        service
            .known_addrs
            .entry(RawAddr::from(self.addr))
            .or_insert(100);
    }
}

pub struct DelNodeReq {
    addr: SocketAddr,
}

impl DelNodeReq {
    pub fn new(addr: SocketAddr) -> Self {
        DelNodeReq { addr }
    }

    pub fn handle(self, service: &mut NodesManager) {
        service.known_addrs.remove(&RawAddr::from(self.addr));
    }
}

pub struct GetRandomNodesReq {
    num: usize,
    return_channel: crossbeam_channel::Sender<Vec<SocketAddr>>,
}

impl GetRandomNodesReq {
    pub fn new(num: usize, return_channel: crossbeam_channel::Sender<Vec<SocketAddr>>) -> Self {
        GetRandomNodesReq {
            num,
            return_channel,
        }
    }

    pub fn handle(self, service: &mut NodesManager) {
        let addrs = service
            .known_addrs
            .keys()
            .take(self.num)
            .map(|addr| addr.socket_addr())
            .collect();

        match self.return_channel.try_send(addrs) {
            Ok(_) => {
                debug!("Get random n nodes and send them success");
            }
            Err(err) => {
                warn!("Get random n nodes, send them failed : {:?}", err);
            }
        }
    }
}

pub struct AddConnectedNodeReq {
    addr: SocketAddr,
    session_id: SessionId,
}

impl AddConnectedNodeReq {
    pub fn new(addr: SocketAddr, session_id: SessionId) -> Self {
        AddConnectedNodeReq { addr, session_id }
    }

    pub fn handle(self, service: &mut NodesManager) {
        // FIXME: If have reached to max_connects, disconnected this node.
        service
            .connected_addrs
            .insert(self.session_id, RawAddr::from(self.addr));
    }
}

pub struct DelConnectedNodeReq {
    session_id: SessionId,
}

impl DelConnectedNodeReq {
    pub fn new(session_id: SessionId) -> Self {
        DelConnectedNodeReq { session_id }
    }

    pub fn handle(self, service: &mut NodesManager) {
        service.connected_addrs.remove(&self.session_id);
    }
}

#[derive(Debug)]
pub struct BroadcastReq {
    key: String,
    msg: ProtoMessage,
}

impl BroadcastReq {
    pub fn new(key: String, msg: ProtoMessage) -> Self {
        BroadcastReq { key, msg }
    }

    pub fn handle(self, service: &mut NodesManager) {
        trace!("Broadcast msg {:?}, from key {}", self.msg, self.key);
        let msg_bytes: Vec<u8> = self.msg.try_into().unwrap();

        let mut buf = BytesMut::with_capacity(4 + 4 + 1 + self.key.len() + msg_bytes.len());
        pubsub_message_to_network_message(&mut buf, Some((self.key, msg_bytes)));
        if let Some(ref mut ctrl) = service.service_ctrl {
            let _ = ctrl.send_message(None, 1, buf.to_vec());
        }
    }
}

pub struct SingleTxReq {
    dst: SessionId,
    key: String,
    msg: ProtoMessage,
}

impl SingleTxReq {
    pub fn new(dst: SessionId, key: String, msg: ProtoMessage) -> Self {
        SingleTxReq { dst, key, msg }
    }

    pub fn handle(self, service: &mut NodesManager) {
        trace!(
            "Send msg {:?} to {}, from key {}",
            self.msg,
            self.dst,
            self.key
        );
        let msg_bytes: Vec<u8> = self.msg.try_into().unwrap();

        let mut buf = BytesMut::with_capacity(4 + 4 + 1 + self.key.len() + msg_bytes.len());
        pubsub_message_to_network_message(&mut buf, Some((self.key, msg_bytes)));
        if let Some(ref mut ctrl) = service.service_ctrl {
            //FIXME: handle the error!
            let _ = ctrl.send_message(Some(vec![self.dst]), 1, buf.to_vec());
        }
    }
}

pub struct GetPeerCountReq {
    return_channel: crossbeam_channel::Sender<usize>,
}

impl GetPeerCountReq {
    pub fn new(return_channel: crossbeam_channel::Sender<usize>) -> Self {
        GetPeerCountReq { return_channel }
    }

    pub fn handle(self, service: &mut NodesManager) {
        let peer_count = service.connected_addrs.len();

        match self.return_channel.try_send(peer_count) {
            Ok(_) => {
                debug!("Get peer count and send it success");
            }
            Err(err) => {
                warn!(
                    "Get peer count {}, but send it failed : {:?}",
                    peer_count, err
                );
            }
        }
    }
}
