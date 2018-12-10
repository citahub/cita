// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use cita_types::{Address, H256, U256};
use core::contracts::solc::sys_config::ChainId;
use core::libexecutor::blacklist::BlackList;
use core::libexecutor::block::{ClosedBlock, OpenBlock};
use core::libexecutor::call_request::CallRequest;
use core::libexecutor::economical_model::EconomicalModel;
use core::receipt::ReceiptError;
use crossbeam_channel::{Receiver, Sender};
use error::ErrorCode;
use jsonrpc_types::rpctypes::{BlockNumber, CountOrCode};
use libproto::auth::Miscellaneous;
use libproto::blockchain::{RichStatus, StateSignal};
use libproto::request::Request_oneof_req as Request;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::{request, response, Message};
use serde_json;
use std::convert::{Into, TryFrom, TryInto};
use std::sync::RwLock;
use std::u8;
use types::ids::BlockId;

use core::libexecutor::command;
use core::libexecutor::lru_cache::LRUCache;
use evm::Schedule;

use super::backlogs::{wrap_height, Backlogs};

pub struct Postman {
    backlogs: Backlogs,
    black_list_cache: RwLock<LRUCache<u64, Address>>,
    mq_req_receiver: Receiver<(String, Vec<u8>)>,
    mq_resp_sender: Sender<(String, Vec<u8>)>,
    fsm_req_sender: Sender<OpenBlock>,
    fsm_resp_receiver: Receiver<ClosedBlock>,
    command_req_sender: Sender<command::Command>,
    command_resp_receiver: Receiver<command::CommandResp>,
}

impl Postman {
    #[allow(unknown_lints, clippy::too_many_arguments)]
    pub fn new(
        current_height: u64,
        current_hash: H256,
        mq_req_receiver: Receiver<(String, Vec<u8>)>,
        mq_resp_sender: Sender<(String, Vec<u8>)>,
        fsm_req_sender: Sender<OpenBlock>,
        fsm_resp_receiver: Receiver<ClosedBlock>,
        command_req_sender: Sender<command::Command>,
        command_resp_receiver: Receiver<command::CommandResp>,
    ) -> Self {
        Postman {
            backlogs: Backlogs::new(current_height, current_hash),
            black_list_cache: RwLock::new(LRUCache::new(10_000_000)),
            mq_req_receiver,
            mq_resp_sender,
            fsm_req_sender,
            fsm_resp_receiver,
            command_req_sender,
            command_resp_receiver,
        }
    }

    pub fn do_loop(&mut self) {
        // 1. broadcast current state toward cita-chain
        self.bootstrap_broadcast();

        // 2. listen and handle messages
        loop {
            match self.recv() {
                (None, None) | (Some(_), Some(_)) => return,
                (Some((key, msg_vec)), None) => {
                    let result = self.handle_mq_message(key.as_str(), msg_vec);
                    if let Err(rollback_id) = result {
                        self.close(rollback_id);
                        return;
                    }
                }
                (None, Some(closed_block)) => {
                    self.handle_fsm_response(closed_block);
                    self.grow_up();
                    self.execute_next_block();
                }
            }
        }
    }

    // call this function every times postman start, to broadcast current state
    // to cita-chain. This broadcast state only contains system config and block header,
    // but not block body, cita-chain would specially deal with it.
    fn bootstrap_broadcast(&mut self) {
        // ensure recent 2 executed result stored in backlogs
        let current_height = self.get_current_height();
        self.load_executed_result(current_height);
        if current_height != 0 {
            self.load_executed_result(current_height - 1);
        }

        // broadcast toward cita-chain
        let bootstrap_executed_result = self
            .backlogs
            .get_completed_result(current_height)
            .expect("loaded from the previous step above; qed");
        let msg: Message = bootstrap_executed_result.clone().into();
        self.response_mq(
            routing_key!(Executor >> ExecutedResult).into(),
            msg.try_into().unwrap(),
        );
    }

    // make sure executor exit also
    fn close(&self, rollback_id: BlockId) {
        command::exit(
            &self.command_req_sender,
            &self.command_resp_receiver,
            rollback_id,
        );
    }

    // listen messages from RabbitMQ and Executor.
    //
    // Return `(None, None)` if any channel closed
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
    fn recv(&self) -> (Option<(String, Vec<u8>)>, Option<ClosedBlock>) {
        select! {
            recv(self.mq_req_receiver, mq_req) => {
                match mq_req {
                    Some(mq_req) => (Some(mq_req), None),
                    None => (None, None),
                }
            },
            recv(self.fsm_resp_receiver, fsm_resp) => {
                match fsm_resp {
                    Some(fsm_resp) => (None, Some(fsm_resp)),
                    None => (None, None),
                }
            }
        }
    }

    // update executed result into backlogs based on arrived result from executor
    fn handle_fsm_response(&mut self, closed_block: ClosedBlock) {
        let height = closed_block.number();
        info!("postman receive {}-th ClosedBlock from executor", height);
        self.backlogs.insert_closed(height, closed_block);
    }

    fn handle_mq_message(&mut self, key: &str, msg_vec: Vec<u8>) -> Result<(), BlockId> {
        let mut msg = Message::try_from(msg_vec).unwrap();
        trace!("receive {} from RabbitMQ", key);
        match RoutingKey::from(key) {
            routing_key!(Auth >> MiscellaneousReq) => {
                self.reply_auth_miscellaneous();
            }

            routing_key!(Chain >> Request) => {
                let req = msg.take_request().unwrap();
                self.reply_chain_request(req);
            }

            routing_key!(Chain >> RichStatus) => {
                if let Some(status) = msg.take_rich_status() {
                    self.update_by_rich_status(&status);
                };
            }

            routing_key!(Chain >> StateSignal) => {
                if let Some(state_signal) = msg.take_state_signal() {
                    self.reply_chain_state_signal(&state_signal)?;
                }
            }

            routing_key!(Consensus >> SignedProposal)
            | routing_key!(Net >> SignedProposal)
            | routing_key!(Consensus >> BlockWithProof)
            | routing_key!(Net >> SyncResponse)
            | routing_key!(Chain >> LocalSync) => {
                self.update_backlog(key, msg);
                self.grow_up();
                self.execute_next_block();
            }

            _ => {
                error!("receive unknown key: {} !!!!", key);
            }
        }
        Ok(())
    }

    // cita-chain broadcast StateSignal to indicate its state. So we could figure out
    // which blocks cita-chain lack of, then re-send the lacking blocks to cita-chain.
    fn reply_chain_state_signal(&self, state_signal: &StateSignal) -> Result<(), BlockId> {
        let specified_height = state_signal.get_height();
        if specified_height < self.get_current_height() {
            self.send_executed_info_to_chain(specified_height + 1)?;
            for height in self.backlogs.completed_keys() {
                if *height > specified_height + 1 {
                    self.send_executed_info_to_chain(*height)?;
                }
            }
        } else if specified_height > self.get_current_height() {
            self.signal_to_chain();
        }
        Ok(())
    }

    fn send_executed_info_to_chain(&self, height: u64) -> Result<(), BlockId> {
        if height > self.get_current_height() {
            error!("This must be because the Executor database was manually deleted.");
            return Ok(());
        }

        let executed_result = self.backlogs.get_completed_result(height);

        // Consider an abnormal case:
        //
        // 1. Our local node is height 50, and lags behind others
        //    100 blocks, so let's start to synchronize!
        //
        // 2. During synchronizing, cita-executor catches up to height 60, and sends
        //    notifications `ExecutedResult<height=51..60>` (from executing sync blocks) to
        //    cita-chain.
        //
        // 3. But suddenly cita-executor and cita-chain crashes before cita-chain
        //    receiving those `ExecutedResult<height=51..60>`, and even these within
        //    RabbitMQ be lost!
        //
        // 4. Cita-executor restart, its height is 60.
        //
        // 5. Cita-chain restart, its height is still 50.
        //
        // At this case above, cita-executor would hear `StateSignal<height=50>` from
        // cita-chain. But cita-executor could not anymore construct
        // `ExecutedResult<height=51..60>` based on its persisted data. It has to rollback
        // to 50 to keep equal to cita-chain, and then re-synchronize.
        //
        // Here the returned value `BlockId::Number(height - 1)` would be passed out to main()
        // thread. Then main() would restart executor thread and let executor starts with
        // `BlockId::Number(height - 1)`.
        if executed_result.is_none() {
            warn!(
                "chain(height={}) is lagging behind executor(height={}). \
                 Gonna roll back to {}",
                height,
                self.get_current_height(),
                height - 1
            );
            return Err(BlockId::Number(height - 1));
        }

        trace!("send {}-th ExecutedResult", height);
        let executed_result = executed_result.unwrap().clone();
        let msg: Message = executed_result.into();
        self.response_mq(
            routing_key!(Executor >> ExecutedResult).into(),
            msg.try_into().unwrap(),
        );
        Ok(())
    }

    fn update_backlog(&mut self, key: &str, mut msg: Message) -> bool {
        match RoutingKey::from(key) {
            // Proposal{block: {body, previous_proof}}
            //   WHERE previous_proof.height == block.height - 1
            routing_key!(Consensus >> SignedProposal) | routing_key!(Net >> SignedProposal) => {
                let mut proposal = msg.take_signed_proposal().unwrap();
                let open_block = OpenBlock::from(proposal.take_proposal().take_block());
                self.backlogs.insert_proposal(open_block)
            }

            // BlockWithProof{present_proof, block: {body, previous_proof}}
            //   WHERE present_proof.height == block.height
            //     AND previous_proof.height == block.height - 1
            routing_key!(Consensus >> BlockWithProof) => {
                let mut proofed = msg.take_block_with_proof().unwrap();
                let open_block = OpenBlock::from(proofed.take_blk());
                self.backlogs.insert_block_with_proof(open_block)
            }

            // SyncBlock{block: {body, previous_proof}}
            //   WHERE previous_proof.height == block.height - 1
            routing_key!(Net >> SyncResponse) | routing_key!(Chain >> LocalSync) => {
                let mut sync_res = msg.take_sync_response().unwrap();
                for proto_block in sync_res.take_blocks().into_iter() {
                    let open_block = OpenBlock::from(proto_block);
                    if !self.backlogs.insert_synchronized(open_block) {
                        return false;
                    }
                }
                true
            }
            _ => unimplemented!(),
        }
    }

    fn load_executed_result(&mut self, height: u64) {
        let executed_result = command::load_executed_result(
            &self.command_req_sender,
            &self.command_resp_receiver,
            height,
        );
        self.backlogs
            .insert_completed_result(height, executed_result);
    }

    // Grow up if current block executed completely,
    // 1. Update backlogs
    // 2. Update black list
    // 3. Notify executor to grow up too
    // 4. Delivery rich status of new height
    fn grow_up(&mut self) {
        let next_height = self.get_current_height() + 1;
        match self.backlogs.complete(next_height) {
            Ok(closed_block) => {
                trace!("postman notice executor to grow up to {}", next_height);
                self.pub_black_list(&closed_block);
                let executed_result = command::grow(
                    &self.command_req_sender,
                    &self.command_resp_receiver,
                    closed_block,
                );
                self.backlogs
                    .insert_completed_result(next_height, executed_result);
                self.send_executed_info_to_chain(next_height).unwrap();
            }
            Err(reason) => trace!("{}", reason),
        }
    }

    fn execute_next_block(&mut self) {
        let next_height = self.get_current_height() + 1;
        match self.backlogs.ready(next_height) {
            Ok(open_block) => {
                trace!("postman send {}-th block to executor", next_height);
                self.fsm_req_sender.send(open_block.clone());
            }
            Err(reason) => trace!("{}", reason),
        }
    }

    fn get_economical_model(&self) -> EconomicalModel {
        command::economical_model(&self.command_req_sender, &self.command_resp_receiver)
    }

    /// Find the public key of all senders that caused the specified error message, and then publish it
    // TODO: I think it is not necessary to distinguish economical_model, maybe remove
    //       this opinion in the future
    fn pub_black_list(&self, close_block: &ClosedBlock) {
        match self.get_economical_model() {
            EconomicalModel::Charge => {
                // Get all transaction hash that is reported as not enough quota
                let blacklist_transaction_hash: Vec<H256> = close_block
                    .receipts
                    .iter()
                    .filter(|ref receipt| match receipt.error {
                        Some(ReceiptError::NotEnoughBaseQuota) => true,
                        _ => false,
                    })
                    .map(|receipt| receipt.transaction_hash)
                    .filter(|hash| hash != &H256::default())
                    .collect();

                let schedule = Schedule::new_v1();
                // Filter out accounts in the black list where the account balance has reached the benchmark value.
                // Get the smaller value between tx_create_gas and tx_gas for the benchmark value.
                let bm_value = std::cmp::min(schedule.tx_gas, schedule.tx_create_gas);
                let mut clear_list: Vec<Address> = self
                    .black_list_cache
                    .read()
                    .unwrap()
                    .values()
                    .filter(|address| {
                        close_block
                            .state
                            .balance(address)
                            .and_then(|x| Ok(x >= U256::from(bm_value)))
                            .unwrap_or(false)
                    })
                    .cloned()
                    .collect();

                // Get address of sending account by transaction hash
                let blacklist: Vec<Address> = close_block
                    .body()
                    .transactions()
                    .iter()
                    .filter(|tx| blacklist_transaction_hash.contains(&tx.get_transaction_hash()))
                    .map(|tx| *tx.sender())
                    .collect();

                {
                    let mut black_list_cache = self.black_list_cache.write().unwrap();
                    black_list_cache
                        .prune(&clear_list)
                        .extend(&blacklist[..], close_block.number());
                    clear_list.extend(black_list_cache.lru().iter());
                }

                let black_list = BlackList::new()
                    .set_black_list(blacklist)
                    .set_clear_list(clear_list);

                if !black_list.is_empty() {
                    let black_list_bytes: Message = black_list.protobuf().into();

                    info!(
                        "black list is {:?}, clear list is {:?}",
                        black_list.black_list(),
                        black_list.clear_list()
                    );

                    self.response_mq(
                        routing_key!(Executor >> BlackList).into(),
                        black_list_bytes.try_into().unwrap(),
                    );
                }
            }
            EconomicalModel::Quota => {}
        }
    }

    fn update_by_rich_status(&mut self, rich_status: &RichStatus) {
        let next_height = wrap_height(rich_status.get_height() as usize + 1);
        self.backlogs.prune(next_height);
    }

    fn reply_auth_miscellaneous(&self) {
        let mut miscellaneous = Miscellaneous::new();
        let option = command::chain_id(&self.command_req_sender, &self.command_resp_receiver);
        if let Some(chain_id) = option {
            match chain_id {
                ChainId::V0(v0) => miscellaneous.set_chain_id(v0),
                ChainId::V1(v1) => miscellaneous.set_chain_id_v1(<[u8; 32]>::from(v1).to_vec()),
            }

            trace!("reply miscellaneous msg, chain_id: {:?}", chain_id);
        }

        let msg: Message = miscellaneous.into();
        self.response_mq(
            routing_key!(Executor >> Miscellaneous).into(),
            msg.try_into().unwrap(),
        );
    }

    fn reply_chain_request(&self, mut req: request::Request) {
        let mut response = response::Response::new();
        response.set_request_id(req.take_request_id());

        match req.req.unwrap() {
            Request::call(call) => {
                trace!("Chainvm Call {:?}", call);
                let _ = serde_json::from_str::<BlockNumber>(&call.height)
                    .map(|block_id| {
                        let call_request = CallRequest::from(call);
                        command::eth_call(
                            &self.command_req_sender,
                            &self.command_resp_receiver,
                            call_request,
                            block_id.into(),
                        )
                        .map(|ok| {
                            response.set_call_result(ok);
                        })
                        .map_err(|err| {
                            response.set_code(ErrorCode::query_error());
                            response.set_error_msg(err);
                        })
                    })
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    });
            }

            Request::transaction_count(tx_count) => {
                trace!("transaction count request from jsonrpc {:?}", tx_count);
                let _ = serde_json::from_str::<CountOrCode>(&tx_count)
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    })
                    .map(|tx_count| {
                        let address = Address::from_slice(tx_count.address.as_ref());
                        match command::nonce_at(
                            &self.command_req_sender,
                            &self.command_resp_receiver,
                            address,
                            tx_count.block_id.into(),
                        ) {
                            Some(nonce) => {
                                response.set_transaction_count(u64::from(nonce));
                            }
                            None => {
                                response.set_transaction_count(0);
                            }
                        };
                    });
            }

            Request::code(code_content) => {
                trace!("code request from jsonrpc  {:?}", code_content);
                let _ = serde_json::from_str::<CountOrCode>(&code_content)
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    })
                    .map(|code_content| {
                        let address = Address::from_slice(code_content.address.as_ref());
                        if let Some(code) = command::code_at(
                            &self.command_req_sender,
                            &self.command_resp_receiver,
                            address,
                            code_content.block_id.into(),
                        ) {
                            response.set_contract_code(code);
                        } else {
                            response.set_contract_code(vec![]);
                        };
                    });
            }

            Request::abi(abi_content) => {
                trace!("abi request from jsonrpc  {:?}", abi_content);
                let _ = serde_json::from_str::<CountOrCode>(&abi_content)
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    })
                    .map(|abi_content| {
                        let address = Address::from_slice(abi_content.address.as_ref());
                        if let Some(abi) = command::abi_at(
                            &self.command_req_sender,
                            &self.command_resp_receiver,
                            address,
                            abi_content.block_id.into(),
                        ) {
                            response.set_contract_abi(abi);
                        } else {
                            response.set_contract_abi(vec![]);
                        };
                    });
            }

            Request::balance(balance_content) => {
                trace!("balance request from jsonrpc  {:?}", balance_content);
                let _ = serde_json::from_str::<CountOrCode>(&balance_content)
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    })
                    .map(|balance_content| {
                        let address = Address::from_slice(balance_content.address.as_ref());
                        if let Some(balance) = command::balance_at(
                            &self.command_req_sender,
                            &self.command_resp_receiver,
                            address,
                            balance_content.block_id.into(),
                        ) {
                            response.set_balance(balance);
                        } else {
                            response.set_balance(vec![]);
                        };
                    });
            }

            Request::meta_data(data) => {
                match command::metadata(&self.command_req_sender, &self.command_resp_receiver, data)
                {
                    Ok(metadata) => {
                        response.set_meta_data(serde_json::to_string(&metadata).unwrap())
                    }
                    Err(error_msg) => {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(error_msg);
                    }
                }
            }

            Request::state_proof(state_info) => {
                trace!("state_proof info is {:?}", state_info);
                let _ = serde_json::from_str::<BlockNumber>(&state_info.height)
                    .map(|block_id| {
                        match command::state_at(
                            &self.command_req_sender,
                            &self.command_resp_receiver,
                            block_id.into(),
                        )
                        .and_then(|state| {
                            state.get_state_proof(
                                &Address::from(state_info.get_address()),
                                &H256::from(state_info.get_position()),
                            )
                        }) {
                            Some(state_proof_bs) => {
                                response.set_state_proof(state_proof_bs);
                            }
                            None => {
                                response.set_code(ErrorCode::query_error());
                                response.set_error_msg("get state proof failed".to_string());
                            }
                        }
                    })
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    });
            }
            Request::storage_key(skey) => {
                trace!("storage key info is {:?}", skey);
                let _ = serde_json::from_str::<BlockNumber>(&skey.height)
                    .map(|block_id| {
                        match command::state_at(
                            &self.command_req_sender,
                            &self.command_resp_receiver,
                            block_id.into(),
                        )
                        .and_then(|state| {
                            state
                                .storage_at(
                                    &Address::from(skey.get_address()),
                                    &H256::from(skey.get_position()),
                                )
                                .ok()
                        }) {
                            Some(storage_val) => {
                                response.set_storage_value(storage_val.to_vec());
                            }
                            None => {
                                response.set_code(ErrorCode::query_error());
                                response
                                    .set_error_msg("get storage at something failed".to_string());
                            }
                        }
                    })
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    });
            }

            _ => {
                error!("bad request msg!!!!");
            }
        };
        let msg: Message = response.into();
        self.response_mq(
            routing_key!(Executor >> Response).into(),
            msg.try_into().unwrap(),
        );
    }

    fn signal_to_chain(&self) {
        let mut state_signal = StateSignal::new();
        state_signal.set_height(self.get_current_height());
        let msg: Message = state_signal.into();
        self.response_mq(
            routing_key!(Executor >> StateSignal).into(),
            msg.try_into().unwrap(),
        );
    }

    fn get_current_height(&self) -> u64 {
        self.backlogs.get_current_height()
    }

    fn response_mq(&self, key: String, message: Vec<u8>) {
        trace!("send {} into RabbitMQ", key);
        self.mq_resp_sender.send((key, message));
    }
}

#[cfg(test)]
mod tests {
    use self::helpers::generate_executed_result;
    use super::*;
    use libproto::Message;
    use tests::helpers;

    #[test]
    fn test_bootstrap_broadcast_at_0th() {
        let mut postman = helpers::generate_postman(0, H256::from(0));
        let (mq_resp_sender, mq_resp_receiver) = crossbeam_channel::unbounded();
        let (command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
        let (command_resp_sender, command_resp_receiver) = crossbeam_channel::bounded(0);
        postman.mq_resp_sender = mq_resp_sender;
        postman.command_req_sender = command_req_sender;
        postman.command_resp_receiver = command_resp_receiver;

        ::std::thread::spawn(move || {
            let command = command_req_receiver.recv().unwrap();
            match command {
                command::Command::LoadExecutedResult(0) => command_resp_sender.send(
                    command::CommandResp::LoadExecutedResult(libproto::ExecutedResult::new()),
                ),
                _ => panic!("received should be Command::LoadExecutedResult(0)"),
            }
        });
        postman.bootstrap_broadcast();

        assert_eq!(0, postman.get_current_height());
        assert!(postman.backlogs.get_completed_result(0).is_some());
        assert!(postman.backlogs.get_completed_result(1).is_none());

        let (key, _message) = mq_resp_receiver.recv().unwrap();
        assert_eq!(
            routing_key!(Executor >> ExecutedResult),
            RoutingKey::from(key)
        );
    }

    #[test]
    fn test_bootstrap_broadcast_at_3th() {
        let mut postman = helpers::generate_postman(3, H256::from(0));
        let (mq_resp_sender, mq_resp_receiver) = crossbeam_channel::unbounded();
        let (command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
        let (command_resp_sender, command_resp_receiver) = crossbeam_channel::bounded(0);
        postman.mq_resp_sender = mq_resp_sender;
        postman.command_req_sender = command_req_sender;
        postman.command_resp_receiver = command_resp_receiver;

        ::std::thread::spawn(move || {
            let command = command_req_receiver.recv().unwrap();
            match command {
                command::Command::LoadExecutedResult(3) => command_resp_sender.send(
                    command::CommandResp::LoadExecutedResult(libproto::ExecutedResult::new()),
                ),
                _ => panic!("received should be Command::LoadExecutedResult(3)"),
            }
            let command = command_req_receiver.recv().unwrap();
            match command {
                command::Command::LoadExecutedResult(2) => command_resp_sender.send(
                    command::CommandResp::LoadExecutedResult(libproto::ExecutedResult::new()),
                ),
                _ => panic!("received should be Command::LoadExecutedResult(2)"),
            }
        });
        postman.bootstrap_broadcast();

        assert_eq!(3, postman.get_current_height());
        assert!(postman.backlogs.get_completed_result(0).is_none());
        assert!(postman.backlogs.get_completed_result(1).is_none());
        assert!(postman.backlogs.get_completed_result(2).is_some());
        assert!(postman.backlogs.get_completed_result(3).is_some());

        let (key, _message) = mq_resp_receiver.recv().unwrap();
        assert_eq!(
            routing_key!(Executor >> ExecutedResult),
            RoutingKey::from(key)
        );
    }

    #[test]
    fn test_priority_equal() {
        let current_height = 3;
        let parent_hash = H256::from(0);
        let current_hash = H256::from(0);
        let mut postman = helpers::generate_postman(current_height, current_hash);

        // generate 2 equal BlockWithProof but with different timestamp
        let mut block_with_proof =
            helpers::generate_block_with_proof(current_height + 1, parent_hash);
        block_with_proof.mut_blk().mut_header().set_timestamp(1);
        let message_a: Message = block_with_proof.clone().into();
        let message_b: Message = {
            block_with_proof.mut_blk().mut_header().set_timestamp(2);
            block_with_proof.into()
        };
        let routing_key = routing_key!(Consensus >> BlockWithProof).to_string();

        // give 2 BlockWithProof one by one
        assert_eq!(
            true,
            postman.update_backlog(&routing_key, message_a,),
            "handle first {} should be ok cause previous is None",
            routing_key,
        );
        assert_eq!(
            true,
            postman.update_backlog(&routing_key, message_b,),
            "handle second {} should be ok cause previous.priority = present.priority",
            routing_key,
        );

        let open_block = postman
            .backlogs
            .ready(current_height + 1)
            .expect("should return OpenBlock within BlockWithProof-B");
        assert_eq!(
            2,
            open_block.timestamp(),
            "block timestamp should be equal to BlockWithProof-B"
        );
    }

    #[test]
    fn test_priority_lower_then_higher() {
        let current_height = 3;
        let parent_hash = H256::from(0);
        let current_hash = H256::from(0);
        let mut postman = helpers::generate_postman(current_height, current_hash);

        // generate SignedProposal
        let mut signed_proposal =
            helpers::generate_signed_proposal(current_height + 1, parent_hash.clone());
        signed_proposal
            .mut_proposal()
            .mut_block()
            .mut_header()
            .set_timestamp(1);
        let message_a: Message = signed_proposal.into();
        let routing_key = routing_key!(Consensus >> SignedProposal).to_string();

        // give SignedProposal
        assert_eq!(
            true,
            postman.update_backlog(&routing_key, message_a,),
            "handle first {} should be ok cause previous is None",
            routing_key,
        );
        {
            let open_block = postman
                .backlogs
                .ready(current_height + 1)
                .expect("should return OpenBlock within SignedProposal-A");
            assert_eq!(
                1,
                open_block.timestamp(),
                "block timestamp should be equal to SignedProposal-A"
            );
        }

        // generate BlockWithProof
        let mut block_with_proof =
            helpers::generate_block_with_proof(current_height + 1, parent_hash);
        block_with_proof.mut_blk().mut_header().set_timestamp(2);
        let message_b: Message = block_with_proof.into();
        let routing_key = routing_key!(Consensus >> BlockWithProof).to_string();

        // give BlockWithProof
        assert_eq!(
            true,
            postman.update_backlog(&routing_key, message_b,),
            "handle second {} should be ok cause previous.priority < present.priority",
            routing_key,
        );

        let open_block = postman
            .backlogs
            .ready(current_height + 1)
            .expect("should return OpenBlock within BlockWithProof-B");
        assert_eq!(
            2,
            open_block.timestamp(),
            "block timestamp should be equal to BlockWithProof-B"
        );
    }

    #[test]
    fn test_priority_higher_then_lower() {
        let current_height = 3;
        let parent_hash = H256::from(0);
        let current_hash = H256::from(0);
        let mut postman = helpers::generate_postman(current_height, current_hash);

        // generate BlockWithProof
        let mut block_with_proof =
            helpers::generate_block_with_proof(current_height + 1, parent_hash);
        block_with_proof.mut_blk().mut_header().set_timestamp(1);
        let message_a: Message = block_with_proof.into();
        let routing_key = routing_key!(Consensus >> BlockWithProof).to_string();

        // give BlockWithProof
        assert_eq!(
            true,
            postman.update_backlog(&routing_key, message_a,),
            "handle first {} should be ok cause previous is None",
            routing_key,
        );

        {
            let open_block = postman
                .backlogs
                .ready(current_height + 1)
                .expect("should return OpenBlock within BlockWithProof-A");
            assert_eq!(
                1,
                open_block.timestamp(),
                "block timestamp should be equal to BlockWithProof-A"
            );
        }

        // generate SignedProposal
        let mut signed_proposal =
            helpers::generate_signed_proposal(current_height + 1, parent_hash.clone());
        signed_proposal
            .mut_proposal()
            .mut_block()
            .mut_header()
            .set_timestamp(2);
        let message_b: Message = signed_proposal.into();
        let routing_key = routing_key!(Consensus >> SignedProposal).to_string();

        // give SignedProposal
        assert_eq!(
            false,
            postman.update_backlog(&routing_key, message_b,),
            "raise error cause lower priority",
        );

        let open_block = postman
            .backlogs
            .ready(current_height + 1)
            .expect("should return OpenBlock within BlockWithProof-A");
        assert_eq!(
            1,
            open_block.timestamp(),
            "block timestamp should be equal to BlockWithProof-A"
        );
    }

    #[test]
    fn test_state_signal_chain_higher_executor() {
        let (mq_resp_sender, mq_resp_receiver) = crossbeam_channel::unbounded();
        let mut postman = helpers::generate_postman(2, Default::default());
        postman.mq_resp_sender = mq_resp_sender;

        // chain height = 5 >  executor height = 2
        let mut state_signal = StateSignal::new();
        state_signal.set_height(5);
        // mock the state signal chain send to executor
        let _ = postman.reply_chain_state_signal(&state_signal);

        let (key, msg_vec) = mq_resp_receiver.recv().unwrap();
        assert_eq!(routing_key!(Executor >> StateSignal), RoutingKey::from(key));
        let mut msg = Message::try_from(msg_vec).unwrap();
        let chain_state_signal: StateSignal = msg.take_state_signal().unwrap();
        let chain_height = chain_state_signal.get_height();
        assert_eq!(
            chain_height, 2,
            "mock chain will rececive executor's height, then sync local"
        );
    }

    #[test]
    fn test_state_signal_chain_lower_executor() {
        // Consider a situation:
        // q: when chain height < executor height, what executor should do?
        // ans: Executor will send the executed result of the corresponding higher height in backlog via (Executor >> ExecutedResult)
        let (mq_resp_sender, mq_resp_receiver) = crossbeam_channel::unbounded();
        let mut postman = helpers::generate_postman(5, Default::default());
        postman.mq_resp_sender = mq_resp_sender;

        let execute_result_3 = generate_executed_result(3);
        let execute_result_4 = generate_executed_result(4);
        let execute_result_5 = generate_executed_result(5);

        postman
            .backlogs
            .insert_completed_result(3, execute_result_3);
        postman
            .backlogs
            .insert_completed_result(4, execute_result_4);
        postman
            .backlogs
            .insert_completed_result(5, execute_result_5);

        // chain height = 2 < executor height = 5
        let mut state_signal = StateSignal::new();
        state_signal.set_height(2);
        let _ = postman.reply_chain_state_signal(&state_signal);

        // chain is lower than executor and have cached 3, 4, 5 executed result
        for i in 3..6 {
            let (key, msg_vec) = mq_resp_receiver.recv().unwrap();
            assert_eq!(
                routing_key!(Executor >> ExecutedResult),
                RoutingKey::from(key)
            );
            let mut msg = Message::try_from(msg_vec).unwrap();
            let execute_result: libproto::ExecutedResult = msg.take_executed_result().unwrap();
            assert_eq!(
                execute_result.get_executed_info().get_header().get_height(),
                i
            );
        }
    }

    #[test]
    fn test_state_signal_chain_lower_executor_without_cache() {
        // Consider another situation:
        // q: when chain height > executor height, it indicate executor has lose pace. how executor handle this situation?
        // ans: Executor will roll back to the chain height and restart work.
        let postman = helpers::generate_postman(5, Default::default());

        // chain height = 2 < executor height = 5, postman will roll back to chain height
        // just a uint test, more test about rolling back in other tests.
        let mut state_signal = StateSignal::new();
        state_signal.set_height(2);
        let res = postman.reply_chain_state_signal(&state_signal);

        assert_eq!(
            res.err(),
            Some(BlockId::Number(2)),
            "no executed result, executed should roll back"
        );
    }

    #[test]
    fn test_update_rich_status_with_prune() {
        let mut postman = helpers::generate_postman(6, Default::default());

        let execute_result_1 = generate_executed_result(1);
        let execute_result_2 = generate_executed_result(2);
        let execute_result_3 = generate_executed_result(3);
        let execute_result_4 = generate_executed_result(4);

        postman
            .backlogs
            .insert_completed_result(1, execute_result_1);
        postman
            .backlogs
            .insert_completed_result(2, execute_result_2);
        postman
            .backlogs
            .insert_completed_result(3, execute_result_3);
        postman
            .backlogs
            .insert_completed_result(4, execute_result_4);

        let mut rich_status = RichStatus::new();
        rich_status.set_height(2);
        // chain height = 2, executor height = 6
        // 3 + 2 < 6, executor backlogs will prune executed result which height <= 2
        postman.update_by_rich_status(&rich_status);

        assert!(postman.backlogs.get_completed_result(1).is_none());
        assert!(postman.backlogs.get_completed_result(2).is_none());
        assert!(postman.backlogs.get_completed_result(3).is_some());
        assert!(postman.backlogs.get_completed_result(4).is_some());
    }

    #[test]
    fn test_update_rich_status_without_prune() {
        let mut postman = helpers::generate_postman(5, Default::default());

        let execute_result_1 = generate_executed_result(1);
        let execute_result_2 = generate_executed_result(2);
        let execute_result_3 = generate_executed_result(3);
        let execute_result_4 = generate_executed_result(4);

        postman
            .backlogs
            .insert_completed_result(1, execute_result_1);
        postman
            .backlogs
            .insert_completed_result(2, execute_result_2);
        postman
            .backlogs
            .insert_completed_result(3, execute_result_3);
        postman
            .backlogs
            .insert_completed_result(4, execute_result_4);

        let mut rich_status = RichStatus::new();
        rich_status.set_height(2);
        // chain height = 2, executor height = 5
        // 3 + 2 = 5, not < 5, so executor backlogs will not prune
        postman.update_by_rich_status(&rich_status);

        assert!(postman.backlogs.get_completed_result(1).is_some());
        assert!(postman.backlogs.get_completed_result(2).is_some());
        assert!(postman.backlogs.get_completed_result(3).is_some());
    }
}
