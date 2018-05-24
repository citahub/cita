use futures::sync::oneshot;
use jsonrpc_types::request::Version;
use jsonrpc_types::response::Output;
use jsonrpc_types::{Call, Error, Id};
use libproto::request as reqlib;
use libproto::router::{MsgType, RoutingKey, SubModules};
use serde_json;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Arc;
use util::Mutex;
use ws;

pub enum TransferType {
    /// http output sender
    HTTP((ReqInfo, oneshot::Sender<Output>)),
    /// websocket output sender
    WEBSOCKET((ReqInfo, ws::Sender)),
}

#[derive(Debug, Clone)]
pub struct ReqInfo {
    pub jsonrpc: Option<Version>,
    pub id: Id,
}

pub type RpcMap = Arc<Mutex<HashMap<Vec<u8>, TransferType>>>;
pub type ReqSender = Mutex<mpsc::Sender<(String, reqlib::Request)>>;

impl ReqInfo {
    pub fn new(jsonrpc: Option<Version>, id: Id) -> ReqInfo {
        ReqInfo {
            jsonrpc: jsonrpc,
            id: id,
        }
    }
}

pub fn encode_request(body: &str) -> Result<Call, Error> {
    let rpc: Result<Call, serde_json::Error> = serde_json::from_str(body);
    match rpc {
        Err(_err_msg) => Err(Error::from(_err_msg)),
        Ok(rpc) => Ok(rpc),
    }
}

pub fn select_topic(method: &str) -> String {
    if method.starts_with("cita_send") {
        routing_key!(Jsonrpc >> RequestNewTx).into()
    } else if method.starts_with("cita") || method.starts_with("eth") {
        routing_key!(Jsonrpc >> Request).into()
    } else if method.starts_with("net_") {
        routing_key!(Jsonrpc >> RequestNet).into()
    } else {
        "jsonrpc".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_topic() {
        assert_eq!(select_topic("net_work"), "jsonrpc.request_net".to_string());
        assert_eq!(
            select_topic("cita_send"),
            "jsonrpc.request_new_tx".to_string()
        );
        assert_eq!(select_topic("cita"), "jsonrpc.request".to_string());
        assert_eq!(select_topic("eth"), "jsonrpc.request".to_string());
        assert_eq!(select_topic("123"), "jsonrpc".to_string());
    }
}
