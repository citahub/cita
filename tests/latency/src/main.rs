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

extern crate pubsub;

use pubsub::start_pubsub;
use std::env;
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("need two argument : max msg count and msg size!");
        return;
    }
    let max = args[1].parse::<u64>().unwrap();
    let size = args[2].parse::<usize>().unwrap();
    println!{"test count {:?}, test size: {:?}", max, size};
    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    start_pubsub("latency_req", vec!["latency_res"], tx_sub, rx_pub);

    thread::spawn(move || {
        let (tx_sub, rx_sub) = channel();
        let (tx_pub, rx_pub) = channel();
        start_pubsub("latency_res", vec!["latency_req"], tx_sub, rx_pub);
        loop {
            let (_, msg) = rx_sub.recv().unwrap();
            tx_pub.send(("latency_res".to_string(), msg)).unwrap();
        }
    });

    let filesize = vec![0u8; size];
    tx_pub.send(("latency_req".to_string(), filesize)).unwrap();

    let mut count = 0;
    let start = SystemTime::now();
    loop {
        let (_, msg) = rx_sub.recv().unwrap();
        tx_pub.send(("latency_req".to_string(), msg)).unwrap();
        count = count + 1;
        if count == max {
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
