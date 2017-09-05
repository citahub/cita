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
extern crate kafka;
extern crate env_logger;

use std::time::Duration;

use kafka::producer::{Producer,Record,RequiredAcks};
use kafka::consumer::{Consumer,FetchOffset,GroupOffsetStorage};
use kafka::error::Error as kafkaError;

use::std::sync::mpsc::Receiver;
use::std::sync::mpsc::Sender;
use std::thread;

pub fn start_kafkamq(name:&str,keys:Vec<String>,tx:Sender<(String,Vec<u8>)>,rx:Receiver<(String,Vec<u8>)>){
    println!("into start_kafkamq");
    env_logger::init().unwrap();
    let mut brokers:Vec<String> = Vec::new();
    brokers.push("localhost:9092".to_string());
    let group = "my-group".to_owned(); 
//producer thread
    let brokers1 = brokers.clone();
    let  _=thread::Builder::new().name("publisher".to_string()).spawn(move || {
        println!("producer thread running!");
        let mut tmp = 0;
       loop{
           tmp=tmp+1;
           println!("tmp:{}",tmp);
            let mut ret = rx.recv();
            if ret.is_err(){
                println!("break!!!!!");
                break;
            }
            let(topic,msg)=ret.unwrap(); 
            println!("{},{:?}",topic,msg);
        if let Err(e) = produce_message(&msg, &topic, brokers1.clone()) {
            println!("Failed producing messages: {}", e);
        }
                
        }
        
    });
//comsumer thread
    let brokers2 = brokers.clone();
    let _=thread::Builder::new().name("subscriber".to_string()).spawn(move ||{
        println!("consumer thread running!");
     for topic in keys
     {
        let mut con =    Consumer::from_hosts(brokers2.clone())
                        .with_topic(topic.to_string())
                        //.with_group(group)
                        .with_fallback_offset(FetchOffset::Earliest)
                        .with_offset_storage(GroupOffsetStorage::Kafka)
                        .create().unwrap();

        
        loop{
            let mss = con.poll().unwrap();
            if mss.is_empty(){
                println!("No Messages available right now.");
            }
            for ms in mss.iter(){
                for m in ms.messages(){
                    println!("{:?}",m.value);
                    let _=tx.send((topic.to_string(),m.value.to_vec()));
                } 
                let _=con.consume_messageset(ms);
            
            }
            con.commit_consumed().unwrap();
        }
        
    }
    });
}

fn produce_message<'a,'b>(data:&'a[u8],topic:&'b str,brokers:Vec<String>)->Result<(),kafkaError>{
    println!("About to publish a message at {:?} to: {}", brokers, topic);

    let mut producer = try!(Producer::from_hosts(brokers.to_owned())
                            .with_ack_timeout(Duration::from_secs(1))
                            .with_required_acks(RequiredAcks::One)
                            .create());
    try!(producer.send(&Record {
                           topic: topic,
                           partition: -1,
                           key: (),
                           value: data,
                       }));
    try!(producer.send(&Record::from_value(topic, data)));

    Ok(())
}
