#[macro_use]
extern crate log;
extern crate futures;
extern crate rdkafka;

use futures::*;

use rdkafka::Message;
use rdkafka::client::Context;
use rdkafka::config::{ClientConfig, TopicConfig, RDKafkaLogLevel};
use rdkafka::consumer::{Consumer, ConsumerContext, CommitMode, Rebalance};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::error::KafkaResult;
use rdkafka::producer::FutureProducer;
use rdkafka::types;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

// The Context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular ConsumerContext sets up custom callbacks to log rebalancing events.
struct ConsumerContextExample;

impl Context for ConsumerContextExample {}

impl ConsumerContext for ConsumerContextExample {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        trace!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        trace!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, _result: KafkaResult<()>, _offsets: *mut types::RDKafkaTopicPartitionList) {
        trace!("Committing offsets");
    }
}




//producer thread
pub fn start_kafka(_: &str, keys: Vec<String>, tx: Sender<(String, Vec<u8>)>, rx: Receiver<(String, Vec<u8>)>) {
    let brokers = "localhost:9092";
    let _ = thread::Builder::new().name("publisher".to_string()).spawn(move || {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set_default_topic_config(TopicConfig::new().set("produce.offset.report", "true").finalize())
            .create::<FutureProducer<_>>()
            .expect("Producer creation error");

        loop {
            let ret = rx.recv();
            if ret.is_err() {
                break;
            }
            let (topic, msg) = ret.unwrap();

            // The send operation on the topic returns a future, that will be completed once the
            // result or failure from Kafka will be received.
            let _ = producer.send_copy(&topic, None, Some(&msg), Some(&vec![0, 1, 2, 3]), None)
                            .map(move |delivery_status| {
                                     // This will be executed onw the result is received
                                     //println!("Delivery status for message {} received", 1);
                                     delivery_status
                                 })
                            .wait();

        }
    });
    //thread::sleep(Duration::new(10,0));



    //A type alias with your custom consumer can be created for convenience.
    type LoggingConsumer = StreamConsumer<ConsumerContextExample>;

    //let topics = "network.newtx";
    let brokers = "localhost:9092";

    let context = ConsumerContextExample;

    let consumer = ClientConfig::new()
        .set("group.id", "example_consumer_group_id")
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("statistics.interval.ms", "30000")
        .set_default_topic_config(TopicConfig::new().set("auto.offset.reset", "smallest").finalize())
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context::<_, LoggingConsumer>(context)
        .expect("Consumer creation failed");

    //consumer thread
    let keys_str = keys.clone();

    let _ = thread::Builder::new().name("subscriber".to_string()).spawn(move || {
        loop {
            let keys_str = keys_str.clone();
            let keys = keys_str.iter().map(|elem| &elem[..]).collect::<Vec<_>>();
            consumer.subscribe(keys.as_slice()).expect("Can't subscribe to specified topics");
            // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
            // such as complex computations on a thread pool or asynchronous IO.
            let message_stream = consumer.start();
            for message in message_stream.wait() {
                match message {
                    Err(_) => {
                        trace!("Error while reading from stream.");
                    }
                    Ok(Ok(m)) => {
                        let key = match m.key_view::<[u8]>() {
                            None => &[],
                            Some(Ok(s)) => s,
                            Some(Err(e)) => {
                                trace!("Error while deserializing message key: {:?}", e);
                                &[]
                            }
                        };
                        let payload = match m.payload_view::<str>() {
                            None => "",
                            Some(Ok(s)) => s,
                            Some(Err(e)) => {
                                trace!("Error while deserializing message payload: {:?}", e);
                                ""
                            }
                        };

                        let _ = tx.send((m.topic().to_string(), payload.as_bytes().to_vec()));
                        trace!("key: '{:?}', payload: '{:?}', topic: {}, partition: {}, offset: {}", key, payload.as_bytes(), m.topic(), m.partition(), m.offset());
                        consumer.commit_message(&m, CommitMode::Async).unwrap();
                    }
                    Ok(Err(e)) => {
                        trace!("Kafka error: {}", e);
                    }
                };
            }

        }
    });
}
