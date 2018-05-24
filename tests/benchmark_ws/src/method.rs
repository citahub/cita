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

use jsonrpc_types::method::method;
use jsonrpc_types::request::Version;
use jsonrpc_types::response::ResultBody;
use jsonrpc_types::{Call, Error, Id, Params, RpcRequest};
use serde_json;
use uuid::Uuid;

pub trait Method<F> {
    fn request(&mut self, method: &str, params: Params, call_back: F);
    fn send_transaction(&mut self, params: Params, call_back: F);
    fn get_current_height(&mut self, params: Params, call_back: F);
    fn get_block_by_number(&mut self, params: Params, call_back: F);
    fn get_receipt(&mut self, params: Params, call_back: F);
}

impl<T: Work, F> Method<F> for T
where
    F: 'static,
{
    fn request(&mut self, method: &str, params: Params, call_back: F) {
        let request_id = Uuid::new_v4().to_string();
        let rpc = Call {
            jsonrpc: Some(Version::V2),
            method: method.to_string(),
            id: Id::Str(request_id.clone()),
            params: Some(params),
        };
        let _ = serde_json::to_string(&rpc).map(|data| {
            self.send(data);
            self.insert(request_id, call_back);
        });
    }

    fn send_transaction(&mut self, params: Params, call_back: F) {
        self.request(method::CITA_SEND_TRANSACTION, params, call_back)
    }

    fn get_current_height(&mut self, params: Params, call_back: F) {
        self.request(method::CITA_BLOCK_BUMBER, params, call_back)
    }

    fn get_block_by_number(&mut self, params: Params, call_back: F) {
        self.request(method::CITA_GET_BLOCK_BY_NUMBER, params, call_back)
    }

    fn get_receipt(&mut self, params: Params, call_back: F) {
        self.request(method::ETH_GET_TRANSACTION_RECEIPT, params, call_back)
    }
}

pub trait Work {
    fn insert<F: 'static>(&mut self, id: String, call_back: F) -> bool;
    fn is_exist(&self, id: &String) -> bool;
    fn exce(&mut self, id: &String, ret: Result<ResultBody, Error>);
    fn send(&mut self, data: String);
}

// impl Work for Worker {
//     fn insert<F: 'static + FnMut(Result<ResultBody, Error>)>(
//         &mut self,
//         id: String,
//         call_back: F,
//     ) -> bool {
//         self.requests.insert(id, Box::new(call_back)).is_some()
//     }

//     fn is_exist(&self, id: &String) -> bool {
//         self.requests.contains_key(id)
//     }

//     fn exce(&mut self, id: &String, ret: Result<ResultBody, Error>) {
//         if let Some(mut call_back) = self.requests.remove(id) {
//             call_back(ret);
//         }
//     }
// }
