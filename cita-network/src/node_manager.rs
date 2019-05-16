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

use crate::cita_protocol::{pubsub_message_to_network_message, CITA_FRAME_HEADER_LEN};
use crate::config::NetConfig;
use crate::p2p_protocol::transfer::TRANSFER_PROTOCOL_ID;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use cita_types::Address;
use fnv::FnvHashMap as HashMap;
use libproto::{Message as ProtoMessage, TryInto};
use logger::{debug, error, info, trace, warn};
use notify::DebouncedEvent;
use pubsub::channel::{select, tick, unbounded, Receiver, Sender};
use rand;
use std::sync::mpsc::Receiver as StdReceiver;
use std::{
    collections::{BTreeMap, BTreeSet},
    convert::Into,
    io::Cursor,
    net::{SocketAddr, ToSocketAddrs},
    time::{Duration, Instant},
};
use tentacle::{
    service::{DialProtocol, ServiceControl, SessionType, TargetSession},
    utils::socketaddr_to_multiaddr,
    SessionId,
};

pub const DEFAULT_MAX_CONNECTS: usize = 666;
pub const DEFAULT_MAX_KNOWN_ADDRS: usize = 1000;
pub const DEFAULT_PORT: usize = 4000;
pub const CHECK_CONNECTED_NODES: Duration = Duration::from_secs(3);

// Score uses to manage known_nodes list. If a node has too low score, do not dial it again.
// Maybe some complex algorithm can be designed later. But for now, just keeps as simple as below:
//  1. Deducts 10 score for each Dial;
//  2. Deducts 25 score for each Dial Error;
//  3. Deducts 20 score for each Disconnected by server;
//  4. Add 5 score for every dialing round if the node keep on line; so If a node keep on line,
//     it will get FULL_SCORE very fast.
//  5. Gives a Time sugar score (2 : nodes was configured in config file, and 1 : nodes was
//     discovered by P2P framework ) when a node's score less than MIN_DIALING_SCORE;

// A new node come into known_nodes list has a FULL_SCORE.
pub const FULL_SCORE: i32 = 100;
// Score lower than MIN_DIALING_SCORE, stop dialing.
pub const MIN_DIALING_SCORE: i32 = 60;
// A node needs DIALING_SCORE for every dial.
pub const DIALING_SCORE: i32 = 10;
// A node connected successfully, can get SUCCESS_DIALING_SCORE.
pub const SUCCESS_DIALING_SCORE: i32 = 10;
// A node is refused by server, should need REFUSED_SCORE each time.
pub const REFUSED_SCORE: i32 = 20;
// A node is dialed error by client, should need DIALED_ERROR_SCORE each time.
pub const DIALED_ERROR_SCORE: i32 = 25;
// A node is dialed error by client, should need DIALED_ERROR_SCORE each time.
pub const KEEP_ON_LINE_SCORE: i32 = 5;

#[derive(Debug, PartialEq)]
pub enum NodeSource {
    FromConfig,
    FromDiscovery,
}

#[derive(Debug)]
pub struct NodeStatus {
    // score: Score for a node, it will affect whether the node will be chosen to dail again,
    // or be deleted from the known_addresses list. But for now, it useless.
    pub score: i32,

    // session_id: Indicates that this node has been connected to a session. 'None' for has not
    // connected yet.
    pub session_id: Option<SessionId>,
    pub node_src: NodeSource,
}

impl NodeStatus {
    pub fn new(score: i32, session_id: Option<SessionId>, node_src: NodeSource) -> Self {
        NodeStatus {
            score,
            session_id,
            node_src,
        }
    }
}

#[derive(Debug)]
pub struct SessionInfo {
    pub ty: SessionType,
    pub addr: SocketAddr,
}

impl SessionInfo {
    pub fn new(ty: SessionType, addr: SocketAddr) -> Self {
        SessionInfo { ty, addr }
    }
}

#[derive(Debug)]
pub struct TransformAddr {
    // Real linked addr
    pub conn_addr: SocketAddr,
    // Outbound addr transformed from Inbound addr
    pub trans_addr: Option<SocketAddr>,
}

impl TransformAddr {
    pub fn new(conn_addr: SocketAddr, trans_addr: Option<SocketAddr>) -> Self {
        TransformAddr {
            conn_addr,
            trans_addr,
        }
    }
}

pub struct NodesManager {
    known_addrs: HashMap<SocketAddr, NodeStatus>,
    config_addrs: BTreeMap<String, Option<SocketAddr>>,

    connected_addrs: HashMap<SessionId, TransformAddr>,
    pending_connected_addrs: HashMap<SessionId, SessionInfo>,

    connected_peer_keys: HashMap<Address, SessionId>,

    check_connected_nodes: Receiver<Instant>,
    max_connects: usize,
    nodes_manager_client: NodesManagerClient,
    nodes_manager_service_receiver: Receiver<NodesManagerMessage>,
    service_ctrl: Option<ServiceControl>,
    peer_key: Address,
    enable_tls: bool,
    dialing_node: Option<SocketAddr>,
    self_addr: Option<SocketAddr>,
}

impl NodesManager {
    pub fn new(known_addrs: HashMap<SocketAddr, NodeStatus>) -> Self {
        let mut node_mgr = NodesManager::default();
        node_mgr.known_addrs = known_addrs;
        node_mgr
    }

    pub fn from_config(cfg: NetConfig, key: Address) -> Self {
        let mut node_mgr = NodesManager::default();
        let max_connects = cfg.max_connects.unwrap_or(DEFAULT_MAX_CONNECTS);
        node_mgr.max_connects = max_connects;
        node_mgr.peer_key = key;

        if let Some(enable_tls) = cfg.enable_tls {
            node_mgr.enable_tls = enable_tls;
        }
        if let Some(cfg_addrs) = cfg.peers {
            for addr in cfg_addrs {
                if let (Some(ip), Some(port)) = (addr.ip, addr.port) {
                    let addr_str = format!("{}:{}", ip, port);
                    node_mgr.config_addrs.insert(addr_str, None);
                } else {
                    warn!("[NodeManager] ip(host) & port 'MUST' be set in peers.");
                }
            }
        } else {
            warn!("[NodeManager] Does not set any peers in config file!");
        }
        node_mgr
    }

    pub fn notify_config_change(
        rx: StdReceiver<DebouncedEvent>,
        node_client: NodesManagerClient,
        fname: String,
    ) {
        loop {
            match rx.recv() {
                Ok(event) => match event {
                    DebouncedEvent::Create(path_buf) | DebouncedEvent::Write(path_buf) => {
                        if path_buf.is_file() {
                            let file_name = path_buf.file_name().unwrap().to_str().unwrap();
                            if file_name == fname {
                                info!("file {} changed, will auto reload!", file_name);

                                let config = NetConfig::new(file_name);
                                if let Some(peers) = config.peers {
                                    let mut addr_strs = Vec::new();
                                    for addr in peers {
                                        if let (Some(ip), Some(port)) = (addr.ip, addr.port) {
                                            addr_strs.push(format!("{}:{}", ip, port));
                                        }
                                    }
                                    node_client.fix_modified_config(ModifiedConfigPeersReq::new(
                                        addr_strs,
                                    ));
                                }
                            }
                        }
                    }
                    _ => trace!("file notify event: {:?}", event),
                },
                Err(e) => warn!("watch error: {:?}", e),
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            select! {
                recv(self.nodes_manager_service_receiver) -> msg => {
                    match msg {
                        Ok(data) => {
                            data.handle(self);
                        },
                        Err(err) => error!("[NodeManager] Receive data error {:?}", err),
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
        if let Some(dialing_node) = self.dialing_node {
            info!(
                "[NodeManager] Dialing node: {:?}, waiting for next round.",
                dialing_node
            );
            return;
        }
        self.translate_address();

        // If connected node has not reach MAX, select a node from known_addrs to dial.
        if self.connected_addrs.len() < self.max_connects {
            for (key, value) in self.known_addrs.iter_mut() {
                // Node has been connected
                if let Some(session_id) = value.session_id {
                    debug!(
                        "[NodeManager] Address {:?} has been connected on : {:?}.",
                        *key, session_id
                    );

                    // Node keep on line, reward KEEP_ON_LINE_SCORE.
                    value.score = if (value.score + KEEP_ON_LINE_SCORE) > FULL_SCORE {
                        FULL_SCORE as i32
                    } else {
                        value.score + KEEP_ON_LINE_SCORE
                    };
                    continue;
                }

                // Give 50% probability to select this node, this design can avoid two nodes
                // simultaneously dialing each other.
                let selected_miss: bool = (rand::random::<u32>() % 2) != 0;
                if selected_miss {
                    debug!(
                        "[NodeManager] Address {:?} selects miss in this round.",
                        *key
                    );
                    continue;
                }

                if let Some(self_addr) = self.self_addr {
                    if *key == self_addr {
                        debug!(
                            "[NodeManager] Trying to connected self: {:?}, skip it",
                            self_addr
                        );
                        continue;
                    }
                }

                // Score design prevents the client from dialing to a node all the time.
                if value.score < MIN_DIALING_SCORE {
                    debug!(
                        "[NodeManager] Address {:?} has to low score ({:?}) to dial.",
                        *key, value.score
                    );

                    // The node will get time sugar, the nodes which in config file can get 2, and the
                    // other nodes which discovered by P2P can get 1.
                    value.score += if value.node_src == NodeSource::FromConfig {
                        2
                    } else {
                        1
                    };
                    continue;
                }

                // Dial this address
                if let Some(ref mut ctrl) = self.service_ctrl {
                    self.dialing_node = Some(*key);
                    info!("Trying to dial: {:?}", self.dialing_node);
                    match ctrl.dial(socketaddr_to_multiaddr(*key), DialProtocol::All) {
                        Ok(_) => {
                            // Need DIALING_SCORE for every dial.
                            value.score -= DIALING_SCORE;
                            debug!("[NodeManager] Dail success");
                        }
                        Err(err) => {
                            warn!("[NodeManager] Dail failed : {:?}", err);
                        }
                    }
                }
                break;
            }
        }

        debug!(
            "[NodeManager] connected_addrs info: {:?}",
            self.connected_addrs
        );
        debug!("[NodeManager] known_addrs info: {:?}", self.known_addrs);
        debug!(
            "[NodeManager] Address in connected : {:?}",
            self.connected_peer_keys
        );
    }

    pub fn set_service_task_sender(&mut self, ctrl: ServiceControl) {
        self.service_ctrl = Some(ctrl);
    }

    pub fn is_enable_tls(&self) -> bool {
        self.enable_tls
    }

    pub fn translate_address(&mut self) {
        for (key, value) in self.config_addrs.iter_mut() {
            // The address has translated.
            if value.is_some() {
                debug!("[NodeManager] The Address {:?} has been translated.", key);
                continue;
            }
            match key.to_socket_addrs() {
                Ok(mut result) => {
                    if let Some(socket_addr) = result.next() {
                        // An init node from config file, give it FULL_SCORE.
                        let node_status = NodeStatus::new(FULL_SCORE, None, NodeSource::FromConfig);
                        self.known_addrs.insert(socket_addr, node_status);
                        *value = Some(socket_addr);
                    } else {
                        error!("[NodeManager] Can not convert to socket address!");
                    }
                }
                Err(e) => {
                    error!(
                        "[NodeManager] Can not convert to socket address! error: {}",
                        e
                    );
                }
            }
        }
    }
}

impl Default for NodesManager {
    fn default() -> NodesManager {
        let (tx, rx) = unbounded();
        let ticker = tick(CHECK_CONNECTED_NODES);
        let client = NodesManagerClient { sender: tx };

        // Set enable_tls = false as default.
        NodesManager {
            check_connected_nodes: ticker,
            known_addrs: HashMap::default(),
            config_addrs: BTreeMap::default(),
            connected_addrs: HashMap::default(),
            connected_peer_keys: HashMap::default(),
            pending_connected_addrs: HashMap::default(),
            max_connects: DEFAULT_MAX_CONNECTS,
            nodes_manager_client: client,
            nodes_manager_service_receiver: rx,
            service_ctrl: None,
            peer_key: Address::zero(),
            enable_tls: false,
            dialing_node: None,
            self_addr: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NodesManagerClient {
    sender: Sender<NodesManagerMessage>,
}

impl NodesManagerClient {
    pub fn new(sender: Sender<NodesManagerMessage>) -> Self {
        NodesManagerClient { sender }
    }

    pub fn add_node(&self, req: AddNodeReq) {
        self.send_req(NodesManagerMessage::AddNodeReq(req));
    }

    pub fn dialed_error(&self, req: DialedErrorReq) {
        self.send_req(NodesManagerMessage::DialedErrorReq(req));
    }

    pub fn connected_self(&self, req: ConnectedSelfReq) {
        self.send_req(NodesManagerMessage::ConnectedSelf(req));
    }

    pub fn get_random_nodes(&self, req: GetRandomNodesReq) {
        self.send_req(NodesManagerMessage::GetRandomNodesReq(req));
    }

    pub fn pending_connected_node(&self, req: PendingConnectedNodeReq) {
        self.send_req(NodesManagerMessage::PendingConnectedNodeReq(req));
    }

    pub fn del_connected_node(&self, req: DelConnectedNodeReq) {
        self.send_req(NodesManagerMessage::DelConnectedNodeReq(req));
    }

    pub fn add_repeated_node(&self, req: AddRepeatedNodeReq) {
        self.send_req(NodesManagerMessage::AddRepeatedNode(req));
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

    pub fn get_peers_info(&self, req: GetPeersInfoReq) {
        self.send_req(NodesManagerMessage::GetPeersInfo(req));
    }

    pub fn network_init(&self, req: NetworkInitReq) {
        self.send_req(NodesManagerMessage::NetworkInit(req));
    }

    pub fn add_connected_node(&self, req: AddConnectedNodeReq) {
        self.send_req(NodesManagerMessage::AddConnectedNode(req));
    }

    pub fn fix_modified_config(&self, req: ModifiedConfigPeersReq) {
        self.send_req(NodesManagerMessage::ModifiedConfigPeers(req));
    }

    fn send_req(&self, req: NodesManagerMessage) {
        if let Err(e) = self.sender.try_send(req) {
            warn!(
                "[NodesManager] Send message to node manager failed : {:?}",
                e
            );
        }
    }
}

// Define messages for NodesManager
pub enum NodesManagerMessage {
    AddNodeReq(AddNodeReq),
    DialedErrorReq(DialedErrorReq),
    GetRandomNodesReq(GetRandomNodesReq),
    PendingConnectedNodeReq(PendingConnectedNodeReq),
    DelConnectedNodeReq(DelConnectedNodeReq),
    Broadcast(BroadcastReq),
    SingleTxReq(SingleTxReq),
    GetPeerCount(GetPeerCountReq),
    NetworkInit(NetworkInitReq),
    AddConnectedNode(AddConnectedNodeReq),
    AddRepeatedNode(AddRepeatedNodeReq),
    ConnectedSelf(ConnectedSelfReq),
    GetPeersInfo(GetPeersInfoReq),
    ModifiedConfigPeers(ModifiedConfigPeersReq),
}

impl NodesManagerMessage {
    pub fn handle(self, service: &mut NodesManager) {
        match self {
            NodesManagerMessage::AddNodeReq(req) => req.handle(service),
            NodesManagerMessage::DialedErrorReq(req) => req.handle(service),
            NodesManagerMessage::GetRandomNodesReq(req) => req.handle(service),
            NodesManagerMessage::PendingConnectedNodeReq(req) => req.handle(service),
            NodesManagerMessage::DelConnectedNodeReq(req) => req.handle(service),
            NodesManagerMessage::Broadcast(req) => req.handle(service),
            NodesManagerMessage::SingleTxReq(req) => req.handle(service),
            NodesManagerMessage::GetPeerCount(req) => req.handle(service),
            NodesManagerMessage::NetworkInit(req) => req.handle(service),
            NodesManagerMessage::AddConnectedNode(req) => req.handle(service),
            NodesManagerMessage::AddRepeatedNode(req) => req.handle(service),
            NodesManagerMessage::ConnectedSelf(req) => req.handle(service),
            NodesManagerMessage::GetPeersInfo(req) => req.handle(service),
            NodesManagerMessage::ModifiedConfigPeers(req) => req.handle(service),
        }
    }
}

#[derive(Default, Clone)]
pub struct InitMsg {
    pub chain_id: u64,
    pub peer_key: Address,
}

impl Into<Vec<u8>> for InitMsg {
    fn into(self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut key_data: [u8; 20] = Default::default();
        let mut chain_id_data = vec![];
        chain_id_data.write_u64::<BigEndian>(self.chain_id).unwrap();
        self.peer_key.copy_to(&mut key_data[..]);

        out.extend_from_slice(&chain_id_data);
        out.extend_from_slice(&key_data);
        out
    }
}

impl From<Vec<u8>> for InitMsg {
    fn from(data: Vec<u8>) -> InitMsg {
        let mut chain_id_data: [u8; 8] = Default::default();
        chain_id_data.copy_from_slice(&data[..8]);
        let mut chain_id_data = Cursor::new(chain_id_data);
        let chain_id = chain_id_data.read_u64::<BigEndian>().unwrap();
        let peer_key = Address::from_slice(&data[8..]);

        InitMsg { chain_id, peer_key }
    }
}

pub struct AddConnectedNodeReq {
    session_id: SessionId,
    ty: SessionType,
    init_msg: InitMsg,
}

impl AddConnectedNodeReq {
    pub fn new(session_id: SessionId, ty: SessionType, init_msg: InitMsg) -> Self {
        AddConnectedNodeReq {
            session_id,
            ty,
            init_msg,
        }
    }

    pub fn handle(self, service: &mut NodesManager) {
        if let Some(repeated_id) = service.connected_peer_keys.get(&self.init_msg.peer_key) {
            // Repeated connected, it can a duplicated connected to the same node, or a duplicated
            // node connected to this server. But in either case, disconnect this session.
            // In P2P encrypted communication mode, the repeated connection will be detected by
            // P2P framework, handling this situation by sending a `AddRepeatedNodeReq` message to
            // NodesManager. See the `handle` in `AddRepeatedNodeReq` for more detail.
            info!(
                "[NodeManager] New session [{:?}] repeated with [{:?}], disconnect this session.",
                self.session_id, *repeated_id
            );

            // It is a repeated_session, but not a repeated node.
            if let Some(dialing_addr) = service.dialing_node {
                if self.ty == SessionType::Outbound {
                    if let Some(ref mut node_status) = service.known_addrs.get_mut(&dialing_addr) {
                        node_status.session_id = Some(*repeated_id);
                        node_status.score += SUCCESS_DIALING_SCORE;

                        let _ = service.connected_addrs.entry(*repeated_id).and_modify(|v| {
                            v.trans_addr = Some(dialing_addr);
                        });
                    }
                }
            }

            if let Some(ref mut ctrl) = service.service_ctrl {
                let _ = ctrl.disconnect(self.session_id);
            }
        } else if service.peer_key == self.init_msg.peer_key {
            // Connected self, disconnected the session.
            // In P2P encrypted communication mode, the `connected self` will be detected by
            // P2P framework, handling this situation by sending a `ConnectedSelfReq` message to
            // NodesManager. See the `handle` in `ConnectedSelfReq` for more detail.
            // This logic would be entry twice:
            // one as server, and the other one as client.
            if let Some(dialing_node) = service.dialing_node {
                debug!(
                    "[NodeManager] Connected Self, Delete {:?} from know_addrs",
                    dialing_node
                );
                service.self_addr = Some(dialing_node);
                if let Some(ref mut ctrl) = service.service_ctrl {
                    let _ = ctrl.disconnect(self.session_id);
                }
            }
        } else {
            // Found a successful connection after exchanging `init message`.
            // FIXME: If have reached to max_connects, disconnected this node.
            // Add connected address.
            if let Some(session_info) = service.pending_connected_addrs.remove(&self.session_id) {
                info!(
                    "[NodeManager] Add session [{:?}], address: {:?} to Connected_addrs.",
                    self.session_id, session_info.addr
                );
                let _ = service
                    .connected_addrs
                    .insert(self.session_id, TransformAddr::new(session_info.addr, None));

                // Add connected peer keys
                let _ = service
                    .connected_peer_keys
                    .insert(self.init_msg.peer_key, self.session_id);

                // If it is an active connection, need to set this node in known_addrs has been connected.
                if self.ty == SessionType::Outbound {
                    if let Some(ref mut node_status) =
                        service.known_addrs.get_mut(&session_info.addr)
                    {
                        node_status.session_id = Some(self.session_id);
                        node_status.score += SUCCESS_DIALING_SCORE;
                    }
                }
            }

            info!(
                "[NodeManager] connected_addrs info: {:?}",
                service.connected_addrs
            );
            info!("[NodeManager] known_addrs info: {:?}", service.known_addrs);

            info!(
                "[NodeManager] Address in connected : {:?}",
                service.connected_peer_keys
            );
        }

        // End of dealing node for this round.
        if self.ty == SessionType::Outbound {
            service.dialing_node = None;
        }
    }
}

#[derive(Default)]
pub struct NetworkInitReq {
    session_id: SessionId,
}

impl NetworkInitReq {
    pub fn new(session_id: SessionId) -> Self {
        NetworkInitReq { session_id }
    }

    pub fn handle(self, service: &mut NodesManager) {
        let peer_key = service.peer_key;

        let send_key = "network.init".to_string();
        let init_msg = InitMsg {
            chain_id: 0,
            peer_key,
        };
        let msg_bytes: Vec<u8> = init_msg.into();

        let mut buf = Vec::with_capacity(CITA_FRAME_HEADER_LEN + send_key.len() + msg_bytes.len());
        pubsub_message_to_network_message(&mut buf, Some((send_key, msg_bytes)));

        if let Some(ref mut ctrl) = service.service_ctrl {
            // FIXME: handle the error!
            let ret = ctrl.send_message_to(self.session_id, TRANSFER_PROTOCOL_ID, buf.into());
            info!(
                "[NodeManager] Send network init message!, id: {:?}, peer_addr: {:?}, ret: {:?}",
                self.session_id, peer_key, ret,
            );
        }
    }
}

pub struct AddNodeReq {
    addr: SocketAddr,
    source: NodeSource,
}

impl AddNodeReq {
    pub fn new(addr: SocketAddr, source: NodeSource) -> Self {
        AddNodeReq { addr, source }
    }

    pub fn handle(self, service: &mut NodesManager) {
        if service.known_addrs.len() > DEFAULT_MAX_KNOWN_ADDRS {
            warn!(
                "[NodeManager] Known address has reach Max: {:?}",
                DEFAULT_MAX_KNOWN_ADDRS,
            );
            return;
        }
        // Add a new node, using a default node status.
        let default_node_status = NodeStatus::new(FULL_SCORE, None, self.source);
        service
            .known_addrs
            .entry(self.addr)
            .or_insert(default_node_status);
    }
}

pub struct DialedErrorReq {
    addr: SocketAddr,
}

impl DialedErrorReq {
    pub fn new(addr: SocketAddr) -> Self {
        DialedErrorReq { addr }
    }

    pub fn handle(self, service: &mut NodesManager) {
        if let Some(ref mut node_status) = service.known_addrs.get_mut(&self.addr) {
            node_status.score -= DIALED_ERROR_SCORE;
        }

        // Catch a dial error, this dialing finished
        service.dialing_node = None;
    }
}

pub struct AddRepeatedNodeReq {
    addr: SocketAddr,
    session_id: SessionId,
}

impl AddRepeatedNodeReq {
    pub fn new(addr: SocketAddr, session_id: SessionId) -> Self {
        AddRepeatedNodeReq { addr, session_id }
    }

    pub fn handle(self, service: &mut NodesManager) {
        info!(
            "[NodeManager] Dialing a repeated node [{:?}], on session: {:?}.",
            self.addr, self.session_id
        );

        if let Some(ref mut node_status) = service.known_addrs.get_mut(&self.addr) {
            node_status.session_id = Some(self.session_id);
            node_status.score += SUCCESS_DIALING_SCORE;
        }

        // This dialing is finished.
        service.dialing_node = None;
    }
}

pub struct GetRandomNodesReq {
    num: usize,
    return_channel: Sender<Vec<SocketAddr>>,
}

impl GetRandomNodesReq {
    pub fn new(num: usize, return_channel: Sender<Vec<SocketAddr>>) -> Self {
        GetRandomNodesReq {
            num,
            return_channel,
        }
    }

    pub fn handle(self, service: &mut NodesManager) {
        let addrs = service.known_addrs.keys().take(self.num).cloned().collect();

        if let Err(e) = self.return_channel.try_send(addrs) {
            warn!(
                "[NodeManager] Get random n nodes, send them failed : {:?}",
                e
            );
        }
    }
}

pub struct PendingConnectedNodeReq {
    session_id: SessionId,
    addr: SocketAddr,
    ty: SessionType,
}

impl PendingConnectedNodeReq {
    pub fn new(session_id: SessionId, addr: SocketAddr, ty: SessionType) -> Self {
        PendingConnectedNodeReq {
            session_id,
            addr,
            ty,
        }
    }

    pub fn handle(self, service: &mut NodesManager) {
        if service.connected_addrs.len() >= service.max_connects {
            // Has reached to max connects, refuse this connection
            info!(
                "[NodeManager] Has reached to max connects [{:?}], refuse Session [{:?}], address: {:?}",
                service.max_connects, self.session_id, self.addr
            );
            if let Some(ref mut ctrl) = service.service_ctrl {
                let _ = ctrl.disconnect(self.session_id);
            }
            return;
        }

        info!(
            "[NodeManager] Session [{:?}], address: {:?} pending to add to Connected_addrs.",
            self.session_id, self.addr
        );
        service
            .pending_connected_addrs
            .insert(self.session_id, SessionInfo::new(self.ty, self.addr));
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
        info!("[NodeManager] Disconnected session [{:?}]", self.session_id);

        if let Some(addr) = service.connected_addrs.remove(&self.session_id) {
            let trans_addr = addr.trans_addr.unwrap_or(addr.conn_addr);
            self.fix_node_status(trans_addr, service);

            // Remove connected peer keys
            for (key, value) in service.connected_peer_keys.iter() {
                if self.session_id == *value {
                    info!(
                        "[NodeManager] Remove session [{:?}] from connected_peer_keys.",
                        *value
                    );
                    service.connected_peer_keys.remove(&key.clone());
                    break;
                }
            }
        }

        // Remove pending connected
        if let Some(session_info) = service.pending_connected_addrs.remove(&self.session_id) {
            if session_info.ty == SessionType::Outbound {
                self.fix_node_status(session_info.addr, service);
                // Close a session which open as client, end of this dialing.
                service.dialing_node = None;
            }
        }
    }

    fn fix_node_status(&self, addr: SocketAddr, service: &mut NodesManager) {
        // Set the node as disconnected in known_addrs
        if let Some(ref mut node_status) = service.known_addrs.get_mut(&addr) {
            if let Some(session_id) = node_status.session_id {
                if session_id == self.session_id {
                    info!("Reset node status of address {:?} to None", addr);
                    node_status.score -= REFUSED_SCORE;
                    node_status.session_id = None;
                } else {
                    warn!(
                        "[NodeManager] Expected session id: {:?}, but found: {:?}",
                        self.session_id, session_id
                    );
                }
            } else {
                error!("[NodeManager] Can not get node status from known_addr, this should not happen!");
            }
        }
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
        trace!(
            "[NodeManager] Broadcast msg {:?}, from key {}",
            self.msg,
            self.key
        );
        let msg_bytes: Vec<u8> = self.msg.try_into().unwrap();

        let mut buf = Vec::with_capacity(CITA_FRAME_HEADER_LEN + self.key.len() + msg_bytes.len());
        pubsub_message_to_network_message(&mut buf, Some((self.key, msg_bytes)));
        if let Some(ref mut ctrl) = service.service_ctrl {
            let _ = ctrl.filter_broadcast(TargetSession::All, TRANSFER_PROTOCOL_ID, buf.into());
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
            "[NodeManager] Send msg {:?} to {}, from key {}",
            self.msg,
            self.dst,
            self.key
        );
        let msg_bytes: Vec<u8> = self.msg.try_into().unwrap();

        let mut buf = Vec::with_capacity(CITA_FRAME_HEADER_LEN + self.key.len() + msg_bytes.len());
        pubsub_message_to_network_message(&mut buf, Some((self.key, msg_bytes)));
        if let Some(ref mut ctrl) = service.service_ctrl {
            // FIXME: handle the error!
            let _ = ctrl.send_message_to(self.dst, TRANSFER_PROTOCOL_ID, buf.into());
        }
    }
}

pub struct GetPeerCountReq {
    return_channel: Sender<usize>,
}

impl GetPeerCountReq {
    pub fn new(return_channel: Sender<usize>) -> Self {
        GetPeerCountReq { return_channel }
    }

    pub fn handle(self, service: &mut NodesManager) {
        let peer_count = service.connected_addrs.len();

        if let Err(e) = self.return_channel.try_send(peer_count) {
            warn!(
                "[NodeManager] Get peer count {}, but send it failed : {:?}",
                peer_count, e
            );
        }
    }
}

pub struct GetPeersInfoReq {
    return_channel: Sender<HashMap<Address, String>>,
}

impl GetPeersInfoReq {
    pub fn new(return_channel: Sender<HashMap<Address, String>>) -> Self {
        GetPeersInfoReq { return_channel }
    }

    pub fn handle(self, service: &mut NodesManager) {
        let mut peers = HashMap::default();

        for (key, value) in service.connected_peer_keys.iter() {
            if let Some(addr) = service.connected_addrs.get(&value) {
                peers.insert(key.clone(), addr.conn_addr.ip().to_string());
            } else {
                warn!(
                    "[NodeManager] Can not get socket address for session {} from connected_addr. It must be something wrong!",
                    value
                );
            }
        }

        debug!("[NodeManager] get peers info : {:?}", peers);

        if let Err(e) = self.return_channel.try_send(peers) {
            warn!("[NodeManager] Send peers info failed : {:?}", e);
        }
    }
}

pub struct ConnectedSelfReq {
    addr: SocketAddr,
}

impl ConnectedSelfReq {
    pub fn new(addr: SocketAddr) -> Self {
        ConnectedSelfReq { addr }
    }

    pub fn handle(self, service: &mut NodesManager) {
        service.self_addr = Some(self.addr);
        service.dialing_node = None;
    }
}

pub struct ModifiedConfigPeersReq {
    peers: Vec<String>,
}

impl ModifiedConfigPeersReq {
    pub fn new(peers: Vec<String>) -> Self {
        ModifiedConfigPeersReq { peers }
    }

    pub fn handle(self, service: &mut NodesManager) {
        // If new config deleted some peer,disconnect and remove it from known addrs
        let mut keys: BTreeSet<_> = service.config_addrs.keys().cloned().collect();
        for peer in &self.peers {
            keys.remove(peer);
        }

        info!("left peers {:?}", self.peers);

        // The remainder in keys will be disconnected
        for key in keys {
            service.config_addrs.remove(&key).and_then(|addr| {
                addr.and_then(|addr| {
                    service.known_addrs.remove(&addr).and_then(|node_status| {
                        node_status.session_id.and_then(|sid| {
                            service
                                .service_ctrl
                                .as_mut()
                                .and_then(|ctrl| ctrl.disconnect(sid).ok())
                        })
                    })
                })
            });
        }
        for peer in self.peers {
            service.config_addrs.entry(peer).or_insert(None);
        }
    }
}
