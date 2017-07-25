extern crate pubsub;
extern crate amqp;

use pubsub::{PubSub, Pub};
use std::time::{Duration, SystemTime};
use std::thread;
use std::env;
use std::process::exit;
use amqp::{Consumer, Channel, protocol, Basic};

pub struct MyHandler {
    push: Pub,
    start: SystemTime,
    max: u64,
    count: u64,
}

impl MyHandler {
    pub fn new(ps: Pub, max: u64) -> Self {
        MyHandler {
            push: ps,
            start: SystemTime::now(),
            max: max,
            count: 0,
        }
    }
}

impl Consumer for MyHandler {
    fn handle_delivery(&mut self,
                       channel: &mut Channel,
                       deliver: protocol::basic::Deliver,
                       _: protocol::basic::BasicProperties,
                       _: Vec<u8>) {
        //info!{"delivery id {:?}, payload {:?}", deliver.routing_key, body};
        //dispatch(&self.tx, key_to_id(diliver.routing_key.as_str()), body);
        if self.count == 0 {
            self.start = SystemTime::now();
        }
        self.count += 1;
        self.push.publish("response", vec![0, 1]);
        let _ = channel.basic_ack(deliver.delivery_tag, false);
        if self.count == self.max {
            let sys_time = SystemTime::now();
            let diff = sys_time.duration_since(self.start)
                .expect("SystemTime::duration_since failed");
            println!{"count {} timer diff: {:?}", self.count, diff};
            //wait for complete the response message
            thread::sleep(Duration::new(2,0));
            exit(0);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("need only one argument : max msg count!");
        return;
    }
    let max = args[1].parse::<u64>().unwrap();
    let mut pubsub = PubSub::new();
    let mut _pub = pubsub.get_pub();
    pubsub.start_sub("response", vec!["request"], MyHandler::new(_pub, max));
    loop {
        thread::sleep(Duration::new(10,0));
    }
}
