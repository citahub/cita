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

#![allow(unused_variables)]

pub use byteorder::{BigEndian, ByteOrder};
use core::filters::eth_filter::EthFilter;
use core::libchain::call_request::CallRequest;
pub use core::libchain::chain::*;
use jsonrpc_types::rpctypes;
use jsonrpc_types::rpctypes::{Filter as RpcFilter, Log as RpcLog, Receipt as RpcReceipt, CountAndCode, BlockNumber, BlockParamsByNumber, BlockParamsByHash, RpcBlock};
use libproto;
pub use libproto::*;
pub use libproto::request::Request_oneof_req as Request;
use libproto::submodules;
use protobuf::Message;
use serde_json;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{Sender, Receiver};
use std::vec::Vec;
use threadpool::*;
use types::filter::Filter;
use types::ids::BlockId;
use util::Address;
use util::H256;
// pub const CHAIN_PUB: u32 = 3;

pub fn chain_pool(pool: &ThreadPool, tx: &Sender<(u32, u32, u32, MsgClass)>, id: u32, msg: Vec<u8>) {
    let tx = tx.clone();
    pool.execute(move || {
                     let (cmd_id, origin, content) = parse_msg(msg.as_slice());
                     tx.send((id, cmd_id, origin, content)).unwrap();
                 });
}

// TODO: RPC Errors
pub fn chain_result(chain: Arc<Chain>, rx: &Receiver<(u32, u32, u32, MsgClass)>, ctx_pub: Sender<(String, Vec<u8>)>) {
    let (id, cmd_id, origin, content_ext) = rx.recv().unwrap();
    trace!("chain_result call {:?} {:?}", id_to_key(id), cmd_id);
    match content_ext {
        MsgClass::REQUEST(mut req) => {
            let mut response = request::Response::new();
            response.set_request_id(req.take_request_id());
            let mut topic = "chain.rpc".to_string();
            match req.req.clone().unwrap() {
                // TODO: should check the result, parse it first!
                Request::block_number(_) => {
                    // let sys_time = SystemTime::now();
                    let height = chain.get_current_height();
                    response.set_block_number(height);
                }

                Request::block_by_hash(rpc) => {
                    let rpc: BlockParamsByHash = serde_json::from_str(&rpc).expect("Invalid param");
                    let hash = rpc.hash;
                    let include_txs = rpc.include_txs;
                    match chain.block_by_hash(H256::from(hash.as_slice())) {
                        Some(block) => {
                            let rpc_block = RpcBlock::new(hash, include_txs, block.protobuf().write_to_bytes().unwrap());
                            //TODO，发生错误了，应该加错原因给rpc,通知客户
                            serde_json::to_string(&rpc_block)
                                .map(|data| response.set_block(data))
                                .map_err(|_| response.set_none(true));
                        }
                        None => {
                            response.set_none(true);
                        }
                    }
                }

                Request::block_by_height(block_height) => {
                    let block_height: BlockParamsByNumber = serde_json::from_str(&block_height).expect("Invalid param");
                    let include_txs = block_height.include_txs;
                    match chain.block(block_height.block_id.into()) {
                        Some(block) => {
                            let rpc_block = RpcBlock::new(block.hash().to_vec(), include_txs, block.protobuf().write_to_bytes().unwrap());
                            //TODO，发生错误了，应该加错原因给rpc,通知客户
                            serde_json::to_string(&rpc_block)
                                .map(|data| response.set_block(data))
                                .map_err(|_| response.set_none(true));
                        }
                        None => {
                            response.set_none(true);
                        }
                    }
                }

                Request::transaction(hash) => {
                    match chain.full_transaction(H256::from_slice(&hash)) {
                        Some(ts) => {
                            response.set_ts(ts);
                        }
                        None => {
                            response.set_none(true);
                        }
                    }
                }
                Request::transaction_receipt(hash) => {
                    let tx_hash = H256::from_slice(&hash);
                    let receipt = chain.localized_receipt(tx_hash);
                    if let Some(receipt) = receipt {
                        let rpc_receipt: RpcReceipt = receipt.into();
                        let serialized = serde_json::to_string(&rpc_receipt).unwrap();
                        response.set_receipt(serialized);
                    } else {
                        response.set_none(true);
                    }
                }

                Request::call(call) => {
                    trace!("Chainvm Call {:?}", call);
                    let block_id: BlockNumber = serde_json::from_str(&(call.height)).expect("Invalid param");
                    let call_request = CallRequest::from(call);
                    let result = chain.eth_call(call_request, block_id.into());
                    response.set_call_result(result.unwrap_or_default());
                    if id == submodules::CONSENSUS {
                        topic = "chain.role".to_string();
                    }
                }

                Request::filter(encoded) => {
                    trace!("filter: {:?}", encoded);
                    let rpc_filter: RpcFilter = serde_json::from_str(&encoded).expect("Invalid filter");
                    let filter: Filter = rpc_filter.into();
                    let logs = chain.get_logs(filter);
                    let rpc_logs: Vec<RpcLog> = logs.into_iter().map(|x| x.into()).collect();
                    response.set_logs(serde_json::to_string(&rpc_logs).unwrap());
                }

                Request::transaction_count(tx_count) => {
                    trace!("transaction count request from jsonrpc {:?}", tx_count);
                    //TODO 或许有错误返回给用户更好
                    let tx_count: CountAndCode = serde_json::from_str(&tx_count).expect("Invalid param");
                    let address = Address::from_slice(tx_count.address.as_ref());
                    match chain.nonce(&address, tx_count.block_id.into()) {
                        Some(nonce) => {
                            response.set_transaction_count(u64::from(nonce));
                        }
                        None => {
                            response.set_none(true);
                        }
                    };
                }

                Request::code(code_content) => {
                    trace!("code request from josnrpc  {:?}", code_content);
                    let code_content: CountAndCode = serde_json::from_str(&code_content).expect("Invalid param");

                    let address = Address::from_slice(code_content.address.as_ref());
                    match chain.code_at(&address, code_content.block_id.into()) {
                        Some(code) => {
                            match code {
                                Some(code) => {
                                    response.set_code(code);
                                }
                                None => {
                                    response.set_none(true);
                                }
                            }
                        }
                        None => {
                            response.set_none(true);
                        }
                    };
                }

                Request::new_filter(new_filter) => {
                    trace!("new_filter {:?}", new_filter);
                    let new_filter: RpcFilter = serde_json::from_str(&new_filter).expect("Invalid param");
                    trace!("new_filter {:?}", new_filter);
                    response.set_filter_id(chain.new_filter(new_filter) as u64);
                }

                Request::new_block_filter(_) => {
                    let block_filter = chain.new_block_filter();
                    response.set_filter_id(block_filter as u64);
                }

                Request::uninstall_filter(filter_id) => {
                    trace!("uninstall_filter's id is {:?}", filter_id);
                    let index = rpctypes::Index(filter_id as usize);
                    let b = chain.uninstall_filter(index);
                    response.set_uninstall_filter(b);
                }

                Request::filter_changes(filter_id) => {
                    trace!("filter_changes's id is {:?}", filter_id);
                    let index = rpctypes::Index(filter_id as usize);
                    let log = chain.filter_changes(index).unwrap();
                    trace!("Log is: {:?}", log);
                    response.set_filter_changes(serde_json::to_vec(&log).unwrap());
                }

                Request::filter_logs(filter_id) => {
                    trace!("filter_log's id is {:?}", filter_id);
                    let index = rpctypes::Index(filter_id as usize);
                    let log = chain.filter_logs(index).unwrap_or(vec![]);
                    trace!("Log is: {:?}", log);
                    response.set_filter_logs(serde_json::to_vec(&log).unwrap());
                }
                _ => {
                    error!("error msg!!!!");
                }
            };
            let msg: communication::Message = response.into();
            ctx_pub.send((topic, msg.write_to_bytes().unwrap())).unwrap();
        }

        MsgClass::BLOCK(block) => {
            let mut guard = chain.block_map.write();
            let current_height = chain.get_current_height();
            let max_height = chain.get_max_height();
            let blk_height = block.get_header().get_height();
            let new_map = guard.split_off(&current_height);
            *guard = new_map;

            trace!("received block: block_number:{:?} current_height: {:?} max_height: {:?}", blk_height, current_height, max_height);
            let source = match id {
                submodules::CONSENSUS => BlockSource::CONSENSUS,
                _ => BlockSource::NET,
            };

            if blk_height > current_height && blk_height < current_height + 300 {
                if !guard.contains_key(&blk_height) || (guard.contains_key(&blk_height) && guard[&blk_height].0 == BlockSource::NET && source == BlockSource::CONSENSUS) {
                    trace!("block insert {:?}", blk_height);
                    guard.insert(blk_height, (source, Block::from(block)));
                    let _ = chain.sync_sender.lock().send(blk_height);
                }
            }
        }

        MsgClass::STATUS(status) => {
            let status_height = status.get_height();
            if status_height > chain.get_max_height() {
                chain.max_height.store(status_height as usize, Ordering::SeqCst);
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
                    let msg = factory::create_msg_ex(submodules::CHAIN, topics::SYNC_BLK, communication::MsgType::MSG, communication::OperateType::SINGLE, origin, wtr);
                    trace!("origin {:?}, chain.sync: OperateType {:?}", origin, communication::OperateType::SINGLE);
                    ctx_pub.send(("chain.sync".to_string(), msg.write_to_bytes().unwrap())).unwrap();
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
                trace!("Receive sync {:?} from node-{:?}", BigEndian::read_u64(&content), origin);
                if let Some(block) = chain.block(BlockId::Number(BigEndian::read_u64(&content))) {
                    let msg = factory::create_msg_ex(submodules::CHAIN, topics::NEW_BLK, communication::MsgType::BLOCK, communication::OperateType::SINGLE, origin, block.protobuf().write_to_bytes().unwrap());
                    trace!("origin {:?}, chain.blk: OperateType {:?}", origin, communication::OperateType::SINGLE);
                    ctx_pub.send(("chain.blk".to_string(), msg.write_to_bytes().unwrap())).unwrap();
                }
            } else {
                warn!("other content.");
            }
        }
        MsgClass::VERIFYREQ(req) => {}
        MsgClass::VERIFYRESP(resp) => {}
        _ => {
            error!("error msg!!!!");
        }
    }
}
