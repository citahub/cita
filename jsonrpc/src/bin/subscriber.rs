use std::sync::{RwLock, Arc};
use libproto::{submodules, topics, parse_msg, cmd_id, display_cmd, MsgClass, blockchain, request};
use std::collections::HashMap;
use amqp::{Consumer, Channel, protocol, Basic};
use util::hash::H256;

#[derive(Default)]
pub struct MyHandler {
    pub responses: Arc<RwLock<HashMap<Vec<u8>, request::Response>>>,
    pub tx_responses: Arc<RwLock<HashMap<H256, blockchain::TxResponse>>>,
}

impl MyHandler {
    pub fn new() -> Self {
        MyHandler {
            responses: Arc::new(RwLock::new(HashMap::with_capacity(1000))),
            tx_responses: Arc::new(RwLock::new(HashMap::with_capacity(1000))),
        }
    }
}

impl Consumer for MyHandler {
    fn handle_delivery(&mut self,
                       channel: &mut Channel,
                       deliver: protocol::basic::Deliver,
                       _: protocol::basic::BasicProperties,
                       body: Vec<u8>) {
        let (id, _, content_ext) = parse_msg(body.as_slice());
        trace!("routint_key {:?},get msg cmid {:?}---",
               deliver.routing_key,
               display_cmd(id));
        if id == cmd_id(submodules::CHAIN, topics::RESPONSE) {
            if let MsgClass::RESPONSE(content) = content_ext {
                let mut responses = self.responses.write().unwrap();
                trace!("---from CHAIN RESPONSE---get response rid {:?}------",
                       content.request_id.clone());
                responses.insert(content.request_id.clone(), content);
            } else {
                warn!("from CHAIN RESPONSE Unable to parse right: {:?} ",
                      content_ext);
            }
        } else if id == cmd_id(submodules::CONSENSUS, topics::TX_RESPONSE) {
            if let MsgClass::TXRESPONSE(content) = content_ext {
                let mut tx_responses = self.tx_responses.write().unwrap();
                trace!("---from CONSENSUS TX_RESPONSE---get response hash {:?}------",
                       content.hash.clone());
                tx_responses.insert(H256::from(content.hash.clone().as_slice()), content);
            } else {
                warn!("from CONSENSUS TX_RESPONSE Unable to parse right: {:?} ",
                      content_ext);
            }
        } else {
            // warn!("Unable handle msg {:?}", content_ext);
        }
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}
