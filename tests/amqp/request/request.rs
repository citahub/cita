extern crate pubsub;
extern crate amqp;

use pubsub::PubSub;
use std::time::{Duration, SystemTime};
use std::thread;
use std::env;
use std::process::exit;
use amqp::{Consumer, Channel, protocol, Basic};

pub struct MyHandler {
    count: u64,
    start: SystemTime,
    max: u64,
}

impl MyHandler {
    pub fn new(max: u64) -> Self {
        MyHandler {
            count: 0,
            start: SystemTime::now(),
            max: max,
        }
    }
}

impl Consumer for MyHandler {
    fn handle_delivery(&mut self,
                       channel: &mut Channel,
                       deliver: protocol::basic::Deliver,
                       _: protocol::basic::BasicProperties,
                       _: Vec<u8>) {
        self.count = self.count + 1;
        let _ = channel.basic_ack(deliver.delivery_tag, false);
        if self.count == self.max {
            let sys_time = SystemTime::now();
            let diff = sys_time.duration_since(self.start)
                .expect("SystemTime::duration_since failed");
            println!{"count {:?}, timer diff: {:?}", self.count, diff};
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
    pubsub.start_sub("request", vec!["response"], MyHandler::new(max));
    let mut _pub = pubsub.get_pub();
    for _ in 1..max+1 {
        _pub.publish("request", vec![0, 1]);
    }
    loop {
        thread::sleep(Duration::new(10,0));
    }
}
