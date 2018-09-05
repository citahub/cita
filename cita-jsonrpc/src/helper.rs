// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use futures::sync::oneshot;
use jsonrpc_types::request::RequestInfo;
use jsonrpc_types::response::Output;
use libproto::request::Request as ProtoRequest;
use libproto::router::{MsgType, RoutingKey, SubModules};
use std::collections::HashMap;
use std::sync::mpsc;
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
pub type ReqSender = Mutex<mpsc::Sender<(String, ProtoRequest)>>;

pub fn select_topic(method: &str) -> String {
    match method {
        "peerCount" => routing_key!(Jsonrpc >> RequestNet).into(),
        "sendRawTransaction" | "sendTransaction" => routing_key!(Jsonrpc >> RequestNewTx).into(),
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
