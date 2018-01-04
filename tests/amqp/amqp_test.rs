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

extern crate dotenv;
extern crate pubsub;

use dotenv::dotenv;
use pubsub::start_pubsub;
use std::env;
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("need only one argument : max msg count!");
        return;
    }
    dotenv().ok();
    let mut count = 0;
    let start = SystemTime::now();
    let max = args[1].parse::<u64>().unwrap();
    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    start_pubsub("request", vec!["response"], tx_sub, rx_pub);

    thread::spawn(move || {
        let (tx_sub, rx_sub) = channel();
        let (tx_pub, rx_pub) = channel();
        start_pubsub("response", vec!["request"], tx_sub, rx_pub);
        loop {
            let (_, msg) = rx_sub.recv().unwrap();
            tx_pub.clone().send(("response".to_string(), msg)).unwrap();
        }
    });

    thread::spawn(move || {
        for _ in 1..max + 1 {
            tx_pub.send(("request".to_string(), vec![0, 1])).unwrap();
        }
    });

    loop {
        let (key, msg) = rx_sub.recv().unwrap();
        count = count + 1;
        if count == max {
            println!("{} {:?}", key, msg);
            let sys_time = SystemTime::now();
            let diff = sys_time
                .duration_since(start)
                .expect("SystemTime::duration_since failed");
            println!{"count {:?}, timer diff: {:?}", count, diff};
            thread::sleep(Duration::new(2, 0));
            break;
        }
    }
}
