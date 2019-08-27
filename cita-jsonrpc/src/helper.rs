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

use futures::sync::oneshot;
use jsonrpc_types::rpc_request::RequestInfo;
use jsonrpc_types::rpc_response::Output;
use libproto::request::Request as ProtoRequest;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::channel::Sender;
use std::collections::HashMap;
use std::sync::Arc;
use util::Mutex;
use ws;

pub enum TransferType {
    /// http output sender
    HTTP((RequestInfo, oneshot::Sender<Output>)),
    /// websocket output sender
    WEBSOCKET((RequestInfo, ws::Sender)),
}

pub type RpcMap = Arc<Mutex<HashMap<Vec<u8>, TransferType>>>;
pub type ReqSender = Mutex<Sender<(String, ProtoRequest)>>;

pub fn select_topic(method: &str) -> String {
    match method {
        "peerCount" => routing_key!(Jsonrpc >> RequestNet).into(),
        "peersInfo" => routing_key!(Jsonrpc >> RequestPeersInfo).into(),
        "sendRawTransaction" | "sendTransaction" => routing_key!(Jsonrpc >> RequestNewTx).into(),
        "getVersion" => routing_key!(Jsonrpc >> RequestRpc).into(),
        _ => routing_key!(Jsonrpc >> Request).into(),
    }
}

#[cfg(test)]
mod test {
    use super::select_topic;

    #[test]
    fn test_get_topic() {
        assert_eq!(select_topic("peerCount"), "jsonrpc.request_net".to_string());
        assert_eq!(
            select_topic("sendTransaction"),
            "jsonrpc.request_new_tx".to_string()
        );
        assert_eq!(select_topic("blockNumber"), "jsonrpc.request".to_string());
        assert_eq!(
            select_topic("getBlockByNumber"),
            "jsonrpc.request".to_string()
        );
        assert_eq!(select_topic("error"), "jsonrpc.request".to_string());
    }
}
