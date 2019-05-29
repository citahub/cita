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

use crate::cita_protocol::network_message_to_pubsub_message;
use crate::network::{NetworkClient, RemoteMessage};
use crate::node_manager::{AddConnectedNodeReq, InitMsg, NetworkInitReq, NodesManagerClient};
use bytes::BytesMut;
use libproto::{Message as ProtoMessage, TryFrom, TryInto};
use tentacle::{
    builder::MetaBuilder,
    context::{ProtocolContext, ProtocolContextMutRef},
    service::{ProtocolHandle, ProtocolMeta},
    traits::ServiceProtocol,
    ProtocolId, SessionId,
};
use tokio::codec::length_delimited::LengthDelimitedCodec;

// Quota (1 byte) = 200,
// Max 20 block in one transfer.
// 512M can support BQL set to 2 ** 32 - 1
pub const MAX_FRAME_LENGTH: usize = 512 * 1024 * 1204;
pub const TRANSFER_PROTOCOL_ID: ProtocolId = ProtocolId::new(1);

struct TransferProtocol {
    proto_id: ProtocolId,
    connected_session_ids: Vec<SessionId>,
    network_client: NetworkClient,
    nodes_mgr_client: NodesManagerClient,
}

impl ServiceProtocol for TransferProtocol {
    fn init(&mut self, _control: &mut ProtocolContext) {}

    fn connected(&mut self, control: ProtocolContextMutRef, version: &str) {
        info!(
            "[Transfer] Connected proto id [{}] open on session [{}], address: [{}], type: [{:?}], version: {}",
            self.proto_id, control.session.id, control.session.address, control.session.ty, version
        );
        self.connected_session_ids.push(control.session.id);

        let req = NetworkInitReq::new(control.session.id);
        self.nodes_mgr_client.network_init(req);

        info!(
            "[Transfer] Connected sessions: {:?}",
            self.connected_session_ids
        );
    }

    fn disconnected(&mut self, control: ProtocolContextMutRef) {
        let new_list = self
            .connected_session_ids
            .iter()
            .filter(|&id| id != &control.session.id)
            .cloned()
            .collect();
        self.connected_session_ids = new_list;

        info!(
            "[Transfer] Disconnected proto id [{}] close on session [{}]",
            self.proto_id, control.session.id
        );
    }

    fn received(&mut self, env: ProtocolContextMutRef, data: bytes::Bytes) {
        let mut data = BytesMut::from(data);

        if let Some((key, message)) = network_message_to_pubsub_message(&mut data) {
            if key.eq(&"network.init".to_string()) {
                let msg = InitMsg::from(message);
                let req = AddConnectedNodeReq::new(env.session.id, env.session.ty, msg);
                self.nodes_mgr_client.add_connected_node(req);
                return;
            }

            let mut msg = ProtoMessage::try_from(&message).unwrap();
            msg.set_origin(env.session.id.value() as u32);
            self.network_client
                .handle_remote_message(RemoteMessage::new(key, msg.try_into().unwrap()));
        } else {
            warn!("[Transfer] Cannot convert network message to pubsub message!");
        }
    }
}

pub fn create_transfer_meta(
    network_client: NetworkClient,
    nodes_mgr_client: NodesManagerClient,
) -> ProtocolMeta {
    MetaBuilder::default()
        .id(TRANSFER_PROTOCOL_ID)
        .codec(|| {
            let mut lcodec = LengthDelimitedCodec::new();
            lcodec.set_max_frame_length(MAX_FRAME_LENGTH);
            Box::new(lcodec)
        })
        .service_handle(move || {
            let handle = Box::new(TransferProtocol {
                proto_id: TRANSFER_PROTOCOL_ID,
                connected_session_ids: Vec::default(),
                network_client: network_client.clone(),
                nodes_mgr_client: nodes_mgr_client.clone(),
            });
            ProtocolHandle::Callback(handle)
        })
        .name(|_| "/cita/transfer".to_owned())
        .support_versions(vec!["0.0.2".to_owned()])
        .build()
}
