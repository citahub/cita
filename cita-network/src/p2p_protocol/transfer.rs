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

use crate::cita_protocol::network_message_to_pubsub_message;
use crate::network::{NetworkClient, RemoteMessage};
use crate::node_manager::{AddConnectedNodeReq, InitMsg, NetworkInitReq, NodesManagerClient, RetransNetMsgReq};
use bytes::BytesMut;
use cita_types::Address;
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
    self_address: Address,
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

        info!("[Transfer] Connected sessions: {:?}", self.connected_session_ids);
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

        if let Some(mut info) = network_message_to_pubsub_message(&mut data) {
            if info.key.eq(&"network.init".to_string()) {
                let msg = InitMsg::from(info.data);
                let req = AddConnectedNodeReq::new(env.session.id, env.session.ty, msg);
                self.nodes_mgr_client.add_connected_node(req);
                return;
            }

            if info.addr == self.self_address {
                debug!("[Transfer] Recieve myself {:?} message", info.addr);
                return;
            }

            let sid = env.session.id;
            let mut msg = ProtoMessage::try_from(&info.data).unwrap();
            msg.set_origin(sid.value() as u32);
            self.network_client
                .handle_remote_message(RemoteMessage::new(info.key.clone(), msg.try_into().unwrap()));

            // Now only consensus need be retransfered
            if info.ttl > 0 {
                info.ttl -= 1;
                let req = RetransNetMsgReq::new(info, sid);
                self.nodes_mgr_client.retrans_net_msg(req);
            }
        } else {
            warn!("[Transfer] Cannot convert network message to pubsub message!");
        }
    }
}

pub fn create_transfer_meta(
    network_client: NetworkClient,
    nodes_mgr_client: NodesManagerClient,
    self_address: Address,
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
                self_address,
            });
            ProtocolHandle::Callback(handle)
        })
        .name(|_| "/cita/transfer".to_owned())
        .support_versions(vec!["0.0.2".to_owned()])
        .build()
}
