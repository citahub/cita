// Copyright Cryptape Technologies LLC.
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

use crate::node_manager::{AddNodeReq, GetRandomNodesReq, NodeSource, NodesManagerClient};
use pubsub::channel::unbounded;
use tentacle::{
    builder::MetaBuilder,
    multiaddr::Multiaddr,
    service::{ProtocolHandle, ProtocolMeta},
    utils::{multiaddr_to_socketaddr, socketaddr_to_multiaddr},
    ProtocolId, SessionId,
};
use tentacle_discovery::{AddressManager, Discovery, DiscoveryProtocol, MisbehaveResult, Misbehavior};

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
        let req = AddNodeReq::new(address, NodeSource::FromDiscovery);
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

        info!("[NodeDiscovery] Get random address : {:?} from nodes manager.", ret);

        ret.into_iter().map(socketaddr_to_multiaddr).collect()
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
