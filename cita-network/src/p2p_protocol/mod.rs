// Copyright Rivtower Technologies LLC.
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

use crate::node_manager::{
    AddRepeatedNodeReq, ConnectedSelfReq, DelConnectedNodeReq, DialedErrorReq, NodesManagerClient,
    PendingConnectedNodeReq,
};
use tentacle::{
    context::ServiceContext,
    error,
    service::{ServiceError, ServiceEvent},
    traits::ServiceHandle,
    utils::multiaddr_to_socketaddr,
};

pub mod node_discovery;
pub mod transfer;

// This handle will be shared with all protocol
pub struct SHandle {
    nodes_mgr_client: NodesManagerClient,
}

impl SHandle {
    pub fn new(nodes_mgr_client: NodesManagerClient) -> Self {
        SHandle { nodes_mgr_client }
    }
}

impl ServiceHandle for SHandle {
    fn handle_error(&mut self, _env: &mut ServiceContext, error: ServiceError) {
        match error {
            ServiceError::DialerError { address, error } => {
                let address = multiaddr_to_socketaddr(&address).unwrap();

                // If dial to a connected node, need add it to connected address list.
                match error {
                    error::Error::RepeatedConnection(session_id) => {
                        // Do not need to do something, just log it.
                        info!(
                            "[P2pProtocol] Connected to the same node : {:?}, session id: {:?}",
                            address, session_id
                        );
                        let req = AddRepeatedNodeReq::new(address, session_id);
                        self.nodes_mgr_client.add_repeated_node(req);
                    }
                    error::Error::ConnectSelf => {
                        info!("[P2pProtocol] Connected to self, address: {:?}.", address);
                        let req = ConnectedSelfReq::new(address);
                        self.nodes_mgr_client.connected_self(req);
                    }
                    _ => {
                        // FIXME: Using score for deleting a node from known nodes
                        let req = DialedErrorReq::new(address);
                        self.nodes_mgr_client.dialed_error(req);
                        warn!("[P2pProtocol] Dialed Error in {:?} : {:?}.", address, error);
                    }
                }
            }
            ServiceError::ListenError { address, error } => {
                let address = multiaddr_to_socketaddr(&address).unwrap();
                warn!(
                    "[P2pProtocol] Listen error on {:?}, error info: {:?}",
                    address, error
                );
            }
            ServiceError::ProtocolError {
                id,
                proto_id,
                error,
            } => {
                // FIXME: handle protocol error later
                warn!(
                    "[P2pProtocol] Protocol Error, stream id: {:?}, protocol id: {:?}, error: {:?}",
                    id, proto_id, error
                );
            }
            ServiceError::ProtocolSelectError {
                proto_name,
                session_context,
            } => {
                // FIXME: handle protocol select error later
                warn!(
                    "[P2pProtocol] Protocol SelectError, proto_name: {:?}, session_context: {:?}.",
                    proto_name, session_context,
                );
            }

            ServiceError::SessionTimeout { session_context } => {
                warn!(
                    "[P2pProtocol] SessionTimeout Sessionid {:?} ",
                    session_context.id
                );
                let req = DelConnectedNodeReq::new(session_context.id);
                self.nodes_mgr_client.del_connected_node(req);
            }

            ServiceError::MuxerError {
                session_context,
                error,
            } => {
                warn!(
                    "[P2pProtocol] ServiceError::MuxerError Sessionid {:?}--{:?}",
                    session_context.id, error
                );
            }

            ServiceError::ProtocolHandleError { error, proto_id } => {
                warn!(
                    "[P2pProtocol] ServiceError::ProtocolHandleError proto_id {:?}--{:?}",
                    proto_id, error
                );
            }
            ServiceError::SessionBlocked { session_context } => {
                warn!(
                    "[P2pProtocol] ServiceError::SessionBlocked session_context {:?}",
                    session_context
                );
            }
        }
    }

    fn handle_event(&mut self, _env: &mut ServiceContext, event: ServiceEvent) {
        match event {
            ServiceEvent::SessionOpen { session_context } => {
                if let Some(sock_addr) = multiaddr_to_socketaddr(&session_context.address) {
                    info!("[P2pProtocol] Service open on : {:?}, session id: {:?}, ty: {:?}, public_key: {:?}",
                          sock_addr, session_context.id, session_context.ty, session_context.remote_pubkey);
                    let req = PendingConnectedNodeReq::new(
                        session_context.id,
                        sock_addr,
                        session_context.ty,
                    );
                    self.nodes_mgr_client.pending_connected_node(req);
                } else {
                    info!(
                        "[P2pProtocol] Service open multiaddr {:?} transform failed",
                        session_context.address
                    );
                }
            }
            ServiceEvent::SessionClose { session_context } => {
                let req = DelConnectedNodeReq::new(session_context.id);
                self.nodes_mgr_client.del_connected_node(req);
            }
            ServiceEvent::ListenClose { address } => {
                warn!("ListenClose, address {:?}", address);
            }

            ServiceEvent::ListenStarted { address } => {
                info!("ListenStarted, address {:?}", address);
            }
        }
    }
}
