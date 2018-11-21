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

use cita_types::{Address, H256};
use core::contracts::solc::sys_config::ChainId;
use core::libexecutor::block::{ClosedBlock, OpenBlock};
use core::libexecutor::call_request::CallRequest;
use crossbeam_channel::{Receiver, Sender};
use error::ErrorCode;
use jsonrpc_types::rpctypes::{BlockNumber, CountOrCode};
use libproto::auth::Miscellaneous;
use libproto::blockchain::{RichStatus, StateSignal};
use libproto::request::Request_oneof_req as Request;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::{request, response, ConsensusConfig, ExecutedResult, Message};
use proof::BftProof;
use serde_json;
use std::convert::{Into, TryFrom, TryInto};
use std::u8;
use types::ids::BlockId;

use core::libexecutor::command;

use super::backlogs::Backlogs;

pub struct Postman {
    backlogs: Backlogs,
    mq_req_receiver: Receiver<(String, Vec<u8>)>,
    mq_resp_sender: Sender<(String, Vec<u8>)>,
    fsm_req_sender: Sender<OpenBlock>,
    fsm_resp_receiver: Receiver<(ClosedBlock, ExecutedResult)>,
    command_req_sender: Sender<command::Command>,
    command_resp_receiver: Receiver<command::CommandResp>,
}

impl Postman {
    pub fn new(
        current_height: u64,
        mq_req_receiver: Receiver<(String, Vec<u8>)>,
        mq_resp_sender: Sender<(String, Vec<u8>)>,
        fsm_req_sender: Sender<OpenBlock>,
        fsm_resp_receiver: Receiver<(ClosedBlock, ExecutedResult)>,
        command_req_sender: Sender<command::Command>,
        command_resp_receiver: Receiver<command::CommandResp>,
    ) -> Self {
        Postman {
            backlogs: Backlogs::new(current_height),
            mq_req_receiver,
            mq_resp_sender,
            fsm_req_sender,
            fsm_resp_receiver,
            command_req_sender,
            command_resp_receiver,
        }
    }

    pub fn do_loop(&mut self) {
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
                (None, Some((closed_block, executed_result))) => {
                    self.handle_fsm_response(closed_block, executed_result);
                    self.execute_next_block();
                }
            }
        }
    }

    // call this function every times Executor start/restart, to broadcast current state
    // to cita-chain. This broadcast state only contains system config but not block data,
    // cita-chain would deal with it.
    pub fn bootstrap_broadcast(&self, consensus_config: ConsensusConfig) {
        let mut executed_result = ExecutedResult::new();
        executed_result.set_config(consensus_config);
        let msg: Message = executed_result.into();
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

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
    fn recv(
        &self,
    ) -> (
        Option<(String, Vec<u8>)>,
        Option<(ClosedBlock, ExecutedResult)>,
    ) {
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

    // update executed result into backlogs based on arrived result from executor, and process
    // the next height if possible
    fn handle_fsm_response(&mut self, closed_block: ClosedBlock, executed_result: ExecutedResult) {
        let height = closed_block.number();
        info!("postman receive {}-th ClosedBlock from executor", height);
        self.backlogs
            .insert_result(height, closed_block, executed_result);
        self.maybe_grow_up();
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
                self.maybe_grow_up();
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
            warn!("cita-chain(height={}) is lagging behind cita-executor(height={}), gonna roll back to {}", height, self.get_current_height(), height - 1);
            return Err(BlockId::Number(height - 1));
        }

        trace!("send {}-th ExecutedResult", height);
        let executed_result = executed_result.unwrap();
        let msg: Message = executed_result.into();
        self.response_mq(
            routing_key!(Executor >> ExecutedResult).into(),
            msg.try_into().unwrap(),
        );
        Ok(())
    }

    // TODO: check is_dup_block, hash, height
    fn update_backlog(&mut self, key: &str, mut msg: Message) {
        match RoutingKey::from(key) {
            // Proposal{proof: None, block: {body: Some, proof: None}}
            routing_key!(Consensus >> SignedProposal) | routing_key!(Net >> SignedProposal) => {
                let mut proposal = msg.take_signed_proposal().unwrap();
                let open_block = OpenBlock::from(proposal.take_proposal().take_block());
                let block_height = wrap_height(open_block.number() as usize);

                trace!("insert {}-th Proposal into backlog", block_height);
                self.backlogs.insert_open_block(block_height, open_block);
            }

            // BlockWithProof{proof: Some, block: {body: Some, proof: None}}
            routing_key!(Consensus >> BlockWithProof) => {
                let mut proofed = msg.take_block_with_proof().unwrap();
                let open_block = OpenBlock::from(proofed.take_blk());
                let proof = BftProof::from(proofed.take_proof());
                let proof_height = wrap_height(proof.height);
                let block_height = wrap_height(open_block.number() as usize);

                trace!("insert {}-th Proofed into backlog", block_height);
                self.backlogs.insert_open_block(block_height, open_block);
                self.backlogs.insert_proof(proof_height, proof);
            }

            // SyncBlock{proof: None, block: {body: Some, proof: Some}}
            routing_key!(Net >> SyncResponse) | routing_key!(Chain >> LocalSync) => {
                let mut sync_res = msg.take_sync_response().unwrap();
                for proto_block in sync_res.take_blocks().into_iter() {
                    let open_block = OpenBlock::from(proto_block);
                    let proof = BftProof::from(open_block.proof().clone());
                    let proof_height = wrap_height(proof.height);
                    let block_height = wrap_height(open_block.number() as usize);

                    trace!("insert {}-th Sync into backlog", block_height);
                    self.backlogs.insert_open_block(block_height, open_block);
                    self.backlogs.insert_proof(proof_height, proof);
                }
            }
            _ => unimplemented!(),
        }
    }

    /// Grow up if current block executed completely,
    /// 1. Update backlogs
    /// 2. Notify executor to grow up too
    /// 3. Delivery rich status of new height
    fn maybe_grow_up(&mut self) {
        let next_height = self.get_current_height() + 1;
        if self.backlogs.is_completed(next_height) {
            let backlog = self.backlogs.complete(next_height);
            let closed_block = backlog.clone_closed_block();

            // make sure executor grow up first
            trace!("postman notice executor to grow up to {}", next_height,);
            command::grow(
                &self.command_req_sender,
                &self.command_resp_receiver,
                closed_block,
            );

            self.send_executed_info_to_chain(next_height).unwrap();
            // FIXME self.pub_black_list(&closed_block, ctx_pub);
        }
    }

    fn execute_next_block(&mut self) {
        let next_height = self.get_current_height() + 1;
        if let Some(block) = self.backlogs.get_open_block(next_height) {
            if let Some(closed_block) = self.backlogs.get_closed_block(next_height) {
                if closed_block.is_equivalent(&block) {
                    return;
                }
            }
            trace!("postman send {}-th block to executor", block.number());
            self.fsm_req_sender.send(block);
        }
    }

    fn update_by_rich_status(&mut self, rich_status: &RichStatus) {
        let next_height = rich_status.get_height() + 1;
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

// System Convention: 0-th block's proof is `::std::usize::MAX`
fn wrap_height(height: usize) -> u64 {
    match height {
        ::std::usize::MAX => 0,
        _ => height as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_height() {
        assert_eq!(0, wrap_height(::std::usize::MAX));
        assert_eq!(
            ::std::usize::MAX as u64 - 1,
            wrap_height(::std::usize::MAX - 1)
        );
        assert_eq!(2, wrap_height(2));
    }
}
