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

use crate::citaprotocol::network_message_to_pubsub_message;
use crate::network::{NetworkClient, RemoteMessage};
use crate::node_manager::{AddConnectedKeyReq, InitMsg, NetworkInitReq, NodesManagerClient};
use bytes::BytesMut;
use libproto::{Message as ProtoMessage, TryFrom, TryInto};
use log::{debug, info};
use tentacle::{
    context::{ServiceContext, SessionContext},
    traits::{ProtocolMeta, ServiceProtocol},
    ProtocolId, SessionId,
};
use tokio::codec::length_delimited::LengthDelimitedCodec;

pub struct TransferProtocolMeta {
    id: ProtocolId,
    network_client: NetworkClient,
    nodes_mgr_client: NodesManagerClient,
}

impl TransferProtocolMeta {
    pub fn new(
        id: ProtocolId,
        network_client: NetworkClient,
        nodes_mgr_client: NodesManagerClient,
    ) -> Self {
        TransferProtocolMeta {
            id,
            network_client,
            nodes_mgr_client,
        }
    }
}

impl ProtocolMeta<LengthDelimitedCodec> for TransferProtocolMeta {
    fn id(&self) -> ProtocolId {
        self.id
    }
    fn codec(&self) -> LengthDelimitedCodec {
        LengthDelimitedCodec::new()
    }
    fn service_handle(&self) -> Option<Box<dyn ServiceProtocol + Send + 'static>> {
        let handle = Box::new(TransferProtocol {
            proto_id: self.id,
            connected_session_ids: Vec::default(),
            network_client: self.network_client.clone(),
            nodes_mgr_client: self.nodes_mgr_client.clone(),
        });
        Some(handle)
    }
}

struct TransferProtocol {
    proto_id: ProtocolId,
    connected_session_ids: Vec<SessionId>,
    network_client: NetworkClient,
    nodes_mgr_client: NodesManagerClient,
}

impl ServiceProtocol for TransferProtocol {
    fn init(&mut self, _control: &mut ServiceContext) {}

    fn connected(
        &mut self,
        _control: &mut ServiceContext,
        session: &SessionContext,
        version: &str,
    ) {
        info!(
            "[connected] proto id [{}] open on session [{}], address: [{}], type: [{:?}], version: {}",
            self.proto_id, session.id, session.address, session.ty, version
        );
        self.connected_session_ids.push(session.id);

        let req = NetworkInitReq::new(session.id);
        self.nodes_mgr_client.network_init(req);

        info!(
            "[connected] connected sessions: {:?}",
            self.connected_session_ids
        );
    }

    fn disconnected(&mut self, _control: &mut ServiceContext, session: &SessionContext) {
        let new_list = self
            .connected_session_ids
            .iter()
            .filter(|&id| id != &session.id)
            .cloned()
            .collect();
        self.connected_session_ids = new_list;

        info!(
            "[disconnected] proto id [{}] close on session [{}]",
            self.proto_id, session.id
        );
    }

    fn received(&mut self, _env: &mut ServiceContext, session: &SessionContext, data: Vec<u8>) {
        let mut data = BytesMut::from(data);

        if let Some((key, message)) = network_message_to_pubsub_message(&mut data) {
            debug!("[received] Received network message!key: {:?}", key);
            if key.eq(&"network.init".to_string()) {
                let msg = InitMsg::from(message);
                let req = AddConnectedKeyReq::new(session.id, session.ty, msg);
                self.nodes_mgr_client.add_connected_key(req);
                return;
            }

            let mut msg = ProtoMessage::try_from(&message).unwrap();
            msg.set_origin(session.id as u32);
            self.network_client
                .handle_remote_message(RemoteMessage::new(key, msg.try_into().unwrap()));
        } else {
            debug!("[received] Cannot convert network message to pubsub message!");
        }
    }
}
