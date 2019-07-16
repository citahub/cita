// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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

use libproto::auth::{BalanceVerifyReq, BalanceVerifyTransaction};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::Message;
use libproto::TryInto;
use pubsub::channel::{Receiver, Sender};
use std::convert::Into;
use std::thread;
use std::time::Duration;
use util::instrument::{unix_now, AsMillis};

pub struct BalanceVerifyReqPublisher {
    batch_size: usize,
    timeout: u64,
    check_duration: u32,
    last_timestamp: u64,
    request_buffer: Vec<BalanceVerifyTransaction>,
    rx_banance_verify_tx: Receiver<BalanceVerifyTransaction>,
    tx_pub: Sender<(String, Vec<u8>)>,
}

impl BalanceVerifyReqPublisher {
    pub fn new(
        batch_size: usize,
        timeout: u64,
        rx_banance_verify_tx: Receiver<BalanceVerifyTransaction>,
        tx_pub: Sender<(String, Vec<u8>)>,
    ) -> Self {
        BalanceVerifyReqPublisher {
            batch_size,
            timeout,
            check_duration: 5,
            last_timestamp: AsMillis::as_millis(&unix_now()),
            request_buffer: Vec::new(),
            rx_banance_verify_tx,
            tx_pub,
        }
    }

    pub fn run(&mut self) {
        loop {
            let ret = self.rx_banance_verify_tx.try_recv();
            if ret.is_ok() {
                let verify_req = ret.unwrap();
                self.request_buffer.push(verify_req);
                if self.request_buffer.len() > self.batch_size {
                    self.batch_publish();
                }
            } else {
                thread::sleep(Duration::new(0, self.check_duration * 1_000_000));
                let now = AsMillis::as_millis(&unix_now());
                if now.saturating_sub(self.last_timestamp) > self.timeout
                    && !self.request_buffer.is_empty()
                {
                    self.batch_publish();
                }
            }
        }
    }

    fn batch_publish(&mut self) {
        let mut batch_verify_tx_request = BalanceVerifyReq::new();
        batch_verify_tx_request.set_bv_txs(self.request_buffer.clone().into());

        let msg: Message = batch_verify_tx_request.into();
        self.tx_pub
            .send((
                routing_key!(Auth >> BalanceVerifyReq).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();

        self.last_timestamp = AsMillis::as_millis(&unix_now());
        self.request_buffer.clear();
    }
}
