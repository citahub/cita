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

use client::{Client, RpcMethod};
use config::Param;
use connection::*;
use jsonrpc_types::response::{Output, ResultBody};
use rand::{thread_rng, ThreadRng, Rng};
use serde_json;
use std::collections::VecDeque;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use time;
use util::RwLock;
use ws::{Sender, CloseCode, Message};
type Number = u64;
type Timetamp = u64;
type TxCount = usize;


const CHECK_HEIGHT: i64 = 100;
const BREAK_BLOCK: i64 = 20;

pub struct Worker {
    ws_senders: Arc<RwLock<Vec<Sender>>>,
    param: Param,
    start_time: u64,
    current_height: u64,
    rand: ThreadRng,
}


impl Worker {
    pub fn new(ws_senders: Arc<RwLock<Vec<Sender>>>, param: Param) -> Self {
        Worker {
            ws_senders: ws_senders,
            param: param,
            start_time: 0,
            current_height: 0,
            rand: thread_rng(),
        }
    }

    pub fn genrate_tx(&self) -> String {
        let client = Client::new();
        let height = self.current_height;
        client.create_contract_data(self.param.tx_param.codes[0].clone(), "".to_string(), height)
    }

    fn rand_send(&mut self, data: String) {
        let index = self.rand.gen_range(0, self.ws_senders.read().len());
        let _ = self.ws_senders.read()[index].send(data);
    }

    pub fn bench_peer_count(&mut self) {
        for i in 0..self.param.number {
            let peer = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"net_peerCount\",\"params\":[],\"id\":{:?}}}", i);
            self.rand_send(peer);
        }
        self.start_time = time::precise_time_ns();
    }

    pub fn close_all(&self) {
        while let Some(sender) = self.ws_senders.write().pop() {
            let _ = sender.close(CloseCode::Normal);
        }
    }

    pub fn heart_beat_height(&mut self) {
        let sender = self.ws_senders.read()[0].clone();
        thread::spawn(move || loop {
                          thread::sleep(Duration::new(1, 0));
                          let client = Client::new();
                          let _ = sender.send(client.get_data_by_method(RpcMethod::Height));
                      });
    }

    pub fn bench_tx(&mut self) {
        let mut genrate_txs = vec![];
        for _ in 0..self.param.number {
            genrate_txs.push(self.genrate_tx())
        }
        while let Some(value) = genrate_txs.pop() {
            self.rand_send(value);
        }
        self.start_time = time::precise_time_ns();
    }

    fn full_block(&mut self) {
        let client = Client::new();
        let height = self.current_height;
        self.rand_send(client.get_data_by_method(RpcMethod::GetBlockbyheiht(height)));
    }


    pub fn recive(&mut self, rx: mpsc::Receiver<Message>) {
        let mut check_height_break = CHECK_HEIGHT;
        let mut check_block_break = BREAK_BLOCK;
        let is_bench_tx = self.param.tx_param.enable;
        let is_bench_peer = self.param.peer_param.enable;
        let mut is_first = true;
        let mut success_count = 0;
        let mut failure_count = 0;
        let mut actual_tx_count = 0;
        let mut block_info: VecDeque<(Number, Timetamp, TxCount)> = VecDeque::new();
        loop {
            let _ = rx.recv().map(|msg| {
                if success_count + failure_count == self.param.number {
                    println!("number = {}", self.param.number);
                }
                let text = msg.as_text().unwrap();
                //println!("from server data = {:?}", text);
                serde_json::from_str(text)
                    .map(|out| match out {
                             Output::Success(success) => {
                                 match success.result {
                                     ResultBody::BlockNumber(number) => {
                                         let number = format!("{}", number);
                                         //println!("number = {:?}", number);
                                         let number = u64::from_str(number.as_str()).unwrap();

                                         if is_bench_tx {
                                             println!("current height = {}, check_height_break = {}", number, check_height_break);
                                             let mut is_send = true;
                                             if self.current_height < number {
                                                 self.current_height = number;
                                                 check_height_break = CHECK_HEIGHT;
                                             } else {
                                                 is_send = false;
                                                 check_height_break -= 1;
                                             }
                                             if check_height_break <= 0 {
                                                 self.close_all();
                                             }

                                             if is_first {
                                                 if self.current_height > 10 {
                                                     is_first = false;
                                                     self.bench_tx();
                                                 }
                                             } else {
                                                 if is_send {
                                                     self.full_block();
                                                 }
                                             }
                                         }

                                         if is_bench_peer {
                                             success_count += 1;
                                             if success_count + failure_count >= self.param.number {
                                                 let secs = (time::precise_time_ns() - self.start_time) / 1000000;
                                                 let tps = if secs > 0 { (self.param.number * 1000) as u64 / secs } else { self.param.number as u64 };
                                                 println!("send = {}, tps = {} ,recice respone cast time = {:?} ms , success_count = {:?}, failure_count = {:?}", self.param.number, tps, secs, success_count, failure_count);
                                                 self.close_all();
                                             }
                                         }
                                     }

                                     ResultBody::FullBlock(block) => {
                                         let body = block.body;
                                         let txs_len = body.transactions.len();
                                         let time_stamp = block.header.timestamp;
                                         let block_height = u64::from_str(format!("{}", block.header.number).as_str()).unwrap();
                                         actual_tx_count += txs_len;
                                         println!("block_height = {:?}, check_block_break = {:?} ", block_height, check_block_break);
                                         if txs_len == 0 {
                                             check_block_break -= 1;
                                         } else {
                                             block_info.push_back((block_height, time_stamp, txs_len));
                                         }

                                         if check_block_break <= 0 {
                                             println!("blocks infomation: {:?}", block_info);
                                             let mut first = (0, 0, 0);
                                             let mut last = (0, 0, 0);
                                             if let Some(f) = block_info.pop_front() {
                                                 first = f;

                                             }
                                             if let Some(l) = block_info.pop_back() {
                                                 last = l;
                                             }
                                             let secs = last.1 - first.1;
                                             let tps = if secs > 0 { (actual_tx_count * 1000) as u64 / secs } else { actual_tx_count as u64 };
                                             println!("total_count: {}, start height: {},  use time: {} ms, tps: {}", actual_tx_count, first.0, secs, tps);
                                             self.close_all();
                                         }
                                     }

                                     ResultBody::TxResponse(_tx_res) => {
                                         success_count += 1;
                                         if success_count + failure_count >= self.param.number {
                                             let secs = (time::precise_time_ns() - self.start_time) / 1000000;
                                             let tps = if secs > 0 { (self.param.number * 1000) as u64 / secs } else { self.param.number as u64 };
                                             println!("send = {}, tps = {} ,recice respone cast time = {:?} ms , success_count = {:?}, failure_count = {:?}", self.param.number, tps, secs, success_count, failure_count);
                                         }
                                     }

                                     _ => {
                                         failure_count += 1;
                                         error!("error info!");
                                     }
                                 }
                             }

                             Output::Failure(failure) => {
                                 println!("failure {:?}", failure);
                                 failure_count += 1;
                             }
                         })
                    .map_err(|err| {
                                 println!("{}", err);
                                 failure_count += 1;
                             })
            });
        }
    }
}


mod test {
    use rand::Rng;
    use rand::thread_rng;
    #[test]
    fn test_gen_range() {
        let mut r = thread_rng();
        for _ in 0..1000 {
            let a = r.gen_range(10, 42);
            println!("a = {:?}", a);
            assert!(a >= 10 && a < 42);
            assert_eq!(r.gen_range(0, 1), 0);
            assert_eq!(r.gen_range(3_000_000, 3_000_001), 3_000_000);
        }
    }
}
