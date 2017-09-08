#[macro_use] extern crate log;
extern crate clap;
extern crate futures;
extern crate rdkafka;
extern crate rdkafka_sys;

use std::thread;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use clap::{App, Arg};
use futures::*;

use rdkafka::Message;
use rdkafka::client::{Context};
use rdkafka::consumer::{Consumer, ConsumerContext, CommitMode, Rebalance};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::config::{ClientConfig, TopicConfig, RDKafkaLogLevel};
use rdkafka::producer::FutureProducer;
use rdkafka::error::KafkaResult;

// The Context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular ConsumerContext sets up custom callbacks to log rebalancing events.
struct ConsumerContextExample;

impl Context for ConsumerContextExample {}

impl ConsumerContext for ConsumerContextExample {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, _result: KafkaResult<()>, _offsets: *mut rdkafka_sys::RDKafkaTopicPartitionList) {
        info!("Committing offsets");
    }
}




//producer thread 
pub fn  start_kafka(name: &str, keys: Vec<&str>, tx: Sender<(String,Vec<u8>)>, rx: Receiver<(String, Vec<u8>)>) {
    println!("start kafka!");
    let brokers = "localhost:9092";
    let _ = thread::Builder::new().name("publisher".to_string()).spawn(move || {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id","example_consumer_group_id")
            .set_default_topic_config(TopicConfig::new()
                .set("produce.offset.report", "true")
                .finalize())
            .create::<FutureProducer<_>>()
            .expect("Producer creation error");

        loop {
            let ret = rx.recv();
            if ret.is_err() {
                break;
            }   
                    let (topic, msg) = ret.unwrap();  
            println!("topic:{},msg:{:?}",topic,msg);   
            let futures = (0..5)
                .map(|i| {
                    let value = format!("Message {}", i);
                    // The send operation on the topic returns a future, that will be completed once the
                    // result or failure from Kafka will be received.
                    producer.send_copy(&topic, None, Some(&value), Some(&msg), None)
                        .map(move |delivery_status| {   // This will be executed onw the result is received
                            info!("Delivery status for message {} received", i);
                            delivery_status
                        })
                })
                .collect::<Vec<_>>();

            // This loop will wait until all delivery statuses have been received received.
            for future in futures {
                info!("Future completed. Result: {:?}", future.wait());
            } 
        }     
       
                
    });




// A type alias with your custom consumer can be created for convenience.
// type LoggingConsumer = StreamConsumer<ConsumerContextExample>;

// let matches = App::new("consumer example")
//         .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
//         .about("Simple command line consumer")
//         .arg(Arg::with_name("group-id")
//              .short("g")
//              .long("group-id")
//              .help("Consumer group id")
//              .takes_value(true)
//              .default_value("example_consumer_group_id"))
//         .get_matches();

// let group_id = matches.value_of("group-id").unwrap();
// let topics = "hello";
// let brokers = "localhost:9092";

// let context = ConsumerContextExample;
//     let consumer = ClientConfig::new()
//         .set("group.id", group_id)
//         .set("bootstrap.servers", brokers)
//         .set("enable.partition.eof", "false")
//         .set("session.timeout.ms", "6000")
//         .set("enable.auto.commit", "true")
//         .set("statistics.interval.ms", "30000")
//         .set_log_level(RDKafkaLogLevel::Debug)
//         .create_with_context::<_, LoggingConsumer>(context)
//         .expect("Consumer creation failed");

 
        


// //consumer thread
//     let _ = thread::Builder::new().name("subscriber".to_string()).spawn(move || {
//         consumer.subscribe(&[topics])
//         .expect("Can't subscribe to specified topics");
//         println!("topic:{}",topics);

//  // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
//     // such as complex computations on a thread pool or asynchronous IO.
//         let message_stream = consumer.start();
//         println!("message_staream");
//         for message in message_stream.wait() {
//             println!("into message_stream");
//             match message {
//                 Err(_) => {
//                     warn!("Error while reading from stream.");
//                     println!("err");
//                 },
//                 Ok(Ok(m)) => {
//                     println!("ok branch");
//                     println!("{:?}",m);
//                     let key = match m.key_view::<[u8]>() {
//                         None => &[],
//                         Some(Ok(s)) => s,
//                         Some(Err(e)) => {
//                             warn!("Error while deserializing message key: {:?}", e);
//                             println!("Error while deserializing message key: {:?}", e);
//                             &[]
//                         },
//                     };
//                     let payload = match m.payload_view::<str>() {
//                         None => "",
//                         Some(Ok(s)) => s,
//                         Some(Err(e)) => {
//                             warn!("Error while deserializing message payload: {:?}", e);
//                             println!("Error while deserializing message payload: {:?}", e);
//                             ""
//                         },
//                     };
//                     info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}",
//                         key, payload, m.topic(), m.partition(), m.offset());
//                     println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}",
//                         key, payload, m.topic(), m.partition(), m.offset());
//                     consumer.commit_message(&m, CommitMode::Async).unwrap();
//                 },
//                 Ok(Err(e)) => {
//                     println!("kafka error");
//                     warn!("Kafka error: {}", e);
//                 },
//             };
//         }
//     });
 }
// fn produce(brokers: &str, topic_name: &str) {
//     let producer = ClientConfig::new()
//         .set("bootstrap.servers", brokers)
//         .set("group.id","example_consumer_group_id")
//         .set_default_topic_config(TopicConfig::new()
//             .set("produce.offset.report", "true")
//             .finalize())
//         .create::<FutureProducer<_>>()
//         .expect("Producer creation error");

//     // This loop is non blocking: all messages will be sent one after the other, without waiting
//     // for the results.
//     let futures = (0..5)
//         .map(|i| {
//             let value = format!("Message {}", i);
//             // The send operation on the topic returns a future, that will be completed once the
//             // result or failure from Kafka will be received.
//             producer.send_copy(topic_name, None, Some(&value), Some(&vec![0, 1, 2, 3]), None)
//                 .map(move |delivery_status| {   // This will be executed onw the result is received
//                     info!("Delivery status for message {} received", i);
//                     delivery_status
//                 })
//         })
//         .collect::<Vec<_>>();

//     // This loop will wait until all delivery statuses have been received received.
//    for future in futures {
//         info!("Future completed. Result: {:?}", future.wait());
//     }
// }

                                                                                                              

