// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

use jsonrpc_types::error::Error;
use jsonrpc_types::request::RpcRequest;
use serde_json;
use std::result;


pub type RpcResult<T> = result::Result<T, Error>;

pub trait BaseHandler {
    fn select_topic(method: &String) -> String {
        let topic = if method.starts_with("cita_send") {
                        "jsonrpc.new_tx"
                    } else if method.starts_with("cita") || method.starts_with("eth") {
                        "jsonrpc.request"
                    } else if method.starts_with("net_") {
                        "jsonrpc.net"
                    } else {
                        "jsonrpc"
                    }
                    .to_string();
        topic
    }

    fn into_json(body: String) -> Result<RpcRequest, Error> {
        let rpc: Result<RpcRequest, serde_json::Error> = serde_json::from_str(&body);
        match rpc {
            Err(_err_msg) => Err(Error::parse_error()),
            Ok(rpc) => Ok(rpc),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransferType {
    ALL,
    HTTP,
    WEBSOCKET,
}


impl Default for TransferType {
    fn default() -> TransferType {
        TransferType::ALL
    }
}




#[cfg(test)]
mod test {
    use super::BaseHandler;
    struct Handler {}
    impl BaseHandler for Handler {}

    #[test]
    fn test_get_topic() {
        assert_eq!(Handler::select_topic(&"net_work".to_string()), "jsonrpc.net".to_string());
        assert_eq!(Handler::select_topic(&"cita_send".to_string()), "jsonrpc.new_tx".to_string());
        assert_eq!(Handler::select_topic(&"cita".to_string()), "jsonrpc.request".to_string());
        assert_eq!(Handler::select_topic(&"eth".to_string()), "jsonrpc.request".to_string());
        assert_eq!(Handler::select_topic(&"123".to_string()), "jsonrpc".to_string());
    }

}
