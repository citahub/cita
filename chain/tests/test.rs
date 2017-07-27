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

extern crate libproto;
extern crate protobuf;
extern crate pubsub;
extern crate amqp;
extern crate dotenv;

use libproto::*;
use protobuf::Message;
use pubsub::PubSub;
use amqp::{Consumer, Channel, protocol, Basic};
use libproto::request as reqlib;

#[derive(Default)]
pub struct MyHandler {}

impl Consumer for MyHandler {
    fn handle_delivery(&mut self,
                       channel: &mut Channel,
                       deliver: protocol::basic::Deliver,
                       _: protocol::basic::BasicProperties,
                       _: Vec<u8>) {
        let _ = channel.basic_ack(deliver.delivery_tag, false);
        std::process::exit(0);
    }
}

#[test]
fn get_block_number() {
    dotenv::dotenv().ok();
    let mut pubsub = PubSub::new();
    pubsub.start_sub("jsonrpc", vec!["*.rpc"], MyHandler::default());
    let mut _pub = pubsub.get_pub();
    let request_id = vec![1, 0, 2];
    let mut request = reqlib::Request::new();
    request.set_request_id(request_id);
    request.set_block_number(true);
    let msg: communication::Message = request.into();
    _pub.publish("jsonrpc.request", msg.write_to_bytes().unwrap());
}
