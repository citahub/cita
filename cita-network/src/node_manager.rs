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
use discovery::RawAddr;
use fnv::FnvHashMap;
use libproto::{Message as ProtoMessage, TryInto};
use logger::{debug, error, info, trace, warn};
use pubsub::channel::{select, tick, unbounded, Receiver, Sender};

use std::{
    collections::HashMap,
    collections::HashSet,
    convert::Into,
    io::Cursor,
    net::{SocketAddr, ToSocketAddrs},
    time::{Duration, Instant},
};
use tentacle::{
    multiaddr::ToMultiaddr, service::ServiceControl, yamux::session::SessionType, SessionId,
};

pub const DEFAULT_MAX_CONNECTS: usize = 666;
pub const DEFAULT_PORT: usize = 4000;
pub const CHECK_CONNECTED_NODES: Duration = Duration::from_secs(3);
type IsTranslated = bool;

pub struct NodesManager {
    check_connected_nodes: Receiver<Instant>,
    known_addrs: FnvHashMap<RawAddr, i32>,
    config_addrs: HashMap<String, IsTranslated>,
    connected_addrs: HashMap<SessionId, RawAddr>,
    connected_peer_keys: HashMap<Address, SessionId>,
    max_connects: usize,
    nodes_manager_client: NodesManagerClient,
    nodes_manager_service_receiver: Receiver<NodesManagerMessage>,
    service_ctrl: Option<ServiceControl>,
    peer_key: Address,
    enable_tls: bool,
    dialing_node: Option<SocketAddr>,
    repeated_connections: HashSet<SessionId>,
}

impl NodesManager {
    pub fn new(known_addrs: FnvHashMap<RawAddr, i32>) -> Self {
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
        if let Some(known_addrs) = cfg.peers {
            for addr in known_addrs {
                if let (Some(ip), Some(port)) = (addr.ip, addr.port) {
                    let addr_str = format!("{}:{}", ip, port);
                    node_mgr.config_addrs.insert(addr_str, false);
                } else {
                    warn!("[NodeManager] ip(host) & port 'MUST' be set in peers.");
                }
            }
        } else {
            warn!("[NodeManager] Does not set any peers in config file!");
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
        for (key, value) in self.config_addrs.iter_mut() {
            // The address has translated.
            if *value {
                debug!("[NodeManager] The Address {:?} has been translated.", key);
                continue;
            }
            match key.to_socket_addrs() {
                Ok(mut result) => {
                    if let Some(socket_addr) = result.next() {
                        let raw_addr = RawAddr::from(socket_addr);
                        self.known_addrs.insert(raw_addr, 100);
                        *value = true;
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
        debug!("[NodeManager] Addresses in known: {:?}", self.known_addrs);
        debug!(
            "[NodeManager] Addresses in connected: {:?}",
            self.connected_addrs
        );

        if self.connected_addrs.len() < self.max_connects {
            for key in self.known_addrs.keys() {
                if !self.connected_addrs.values().any(|value| *value == *key) {
                    info!("[NodeManager] Connect to {:?}", key.socket_addr());

                    if let Some(ref mut ctrl) = self.service_ctrl {
                        self.dialing_node = Some(key.socket_addr());
                        match ctrl.dial(key.socket_addr().to_multiaddr().unwrap()) {
                            Ok(_) => {
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
        }
    }

    pub fn set_service_task_sender(&mut self, ctrl: ServiceControl) {
        self.service_ctrl = Some(ctrl);
    }

    pub fn is_enable_tls(&self) -> bool {
        self.enable_tls
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
            known_addrs: FnvHashMap::default(),
            config_addrs: HashMap::default(),
            connected_addrs: HashMap::default(),
            connected_peer_keys: HashMap::default(),
            max_connects: DEFAULT_MAX_CONNECTS,
            nodes_manager_client: client,
            nodes_manager_service_receiver: rx,
            service_ctrl: None,
            peer_key: Address::zero(),
            enable_tls: false,
            dialing_node: None,
            repeated_connections: HashSet::default(),
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

    pub fn network_init(&self, req: NetworkInitReq) {
        self.send_req(NodesManagerMessage::NetworkInit(req));
    }

    pub fn add_connected_key(&self, req: AddConnectedKeyReq) {
        self.send_req(NodesManagerMessage::AddConnectedKey(req));
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
    DelNodeReq(DelNodeReq),
    GetRandomNodesReq(GetRandomNodesReq),
    AddConnectedNodeReq(AddConnectedNodeReq),
    DelConnectedNodeReq(DelConnectedNodeReq),
    Broadcast(BroadcastReq),
    SingleTxReq(SingleTxReq),
    GetPeerCount(GetPeerCountReq),
    NetworkInit(NetworkInitReq),
    AddConnectedKey(AddConnectedKeyReq),
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
            NodesManagerMessage::NetworkInit(req) => req.handle(service),
            NodesManagerMessage::AddConnectedKey(req) => req.handle(service),
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

pub struct AddConnectedKeyReq {
    session_id: SessionId,
    ty: SessionType,
    init_msg: InitMsg,
}

impl AddConnectedKeyReq {
    pub fn new(session_id: SessionId, ty: SessionType, init_msg: InitMsg) -> Self {
        AddConnectedKeyReq {
            session_id,
            ty,
            init_msg,
        }
    }

    pub fn handle(self, service: &mut NodesManager) {
        // Repeated connected, disconnect it.
        if let Some(repeated_id) = service.connected_peer_keys.get(&self.init_msg.peer_key) {
            if let Some(ref mut ctrl) = service.service_ctrl {
                info!(
                    "[NodeManager] New session [{:?}] repeated with [{:?}]",
                    self.session_id, *repeated_id
                );
                if self.ty == SessionType::Client {
                    // Need to replace key for its connected address.
                    if let Some(value) = service.connected_addrs.remove(&self.session_id) {
                        service.connected_addrs.insert(*repeated_id, value);
                    }
                    service.repeated_connections.insert(self.session_id);
                    info!(
                        "[NodeManager] Disconnected session [{:?}], address: {:?}",
                        self.session_id, self.init_msg.peer_key
                    );
                    let _ = ctrl.disconnect(self.session_id);
                }
            }
        } else if service.peer_key == self.init_msg.peer_key {
            // This logic would be entry twice:
            // one as server, and the other one as client.
            if let Some(dialing_node) = service.dialing_node {
                debug!(
                    "[NodeManager] Connected Self, Delete {:?} from know_addrs",
                    dialing_node
                );
                service.known_addrs.remove(&RawAddr::from(dialing_node));
                if let Some(ref mut ctrl) = service.service_ctrl {
                    let _ = ctrl.disconnect(self.session_id);
                }
            }
        } else {
            let _ = service
                .connected_peer_keys
                .insert(self.init_msg.peer_key, self.session_id);
        }

        // End of dealing with the dialing
        if self.ty == SessionType::Client {
            service.dialing_node = None;
        }
        debug!(
            "[NodeManager] Address in connected : {:?}",
            service.connected_peer_keys
        );
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
            let ret = ctrl.send_message(self.session_id, TRANSFER_PROTOCOL_ID, buf);
            info!(
                "[NodeManager] Send network init message!, id: {:?}, peer_addr: {:?}, ret: {:?}",
                self.session_id, peer_key, ret,
            );
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
        let addrs = service
            .known_addrs
            .keys()
            .take(self.num)
            .map(|addr| addr.socket_addr())
            .collect();

        if let Err(e) = self.return_channel.try_send(addrs) {
            warn!(
                "[NodeManager] Get random n nodes, send them failed : {:?}",
                e
            );
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
        info!(
            "[NodeManager] Add session [{:?}], address: {:?} to Connected_addrs.",
            self.session_id, self.addr
        );
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
        // Do not need to remove anything for disconnected a repeated connection.
        if service.repeated_connections.remove(&self.session_id) {
            return;
        }
        info!(
            "[NodeManager] Remove session [{:?}] from Connected_addrs.",
            self.session_id
        );
        service.connected_addrs.remove(&self.session_id);

        for (key, value) in service.connected_peer_keys.iter() {
            if self.session_id == *value {
                info!(
                    "[NodeManager] Remove session [{:?}] from connected_peer_keys.",
                    *key
                );
                service.connected_peer_keys.remove(&key.clone());
                break;
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
            let _ = ctrl.filter_broadcast(None, TRANSFER_PROTOCOL_ID, buf);
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
            let _ = ctrl.send_message(self.dst, TRANSFER_PROTOCOL_ID, buf);
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
