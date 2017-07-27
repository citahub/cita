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

#![feature(test)]
extern crate amqp;
extern crate test;
extern crate pubsub;

use amqp::{Consumer, Channel, Basic, protocol};
use test::Bencher;
use pubsub::PubSub;

struct MyHandler {
    count: u32,
}

impl MyHandler {
    pub fn new() -> Self {
        MyHandler { count: 0 }
    }
}

impl Consumer for MyHandler {
    fn handle_delivery(&mut self,
                       channel: &mut Channel,
                       deliver: protocol::basic::Deliver,
                       _: protocol::basic::BasicProperties,
                       body: Vec<u8>) {
        self.count += 1;
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}

#[bench]
fn publish(b: &mut Bencher) {
    let mut pubsub = PubSub::new();
    let myhandler = MyHandler::new();
    pubsub.start_sub("test_x_queue", vec!["bench.*"], myhandler);

    let mut _pub = pubsub.get_pub();
    let input_data = "hello world!".to_string();

    b.iter(|| { _pub.publish("bench.a", input_data.trim_right().to_string().into_bytes()); });
}
