// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::block::{ClosedBlock, ExecutedBlock, OpenBlock};
use super::economical_model::EconomicalModel;
use super::executor::Executor;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::large_enum_variant))]
pub enum StatusOfFSM {
    Initialize(OpenBlock),
    Pause(ExecutedBlock, usize),
    Execute(ExecutedBlock, usize),
    Finalize(ExecutedBlock),
}

impl std::fmt::Display for StatusOfFSM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            StatusOfFSM::Initialize(ref open_block) => write!(
                f,
                "StatusOfFSM::Initialize(height: {}, parent_hash: {:?}, timestamp: {})",
                open_block.number(),
                open_block.parent_hash(),
                open_block.timestamp(),
            ),
            StatusOfFSM::Pause(ref executed_block, index) => write!(
                f,
                "StatusOfFSM::Pause(height: {}, parent_hash: {:?}, state_root: {:?}, timestamp: {}, index: {})",
                executed_block.number(),
                executed_block.parent_hash(),
                executed_block.state_root,
                executed_block.timestamp(),
                index,
            ),
            StatusOfFSM::Execute(ref executed_block, index) => write!(
                f,
                "StatusOfFSM::Execute(height: {}, parent_hash: {:?}, state_root: {:?}, timestamp: {}, index: {})",
                executed_block.number(),
                executed_block.parent_hash(),
                executed_block.state_root,
                executed_block.timestamp(),
                index,
            ),
            StatusOfFSM::Finalize(ref executed_block) => write!(
                f,
                "StatusOfFSM::Finalize(height: {}, parent_hash: {:?}, state_root: {:?}, timestamp: {})",
                executed_block.number(),
                executed_block.parent_hash(),
                executed_block.state_root,
                executed_block.timestamp(),
            ),
        }
    }
}

pub trait FSM {
    fn into_fsm(&mut self, open_block: OpenBlock) -> ClosedBlock;
    fn fsm_initialize(&self, open_block: OpenBlock) -> StatusOfFSM;
    fn fsm_pause(&self, executed_block: ExecutedBlock, index: usize) -> StatusOfFSM;
    fn fsm_execute(&self, executed_block: ExecutedBlock, index: usize) -> StatusOfFSM;
    fn fsm_finalize(&self, executed_block: ExecutedBlock) -> ClosedBlock;
}

impl FSM for Executor {
    fn into_fsm(&mut self, open_block: OpenBlock) -> ClosedBlock {
        let mut status = StatusOfFSM::Initialize(open_block);
        loop {
            trace!("executor is at {}", status);
            status = match status {
                StatusOfFSM::Initialize(open_block) => self.fsm_initialize(open_block),
                StatusOfFSM::Pause(executed_block, index) => self.fsm_pause(executed_block, index),
                StatusOfFSM::Execute(executed_block, index) => {
                    self.fsm_execute(executed_block, index)
                }
                StatusOfFSM::Finalize(executed_block) => return self.fsm_finalize(executed_block),
            }
        }
    }

    fn fsm_initialize(&self, open_block: OpenBlock) -> StatusOfFSM {
        let executed_block = self.to_executed_block(open_block);
        StatusOfFSM::Pause(executed_block, 0)
    }

    fn fsm_pause(&self, executed_block: ExecutedBlock, index: usize) -> StatusOfFSM {
        match self.fsm_req_receiver.try_recv() {
            None => {
                if index == executed_block.body().transactions().len() {
                    StatusOfFSM::Finalize(executed_block)
                } else {
                    StatusOfFSM::Execute(executed_block, index + 1)
                }
            }
            Some(open_block) => {
                if executed_block.header().is_equivalent(&open_block.header()) {
                    StatusOfFSM::Pause(executed_block, index)
                } else {
                    StatusOfFSM::Initialize(open_block)
                }
            }
        }
    }

    fn fsm_execute(&self, mut executed_block: ExecutedBlock, index: usize) -> StatusOfFSM {
        let conf = self.sys_config.block_sys_config.clone();
        let mut transaction = executed_block.body().transactions[index - 1].clone();
        let quota_price = conf.quota_price;
        let economical_model: EconomicalModel = conf.economical_model;
        if economical_model == EconomicalModel::Charge {
            transaction.gas_price = quota_price;
        }

        executed_block.apply_transaction(&transaction, &self.sys_config);
        StatusOfFSM::Pause(executed_block, index)
    }

    fn fsm_finalize(&self, executed_block: ExecutedBlock) -> ClosedBlock {
        executed_block
            .state
            .borrow_mut()
            .commit()
            .expect("Commit state error.");
        executed_block.close(&(self.sys_config.block_sys_config))
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutedBlock;
    use crate::libexecutor::block::OpenBlock;
    use crate::libexecutor::executor::Executor;
    use crate::libexecutor::fsm::{StatusOfFSM, FSM};
    use crate::tests::helpers::{
        create_block, generate_block_body, generate_block_header, generate_contract, init_executor,
        init_executor2,
    };
    use cita_crypto::{CreateKey, KeyPair};
    use cita_types::Address;
    use std::thread;
    use std::time::Duration;

    fn generate_empty_block() -> OpenBlock {
        let block_body = generate_block_body();
        let mut block_header = generate_block_header();
        block_header.set_number(1);
        OpenBlock {
            body: block_body,
            header: block_header,
        }
    }

    fn generate_block(executor: &Executor, txs: u32) -> OpenBlock {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let data = generate_contract();
        create_block(&executor, Address::from(0), &data, (0, txs), &privkey)
    }

    // transit and commit state root
    fn transit(executor: &mut Executor, status: StatusOfFSM) -> StatusOfFSM {
        let new_status = match status {
            StatusOfFSM::Initialize(open_block) => executor.fsm_initialize(open_block),
            StatusOfFSM::Pause(executed_block, iter) => executor.fsm_pause(executed_block, iter),
            StatusOfFSM::Execute(executed_block, iter) => {
                executor.fsm_execute(executed_block, iter)
            }
            StatusOfFSM::Finalize(_executed_block) => unimplemented!(),
        };
        match new_status {
            StatusOfFSM::Initialize(open_block) => StatusOfFSM::Initialize(open_block),
            StatusOfFSM::Pause(executed_block, iter) => {
                executed_block
                    .state
                    .borrow_mut()
                    .commit()
                    .expect("commit state");
                StatusOfFSM::Pause(executed_block, iter)
            }
            StatusOfFSM::Execute(executed_block, iter) => {
                executed_block
                    .state
                    .borrow_mut()
                    .commit()
                    .expect("commit state");
                StatusOfFSM::Execute(executed_block, iter)
            }
            StatusOfFSM::Finalize(executed_block) => {
                executed_block
                    .state
                    .borrow_mut()
                    .commit()
                    .expect("commit state");
                StatusOfFSM::Finalize(executed_block)
            }
        }
    }

    fn transit_and_assert(
        executor: &mut Executor,
        status_from: StatusOfFSM,
        expect_to: StatusOfFSM,
    ) -> (StatusOfFSM, ExecutedBlock) {
        let status_to = transit(executor, status_from);
        assert_eq!(format!("{}", expect_to), format!("{}", status_to),);

        let executed_block = match expect_to {
            StatusOfFSM::Initialize(_open_block) => unimplemented!(),
            StatusOfFSM::Pause(executed_block, _iter) => executed_block,
            StatusOfFSM::Execute(executed_block, _iter) => executed_block,
            StatusOfFSM::Finalize(executed_block) => executed_block,
        };
        (status_to, executed_block)
    }

    #[test]
    fn test_fsm_initialize() {
        let executor = init_executor();
        let open_block = generate_empty_block();

        {
            let executed_block = executor.to_executed_block(open_block.clone());
            let status_after_init = executor.fsm_initialize(open_block.clone());
            assert_eq!(
                format!("{}", StatusOfFSM::Pause(executed_block, 0)),
                format!("{}", status_after_init)
            );
        }

        {
            let executed_block = executor.to_executed_block(open_block.clone());
            let executed_block_clone = executor.to_executed_block(open_block.clone());
            let status_after_pause_2 = executor.fsm_pause(executed_block, 2);
            assert_eq!(
                format!("{}", StatusOfFSM::Execute(executed_block_clone, 2 + 1)),
                format!("{}", status_after_pause_2)
            );
        }

        {
            let executed_block = executor.to_executed_block(open_block.clone());
            let executed_block_clone = executor.to_executed_block(open_block.clone());
            let status_after_pause_200 = executor.fsm_pause(executed_block, 200);
            assert_eq!(
                format!("{}", StatusOfFSM::Finalize(executed_block_clone)),
                format!("{}", status_after_pause_200)
            );
        }
    }

    #[test]
    fn test_fsm_pause_recv_diff_empty_block() {
        let (fsm_req_sender, fsm_req_receiver) = crossbeam_channel::unbounded();
        let (fsm_resp_sender, _fsm_resp_receiver) = crossbeam_channel::unbounded();
        let (_command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
        let (command_resp_sender, _command_resp_receiver) = crossbeam_channel::bounded(0);
        let executor = init_executor2(
            fsm_req_receiver.clone(),
            fsm_resp_sender,
            command_req_receiver,
            command_resp_sender,
        );
        let mut open_block = generate_empty_block();
        let executed_block = executor.to_executed_block(open_block.clone());

        thread::spawn(move || {
            let mut new_open_block = generate_empty_block();
            new_open_block.header.set_timestamp(2);
            // new_open_block is different from outside open_block
            fsm_req_sender.send(new_open_block);
        });
        ::std::thread::sleep(Duration::new(2, 0));
        let status_after_pause_2 = executor.fsm_pause(executed_block, 2);

        open_block.header.set_timestamp(2);

        assert_eq!(
            format!("{}", StatusOfFSM::Initialize(open_block)),
            format!("{}", status_after_pause_2)
        );
    }

    #[test]
    fn test_fsm_pause_recv_same_empty_block() {
        let (fsm_req_sender, fsm_req_receiver) = crossbeam_channel::unbounded();
        let (fsm_resp_sender, _fsm_resp_receiver) = crossbeam_channel::unbounded();
        let (_command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
        let (command_resp_sender, _command_resp_receiver) = crossbeam_channel::bounded(0);
        let executor = init_executor2(
            fsm_req_receiver.clone(),
            fsm_resp_sender,
            command_req_receiver,
            command_resp_sender,
        );
        let open_block = generate_empty_block();
        let executed_block = executor.to_executed_block(open_block.clone());
        let executed_block_clone = executor.to_executed_block(open_block.clone());

        thread::spawn(move || {
            let new_open_block = generate_empty_block();
            // new_open_block the same as outside open_block
            fsm_req_sender.send(new_open_block);
        });
        ::std::thread::sleep(Duration::new(2, 0));
        let status_after_pause_2 = executor.fsm_pause(executed_block, 2);

        assert_eq!(
            format!("{}", StatusOfFSM::Pause(executed_block_clone, 2)),
            format!("{}", status_after_pause_2)
        );
    }

    #[test]
    fn test_fsm_pause_recv_same_block() {
        let (fsm_req_sender, fsm_req_receiver) = crossbeam_channel::unbounded();
        let (fsm_resp_sender, _fsm_resp_receiver) = crossbeam_channel::unbounded();
        let (_command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
        let (command_resp_sender, _command_resp_receiver) = crossbeam_channel::bounded(0);
        let mut executor = init_executor2(
            fsm_req_receiver.clone(),
            fsm_resp_sender,
            command_req_receiver,
            command_resp_sender,
        );
        let open_block = generate_block(&executor, 2);

        // 1. init -> pause(0) -> execute(1) -> pause(1)
        let status_of_initialize = StatusOfFSM::Initialize(open_block.clone());
        let executed_block = executor.to_executed_block(open_block.clone());
        let (status_of_pause, executed_block) = transit_and_assert(
            &mut executor,
            status_of_initialize,
            StatusOfFSM::Pause(executed_block, 0),
        );
        let (status_of_execute_1th, mut executed_block) = transit_and_assert(
            &mut executor,
            status_of_pause,
            StatusOfFSM::Execute(executed_block, 1),
        );

        // 2. execute 1th transaction
        let transaction = executed_block.body().transactions[0].clone();
        executed_block.apply_transaction(&transaction, &executor.sys_config);
        executed_block
            .state
            .borrow_mut()
            .commit()
            .expect("commit state to re-calculate state root");
        let (status_of_pause_1th, mut executed_block) = transit_and_assert(
            &mut executor,
            status_of_execute_1th,
            StatusOfFSM::Pause(executed_block, 1),
        );

        // 3. send an equivalent OpenBlock into fsm_req channel
        let new_open_block = open_block.clone();
        fsm_req_sender.send(new_open_block);

        // 4. continue until finalize
        let transaction = executed_block.body().transactions[1].clone();
        executed_block.apply_transaction(&transaction, &executor.sys_config);
        executed_block
            .state
            .borrow_mut()
            .commit()
            .expect("commit state to re-calculate state root");
        let mut status = status_of_pause_1th;
        loop {
            status = match status {
                StatusOfFSM::Finalize(_) => {
                    assert_eq!(
                        format!("{}", status),
                        format!("{}", StatusOfFSM::Finalize(executed_block)),
                    );
                    break;
                }
                _ => transit(&mut executor, status),
            };
        }
    }

    #[test]
    fn test_fsm_pause_recv_diff_block() {
        let (fsm_req_sender, fsm_req_receiver) = crossbeam_channel::unbounded();
        let (fsm_resp_sender, _fsm_resp_receiver) = crossbeam_channel::unbounded();
        let (_command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
        let (command_resp_sender, _command_resp_receiver) = crossbeam_channel::bounded(0);
        let mut executor = init_executor2(
            fsm_req_receiver.clone(),
            fsm_resp_sender,
            command_req_receiver,
            command_resp_sender,
        );
        let open_block = generate_block(&executor, 2);

        // 1. init -> pause(0) -> execute(1) -> pause(1)
        let status_of_initialize = StatusOfFSM::Initialize(open_block.clone());
        let status_of_pause = transit(&mut executor, status_of_initialize);
        let status_of_execute = transit(&mut executor, status_of_pause);
        let status_of_pause = transit(&mut executor, status_of_execute);

        // 3. send an un-equivalent OpenBlock into fsm_req channel
        let new_open_block = generate_block(&executor, 10);
        fsm_req_sender.send(new_open_block.clone());

        // 4. continue until finalize
        let mut executed_block = executor.to_executed_block(new_open_block);
        let mut transactions = { executed_block.body.transactions.clone() };
        for transaction in transactions.iter_mut() {
            // let mut t = transaction.clone();
            executed_block.apply_transaction(&transaction, &executor.sys_config);
        }
        executed_block
            .state
            .borrow_mut()
            .commit()
            .expect("commit state to re-calculate state root");
        let mut status = status_of_pause;
        loop {
            status = match status {
                StatusOfFSM::Finalize(_) => {
                    assert_eq!(
                        format!("{}", status),
                        format!("{}", StatusOfFSM::Finalize(executed_block)),
                    );
                    break;
                }
                _ => transit(&mut executor, status),
            };
        }
    }
}
