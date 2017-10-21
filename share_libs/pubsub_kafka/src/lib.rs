#[macro_use]
extern crate log;
extern crate futures;
extern crate rdkafka;
extern crate futures_cpupool;

use futures::*;
use futures_cpupool::CpuPool;

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


pub const KAFKA_URL: &'static str = "KAFKA_URL";

pub fn start_kafka(name: &str, keys: Vec<String>, tx: Sender<(String, Vec<u8>)>, rx: Receiver<(String, Vec<u8>)>) {
    let brokers = std::env::var(KAFKA_URL).expect(format!("{} must be set", KAFKA_URL).as_str());
    let consumer_brokers = brokers.clone();
    let _ = thread::Builder::new().name("publisher".to_string()).spawn(move || {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set_default_topic_config(TopicConfig::new().set("produce.offset.report", "true").finalize())
            .set_log_level(RDKafkaLogLevel::Info)
            .create::<FutureProducer<_>>()
            .expect("Producer creation error");
        let cpu_pool = CpuPool::new(16);

        loop {
            let ret = rx.recv();
            if ret.is_err() {
                break;
            }
            let (topic, msg) = ret.unwrap();
            let f = producer.send_copy::<Vec<u8>, Vec<u8>>(&topic, None, Some(&msg), None, None)
                            .map(move |delivery_status| {
                                     trace!("send message {:?} to topic {:?}, delivery status: {:?}", msg, topic, delivery_status);
                                     delivery_status
                                 });
            let _ = cpu_pool.spawn(f);
        }
    });

    type LoggingConsumer = StreamConsumer<ConsumerContextExample>;

    let context = ConsumerContextExample;

    info!("kafka broker: {}, consumer group {}", consumer_brokers, name);
    let consumer = ClientConfig::new()
        .set("group.id", name)
        .set("bootstrap.servers", &consumer_brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_default_topic_config(TopicConfig::new().set("auto.offset.reset", "smallest").finalize())
        .set_log_level(RDKafkaLogLevel::Info)
        .create_with_context::<_, LoggingConsumer>(context)
        .expect("Consumer creation failed");

    //consumer thread
    let keys_str = keys.clone();

    let _ = thread::Builder::new().name("subscriber".to_string()).spawn(move || {
        let keys_str = keys_str.clone();
        let keys = keys_str.iter().map(|elem| &elem[..]).collect::<Vec<_>>();
        consumer.subscribe(keys.as_slice()).expect("Can't subscribe to specified topics");

        let message_stream = consumer.start();
        for message in message_stream.wait() {
            match message {
                Err(_) => {
                    error!("Error while reading from stream.");
                }
                Ok(Ok(m)) => {
                    let payload = match m.payload_view::<[u8]>() {
                        None => {
                            error!("Deserialize message error, get payload is None");
                            &[]
                        }
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            error!("Error while deserializing message payload: {:?}", e);
                            &[]
                        }
                    };

                    let _ = tx.send((m.topic().to_string(), payload.to_vec()));
                    trace!("payload: '{:?}', topic: {}, partition: {}, offset: {}", payload.to_vec(), m.topic(), m.partition(), m.offset());
                    consumer.commit_message(&m, CommitMode::Async).unwrap();
                }
                Ok(Err(e)) => {
                    error!("Kafka error: {}", e);
                }
            };
        }
    });
}
