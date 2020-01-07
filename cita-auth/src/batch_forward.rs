// Copyright Rivtower Technologies LLC.
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

use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::TryInto;
use libproto::{BatchRequest, Message, Request};
use pubsub::channel::{Receiver, Sender};
use std::convert::Into;
use std::thread;
use std::time::Duration;
use util::instrument::{unix_now, AsMillis};
use uuid::Uuid;

pub struct BatchForward {
    batch_size: usize,
    timeout: u64,
    check_duration: u32,
    last_timestamp: u64,
    request_buffer: Vec<Request>,
    rx_request: Receiver<Request>,
    tx_pub: Sender<(String, Vec<u8>)>,
}

impl BatchForward {
    pub fn new(
        batch_size: usize,
        timeout: u64,
        rx_request: Receiver<Request>,
        tx_pub: Sender<(String, Vec<u8>)>,
    ) -> Self {
        BatchForward {
            batch_size,
            timeout,
            check_duration: 5,
            last_timestamp: AsMillis::as_millis(&unix_now()),
            request_buffer: Vec::new(),
            rx_request,
            tx_pub,
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(tx_req) = self.rx_request.try_recv() {
                self.request_buffer.push(tx_req);
                if self.request_buffer.len() > self.batch_size {
                    self.batch_forward();
                }
            } else {
                thread::sleep(Duration::new(0, self.check_duration * 1_000_000));
                let now = AsMillis::as_millis(&unix_now());
                if now.saturating_sub(self.last_timestamp) > self.timeout
                    && !self.request_buffer.is_empty()
                {
                    self.batch_forward();
                }
            }
        }
    }

    fn batch_forward(&mut self) {
        trace!(
            "batch_forward_tx_to_peer is going to send {} new tx to peer",
            self.request_buffer.len()
        );
        let mut batch_request = BatchRequest::new();
        batch_request.set_new_tx_requests(self.request_buffer.clone().into());

        let request_id = Uuid::new_v4().as_bytes().to_vec();
        let mut request = Request::new();
        request.set_batch_req(batch_request);
        request.set_request_id(request_id);

        let msg: Message = request.into();
        self.tx_pub
            .send((
                routing_key!(Auth >> Request).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();

        self.last_timestamp = AsMillis::as_millis(&unix_now());
        self.request_buffer.clear();
    }
}
