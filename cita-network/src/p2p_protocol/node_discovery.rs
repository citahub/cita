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

use crate::node_manager::{AddNodeReq, GetRandomNodesReq, NodeSource, NodesManagerClient};
use logger::{info, warn};
use pubsub::channel::unbounded;
use tentacle::{
    builder::MetaBuilder,
    multiaddr::{Multiaddr, ToMultiaddr},
    service::{ProtocolHandle, ProtocolMeta},
    utils::multiaddr_to_socketaddr,
    ProtocolId, SessionId,
};
use tentacle_discovery::{
    AddressManager, Discovery, DiscoveryProtocol, MisbehaveResult, Misbehavior,
};

pub const DISCOVERY_PROTOCOL_ID: ProtocolId = ProtocolId::new(0);
pub const DISCOVERY_TIMEOUT_SECS: u64 = 150;

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
    fn add_new_addr(&mut self, _session_id: SessionId, addr: Multiaddr) {
        let address = multiaddr_to_socketaddr(&addr).unwrap();
        let req = AddNodeReq::new(address, NodeSource::NotConfig);
        self.nodes_mgr_client.add_node(req);

        info!("[NodeDiscovery] Add node {:?} to manager", address);
    }

    fn add_new_addrs(&mut self, session_id: SessionId, addrs: Vec<Multiaddr>) {
        for addr in addrs.into_iter() {
            self.add_new_addr(session_id, addr)
        }
    }

    fn misbehave(&mut self, _session_id: SessionId, _kind: Misbehavior) -> MisbehaveResult {
        warn!("[NodeDiscovery] Has not handled misbehave in this version!");
        MisbehaveResult::Disconnect
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

pub fn create_discovery_meta(nodes_mgr_client: NodesManagerClient) -> ProtocolMeta {
    let addr_mgr = NodesAddressManager::new(nodes_mgr_client);
    let timeout = ::std::time::Duration::new(DISCOVERY_TIMEOUT_SECS, 0);
    MetaBuilder::default()
        .id(DISCOVERY_PROTOCOL_ID)
        .service_handle(move || {
            let discovery = Discovery::new(addr_mgr.clone(), Some(timeout));
            ProtocolHandle::Callback(Box::new(DiscoveryProtocol::new(discovery)))
        })
        .name(|_| "/cita/discovery".to_owned())
        .support_versions(vec!["0.0.2".to_owned()])
        .build()
}
