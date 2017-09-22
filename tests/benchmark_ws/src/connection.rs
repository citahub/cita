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

use config::Param;
use jsonrpc_types::response::{Output, RusultBody};
use libproto::TxResponse;
use rpc_method::{Client, RpcMethod};
use serde_json;
use std::str::FromStr;
use time;
use util::U256;
use ws::{Builder, Settings, Sender, CloseCode, Handler, Message, Handshake, Result};

pub struct Connection {
    pub out: Sender,
    pub count: usize, //发送的个数.
    pub time: u64, //每次的时间
    pub total: u64, //总时间
    pub success_count: usize, //成功的个数
    pub failure_count: usize, //失败的个数
    pub param: Param, //参数
    pub height: u64,
}

impl Connection {
    pub fn genrate_tx(&self) -> String {
        let client = Client::new();
        client.create_contract_data(self.param.codes[0].clone(), "".to_string(), self.height)
    }
}

impl Handler for Connection {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send(Client::new().get_data_by_method(RpcMethod::Height))?;
        Ok(self.time = time::precise_time_ns())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let text = msg.as_text().unwrap();
        trace!("from server data = {:?}", text);
        let out: Output = serde_json::from_str(text).unwrap();
        match out {
            Output::Success(success) => {
                self.success_count += 1;
                if self.height == 0 {
                    match success.result {
                        RusultBody::BlockNumber(height) => {
                            let height = format!("{}", height);
                            info!("curent height = {:?}", height);
                            self.height = u64::from_str(height.as_str()).unwrap();
                            for i in 0..self.param.tx_num {
                                info!("i = {:?}", i);
                                self.out.send(self.genrate_tx())?;
                            }
                            info!("curent height = {:?}", height);
                        }
                        _ => {
                            error!("error info!");
                        }
                    }
                }
            }
            Output::Failure(failure) => {
                self.failure_count += 1;
            }
        };
        self.count += 1;
        info!("self.count = {}", self.count);
        if self.count > self.param.tx_num {
            self.out.close(CloseCode::Normal)?;
        }
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        debug!("Connection closing due to ({:?}) {}", code, reason);
        println!("count = {} , success_count = {}, failure_count = {}", self.count, self.success_count, self.failure_count);
        info!("count = {} , success_count = {}, failure_count = {}", self.count, self.success_count, self.failure_count);
    }
}
