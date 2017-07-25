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
