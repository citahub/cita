use futures::sync::oneshot;
use jsonrpc_types::{Call, Error, Id};
use jsonrpc_types::request::Version;
use jsonrpc_types::response::Output;
use libproto::request as reqlib;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc;
use util::Mutex;
use ws;

pub enum TransferType {
    HTTP((ReqInfo, oneshot::Sender<Output>)),
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
        "jsonrpc.new_tx"
    } else if method.starts_with("cita") || method.starts_with("eth") {
        "jsonrpc.request"
    } else if method.starts_with("net_") {
        "jsonrpc.net"
    } else {
        "jsonrpc"
    }.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_topic() {
        assert_eq!(select_topic("net_work"), "jsonrpc.net".to_string());
        assert_eq!(select_topic("cita_send"), "jsonrpc.new_tx".to_string());
        assert_eq!(select_topic("cita"), "jsonrpc.request".to_string());
        assert_eq!(select_topic("eth"), "jsonrpc.request".to_string());
        assert_eq!(select_topic("123"), "jsonrpc".to_string());
    }
}
