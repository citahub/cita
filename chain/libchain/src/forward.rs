#![allow(unused_variables)]

use threadpool::*;
use std::sync::mpsc::{Sender, Receiver};
pub use chain::*;
pub use chain as cita_chain;
use libproto;
pub use libproto::*;
use self::request::Request_oneof_req as Request;
pub use byteorder::{BigEndian, ByteOrder};
use pubsub::Pub;
use util::hash::H256;
use util::Address;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use util::FixedHash;
use std::vec::Vec;
use call_request::CallRequest;
use state::types::ids::BlockId;
use jsonrpc_types::rpctypes::{Filter as RpcFilter, Log as RpcLog, Receipt as RpcReceipt};
use state::types::filter::Filter;
use serde_json;

pub const CHAIN_PUB: u32 = 3;

pub fn chain_pool(pool: &ThreadPool,
                  tx: &Sender<(u32, u32, u32, MsgClass)>,
                  id: u32,
                  msg: Vec<u8>) {
    let tx = tx.clone();
    pool.execute(move || {
                     let (cmd_id, origin, content) = parse_msg(msg.as_slice());
                     tx.send((id, cmd_id, origin, content)).unwrap();
                 });
}

// TODO: RPC Errors
pub fn chain_result(chain: Arc<Chain>, rx: &Receiver<(u32, u32, u32, MsgClass)>, _pub: &mut Pub) {
    let (id, cmd_id, origin, content_ext) = rx.recv().unwrap();
    trace!("chain_result call {:?} {:?}", id, cmd_id);
    match content_ext {
        MsgClass::REQUEST(mut req) => {
            let mut response = request::Response::new();
            response.set_request_id(req.take_request_id());
            match req.req.clone().unwrap() {
                // TODO: should check the result, parse it first!
                Request::block_number(_) => {
                    // let sys_time = SystemTime::now();
                    let height = chain.get_current_height();
                    response.set_block_number(height);
                    let msg: communication::Message = response.into();
                    _pub.publish("chain.rpc", msg.write_to_bytes().unwrap());
                }
                Request::block_by_hash(mut rpc) => {
                    let hash = rpc.take_hash();
                    let include_txs = rpc.include_txs;
                    match chain.block(BlockId::Hash(H256::from(hash.as_slice()))) {
                        Some(block) => {
                            let mut rpc_block = request::RpcBlock::new();
                            rpc_block.set_block(block.write_to_bytes().unwrap());
                            rpc_block.set_include_txs(include_txs);
                            rpc_block.set_hash(hash);
                            response.set_block(rpc_block);
                        }
                        None => {
                            response.set_none(true);
                        }
                    }
                    let msg: communication::Message = response.into();
                    _pub.publish("chain.rpc", msg.write_to_bytes().unwrap());
                }
                Request::block_by_height(rpc) => {
                    let number = rpc.height;
                    let include_txs = rpc.include_txs;
                    match chain.block(BlockId::Number(number)) {
                        Some(blk) => {
                            let mut rpc_block = request::RpcBlock::new();
                            rpc_block.set_block(blk.write_to_bytes().unwrap());
                            rpc_block.set_include_txs(include_txs);
                            // TODO: avoid to compute sha3
                            rpc_block.set_hash(blk.sha3().to_vec());
                            response.set_block(rpc_block);
                        }
                        None => {
                            trace!("Get block failed {:?}", number);
                            response.set_none(true);
                        }
                    }
                    let msg: communication::Message = response.into();
                    _pub.publish("chain.rpc", msg.write_to_bytes().unwrap());
                }
                Request::transaction(hash) => {
                    match chain.transaction(H256::from_slice(&hash)) {
                        Some(ts) => {
                            response.set_ts(ts);
                        }
                        None => {
                            response.set_none(true);
                        }
                    }
                    let msg: communication::Message = response.into();
                    _pub.publish("chain.rpc", msg.write_to_bytes().unwrap());
                }
                Request::transaction_receipt(hash) => {
                    let tx_hash = H256::from_slice(&hash);
                    let receipt = chain.transaction_address(tx_hash)
                         .and_then(|tx_address| chain.localized_receipt(tx_hash, tx_address));

                    if let Some(receipt) = receipt {
                        let rpc_receipt: RpcReceipt = receipt.into();
                        let serialized = serde_json::to_string(&rpc_receipt).unwrap();
                        response.set_receipt(serialized);
                    } else {
                        response.set_none(true);
                    }

                    let msg: communication::Message = response.into();
                    _pub.publish("chain.rpc", msg.write_to_bytes().unwrap());
                }
                Request::call(call) => {
                    trace!("Chainvm Call {:?}", call);
                    let block_id = if call.has_height() {
                        BlockId::Number(call.get_height())
                    } else {
                        match call.get_tag() {
                            BlockTag::Latest => BlockId::Latest,
                            BlockTag::Earliest => BlockId::Earliest,
                        }
                    };

                    let call_request = CallRequest::from(call);
                    let result = chain.cita_call(call_request, block_id);

                    response.set_call_result(result.unwrap_or_default());
                    let msg: communication::Message = response.into();
                    _pub.publish("chain.rpc", msg.write_to_bytes().unwrap());
                }
                Request::filter(encoded) => {
                    trace!("filter: {:?}", encoded);
                    let rpc_filter: RpcFilter = serde_json::from_str(&encoded).expect("Invalid filter");
                    let filter: Filter = rpc_filter.into();
                    let logs = chain.get_logs(filter);
                    let rpc_logs: Vec<RpcLog> = logs.into_iter().map(|x| x.into()).collect();
                    response.set_logs(serde_json::to_string(&rpc_logs).unwrap());
                    let msg: communication::Message = response.into();
                    _pub.publish("chain.rpc", msg.write_to_bytes().unwrap());
                }

                Request::transaction_count(tx_count) => {
                    let block_id = if tx_count.has_height() {
                        BlockId::Number(tx_count.get_height())
                    } else {
                        match tx_count.get_tag() {
                            BlockTag::Latest => BlockId::Latest,
                            BlockTag::Earliest => BlockId::Earliest,
                        }
                    };
                    let address = Address::from_slice(tx_count.address.as_ref());
                    match chain.nonce(&address, block_id) {
                        Some(nonce) => {
                            response.set_transaction_count(u64::from(nonce));
                        }
                        None => {}
                    };
                    let msg: communication::Message = response.into();
                    _pub.publish("chain.rpc", msg.write_to_bytes().unwrap());

                }

                Request::code(code_content) => {
                    let block_id = if code_content.has_height() {
                        BlockId::Number(code_content.get_height())
                    } else {
                        match code_content.get_tag() {
                            BlockTag::Latest => BlockId::Latest,
                            BlockTag::Earliest => BlockId::Earliest,
                        }
                    };
                    let address = Address::from_slice(code_content.address.as_ref());
                    match chain.code_at(&address, block_id) {
                        Some(code) => {
                            match code {
                                Some(code) => {
                                    response.set_code(code);
                                }
                                None => {}
                            }
                        }
                        None => {}
                    };
                    let msg: communication::Message = response.into();
                    _pub.publish("chain.rpc", msg.write_to_bytes().unwrap());

                }

                _ => {}
            }
        }
        MsgClass::RESPONSE(rep) => {}
        MsgClass::HEADER(header) => {}
        MsgClass::BODY(body) => {}
        MsgClass::BLOCK(block) => {
            let mut guard = chain.block_map.write();

            let current_height = chain.get_current_height();
            let max_height = chain.get_max_height();
            let blk_heght = block.get_header().get_height();

            let new_map = guard.split_off(&current_height);
            *guard = new_map;


            trace!("received block: block_number:{:?} current_height: {:?} max_height: {:?}",
                   blk_heght,
                   current_height,
                   max_height);
            let source = match id {
                submodules::CONSENSUS => BlockSource::CONSENSUS,
                _ => BlockSource::NET,
            };
            if blk_heght > current_height && blk_heght < current_height + 300 &&
               !guard.contains_key(&blk_heght) {
                trace!("block insert {:?}", blk_heght);
                guard.insert(blk_heght, (source, block.clone()));
                let _ = chain.sync_sender.lock().send(blk_heght);
            }

            if !chain.get_current_height() < chain.get_max_height() {
                chain.is_sync.store(false, Ordering::SeqCst);
            }
        }
        MsgClass::TX(content) => {}
        MsgClass::TXRESPONSE(content) => {}
        MsgClass::STATUS(status) => {
            let status_height = status.get_height();
            if status_height > chain.get_max_height() {
                chain
                    .max_height
                    .store(status_height as usize, Ordering::SeqCst);
                trace!("recieved status update max_height: {:?}", status_height);
            }
            let known_max_height = chain.get_max_height();
            let current_height = chain.get_current_height();
            let target_height = ::std::cmp::min(current_height + 100, known_max_height);
            if current_height < target_height && !chain.is_sync.load(Ordering::SeqCst) {
                let mut diff = target_height - current_height;
                let mut start_height = current_height + 1;
                while diff > 0 {
                    let mut wtr = vec![0; 8];
                    trace!("request sync {:?}", start_height);
                    BigEndian::write_u64(&mut wtr, start_height);
                    let msg = factory::create_msg_ex(submodules::CHAIN,
                                                     topics::SYNC_BLK,
                                                     communication::MsgType::MSG,
                                                     communication::OperateType::SINGLE,
                                                     origin,
                                                     wtr);
                    trace!("-origin-{:?}---chain.sync---{:?}--",
                           origin,
                           communication::OperateType::SINGLE);
                    _pub.publish("chain.sync", msg.write_to_bytes().unwrap());
                    start_height += 1;
                    diff -= 1;
                }
                if !chain.is_sync.load(Ordering::SeqCst) {
                    chain.is_sync.store(true, Ordering::SeqCst);
                }
            }
        }
        MsgClass::MSG(content) => {
            if libproto::cmd_id(submodules::CHAIN, topics::SYNC_BLK) == cmd_id {
                if let Some(block) = chain.block(BlockId::Number(BigEndian::read_u64(&content))) {
                    let msg = factory::create_msg_ex(submodules::CHAIN,
                                                     topics::NEW_BLK,
                                                     communication::MsgType::BLOCK,
                                                     communication::OperateType::SINGLE,
                                                     origin,
                                                     block.write_to_bytes().unwrap());
                    trace!("-origin-{:?}---chain.blk---{:?}--",
                           origin,
                           communication::OperateType::SINGLE);
                    _pub.publish("chain.blk", msg.write_to_bytes().unwrap());
                }
            } else {
                warn!("other content.");
            }
        }
    }
}
