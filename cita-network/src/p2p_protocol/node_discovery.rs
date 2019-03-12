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

use crate::node_manager::{AddNodeReq, GetRandomNodesReq, NodesManagerClient};
use crossbeam_channel::unbounded;
use discovery::{AddressManager, Direction, Discovery, DiscoveryHandle, Misbehavior, Substream};
use fnv::FnvHashMap;
use futures::{
    prelude::*,
    sync::mpsc::{channel, Sender},
};
use logger::{debug, info, warn};
use tentacle::{
    context::{ServiceContext, SessionContext},
    multiaddr::{Multiaddr, ToMultiaddr},
    traits::{ProtocolMeta, ServiceProtocol},
    utils::multiaddr_to_socketaddr,
    yamux::session::SessionType,
    ProtocolId, SessionId,
};
use tokio::codec::length_delimited::LengthDelimitedCodec;

#[derive(Clone, Debug)]
pub struct NodesAddressManager {
    pub nodes_mgr_client: NodesManagerClient,
}

impl NodesAddressManager {
    pub fn new(nodes_mgr_client: NodesManagerClient) -> Self {
        NodesAddressManager { nodes_mgr_client }
    }
}

impl AddressManager for NodesAddressManager {
    fn add_new(&mut self, addr: Multiaddr) {
        let address = multiaddr_to_socketaddr(&addr).unwrap();
        let req = AddNodeReq::new(address);
        self.nodes_mgr_client.add_node(req);

        info!("[NodeDiscovery] Add node {:?} to manager", address);
    }

    fn misbehave(&mut self, _addr: Multiaddr, _ty: Misbehavior) -> i32 {
        unimplemented!()
    }

    fn get_random(&mut self, n: usize) -> Vec<Multiaddr> {
        let (tx, rx) = unbounded();

        let req = GetRandomNodesReq::new(n, tx);
        self.nodes_mgr_client.get_random_nodes(req);

        let ret = rx.recv().unwrap();

        info!(
            "[NodeDiscovery] Get random address : {:?} from nodes manager.",
            ret
        );

        ret.into_iter()
            .map(|addr| addr.to_multiaddr().unwrap())
            .collect()
    }
}

pub struct DiscoveryProtocol {
    id: usize,
    discovery: Option<Discovery<NodesAddressManager>>,
    discovery_handle: DiscoveryHandle,
    discovery_senders: FnvHashMap<SessionId, Sender<Vec<u8>>>,
}

impl ServiceProtocol for DiscoveryProtocol {
    fn init(&mut self, control: &mut ServiceContext) {
        info!("[NodeDiscovery] Protocol [discovery({})]: init", self.id);

        let discovery_task = self
            .discovery
            .take()
            .map(|discovery| {
                debug!("[NodeDiscovery] Start discovery future_task");
                discovery
                    .for_each(|()| {
                        debug!("[NodeDiscovery] Discovery.for_each()");
                        Ok(())
                    })
                    .map_err(|err| {
                        warn!("[NodeDiscovery] Discovery stream error: {:?}", err);
                    })
                    .then(|_| {
                        warn!("[NodeDiscovery] End of discovery");
                        Ok(())
                    })
            })
            .unwrap();
        control.future_task(discovery_task);
    }

    fn connected(&mut self, control: &mut ServiceContext, session: &SessionContext, _: &str) {
        info!(
            "[NodeDiscovery] Protocol [discovery] open session [{}], address: [{}], type: [{:?}]",
            session.id, session.address, session.ty
        );

        info!("[NodeDiscovery] Listen list: {:?}", control.listens());
        let direction = if session.ty == SessionType::Server {
            Direction::Inbound
        } else {
            Direction::Outbound
        };

        let (sender, receiver) = channel(8);
        self.discovery_senders.insert(session.id, sender);

        let substream = Substream::new(
            &session.address,
            direction,
            self.id,
            session.id,
            receiver,
            control.control().clone(),
            control.listens(),
        );

        match self.discovery_handle.substream_sender.try_send(substream) {
            Ok(_) => {
                debug!("[NodeDiscovery] Send substream success");
            }
            Err(err) => {
                warn!("[NodeDiscovery] Send substream failed: {:?}", err);
            }
        }
    }

    fn disconnected(&mut self, _control: &mut ServiceContext, session: &SessionContext) {
        self.discovery_senders.remove(&session.id);
        info!(
            "[NodeDiscovery] Protocol [discovery] close on session [{}]",
            session.id
        );
    }

    fn received(&mut self, _control: &mut ServiceContext, session: &SessionContext, data: Vec<u8>) {
        if let Some(ref mut sender) = self.discovery_senders.get_mut(&session.id) {
            if let Err(err) = sender.try_send(data) {
                if err.is_full() {
                    warn!("[NodeDiscovery] Channel is full");
                } else if err.is_disconnected() {
                    warn!("[NodeDiscovery] Channel is disconnected");
                } else {
                    warn!("[NodeDiscovery] Other channel error {:?}", err);
                }
            }
        }
    }
}

pub struct DiscoveryProtocolMeta {
    pub id: ProtocolId,
    pub addr_mgr: NodesAddressManager,
}

impl DiscoveryProtocolMeta {
    pub fn new(id: ProtocolId, addr_mgr: NodesAddressManager) -> Self {
        DiscoveryProtocolMeta { id, addr_mgr }
    }
}

impl ProtocolMeta<LengthDelimitedCodec> for DiscoveryProtocolMeta {
    fn id(&self) -> ProtocolId {
        self.id
    }

    fn name(&self) -> String {
        "/cita/discovery".to_owned()
    }

    fn support_versions(&self) -> Vec<String> {
        vec!["0.0.1".to_owned()]
    }

    fn codec(&self) -> LengthDelimitedCodec {
        LengthDelimitedCodec::new()
    }

    fn service_handle(&self) -> Option<Box<dyn ServiceProtocol + Send + 'static>> {
        let discovery = Discovery::new(self.addr_mgr.clone());
        let discovery_handle = discovery.handle();

        let handle = Box::new(DiscoveryProtocol {
            id: self.id,
            discovery: Some(discovery),
            discovery_handle,
            discovery_senders: FnvHashMap::default(),
        });

        Some(handle)
    }
}
