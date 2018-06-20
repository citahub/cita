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
