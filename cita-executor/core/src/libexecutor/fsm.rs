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

use super::block::{ClosedBlock, ExecutedBlock, OpenBlock};
use super::economical_model::EconomicalModel;
use super::executor::{CheckOptions, Executor};
use contracts::solc::PriceManagement;
use libexecutor::ExecutedResult;
use types::ids::BlockId;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::large_enum_variant))]
pub enum StatusOfFSM {
    Initialize(OpenBlock),
    Pause(ExecutedBlock, usize),
    Execute(ExecutedBlock, usize),
    Finalize(ExecutedBlock),
    Reply(ClosedBlock, ExecutedResult),
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
            StatusOfFSM::Pause(ref executed_block, iter) => write!(
                f,
                "StatusOfFSM::Pause(height: {}, parent_hash: {:?}, timestamp: {}, iter: {})",
                executed_block.number(),
                executed_block.parent_hash(),
                executed_block.timestamp(),
                iter,
            ),
            StatusOfFSM::Execute(ref executed_block, iter) => write!(
                f,
                "StatusOfFSM::Execute(height: {}, parent_hash: {:?}, timestamp: {}, iter: {})",
                executed_block.number(),
                executed_block.parent_hash(),
                executed_block.timestamp(),
                iter,
            ),
            StatusOfFSM::Finalize(ref executed_block) => write!(
                f,
                "StatusOfFSM::Finalize(height: {}, parent_hash: {:?}, timestamp: {})",
                executed_block.number(),
                executed_block.parent_hash(),
                executed_block.timestamp(),
            ),
            StatusOfFSM::Reply(ref closed_block, _) => write!(
                f,
                "StatusOfFSM::Reply(height: {}, parent_hash: {:?}, timestamp: {}, tranaction_root: {:?})",
                closed_block.number(),
                closed_block.parent_hash(),
                closed_block.timestamp(),
                closed_block.transactions_root(),
            ),
        }
    }
}

pub trait FSM {
    fn into_fsm(&mut self, open_block: OpenBlock) -> (ClosedBlock, ExecutedResult);
    fn fsm_initialize(&self, open_block: OpenBlock) -> StatusOfFSM;
    fn fsm_pause(&self, executed_block: ExecutedBlock, iter: usize) -> StatusOfFSM;
    fn fsm_execute(&self, executed_block: ExecutedBlock, iter: usize) -> StatusOfFSM;
    fn fsm_finalize(&self, executed_block: ExecutedBlock) -> StatusOfFSM;
}

impl FSM for Executor {
    fn into_fsm(&mut self, open_block: OpenBlock) -> (ClosedBlock, ExecutedResult) {
        let mut status = StatusOfFSM::Initialize(open_block);
        loop {
            trace!("executor is at {}", status);
            status = match status {
                StatusOfFSM::Initialize(open_block) => self.fsm_initialize(open_block),
                StatusOfFSM::Pause(executed_block, iter) => self.fsm_pause(executed_block, iter),
                StatusOfFSM::Execute(executed_block, iter) => {
                    self.fsm_execute(executed_block, iter)
                }
                StatusOfFSM::Finalize(executed_block) => self.fsm_finalize(executed_block),
                StatusOfFSM::Reply(closed_block, executed_result) => {
                    return (closed_block, executed_result)
                }
            }
        }
    }

    fn fsm_initialize(&self, open_block: OpenBlock) -> StatusOfFSM {
        let executed_block = self.to_executed_block(open_block);
        StatusOfFSM::Pause(executed_block, 0)
    }

    fn fsm_pause(&self, executed_block: ExecutedBlock, iter: usize) -> StatusOfFSM {
        match self.fsm_req_receiver.try_recv() {
            None => {
                if iter == executed_block.body().transactions().len() {
                    StatusOfFSM::Finalize(executed_block)
                } else {
                    StatusOfFSM::Execute(executed_block, iter + 1)
                }
            }
            Some(open_block) => {
                if executed_block.header().is_equivalent(&open_block.header()) {
                    let new_executed_block = self.to_executed_block(open_block);
                    let status = StatusOfFSM::Pause(new_executed_block, iter);
                    trace!("executor receive equivalent block: {}", status);
                    status
                } else {
                    StatusOfFSM::Initialize(open_block)
                }
            }
        }
    }

    fn fsm_execute(&self, mut executed_block: ExecutedBlock, iter: usize) -> StatusOfFSM {
        let conf = self.sys_config.clone();
        // FIXME move into Self for performance
        let check_options = CheckOptions {
            permission: conf.check_permission,
            quota: conf.check_quota,
            fee_back_platform: conf.check_fee_back_platform,
            send_tx_permission: conf.check_send_tx_permission,
            create_contract_permission: conf.check_create_contract_permission,
        };

        let mut transaction = executed_block.body().transactions[iter - 1].clone();
        let quota_price = PriceManagement::new(self)
            .quota_price(BlockId::Pending)
            .unwrap_or_else(PriceManagement::default_quota_price);
        let economical_model: EconomicalModel = *self.economical_model.read();
        if economical_model == EconomicalModel::Charge {
            transaction.gas_price = quota_price;
        }

        executed_block.apply_transaction(
            &*self.engine,
            &transaction,
            *self.economical_model.read(),
            &check_options,
        );

        StatusOfFSM::Pause(executed_block, iter)
    }

    fn fsm_finalize(&self, mut executed_block: ExecutedBlock) -> StatusOfFSM {
        // commit changed-accounts into trie structure
        executed_block
            .state
            .commit()
            .expect("failed to commit state trie");

        let closed_block = executed_block.close();
        let executed_result = self.make_executed_result(&closed_block);
        StatusOfFSM::Reply(closed_block, executed_result)
    }
}

#[cfg(test)]
mod tests {

    use libexecutor::fsm::{StatusOfFSM, FSM};
    use std::thread;
    use std::time::Duration;
    use tests::helpers;
    use types::block::{BlockBody, OpenBlock};
    use types::header::OpenHeader;
    use types::transaction::SignedTransaction;

    fn generate_block_body() -> BlockBody {
        let mut stx = SignedTransaction::default();
        stx.data = vec![1; 200];
        let transactions = vec![stx; 200];
        BlockBody { transactions }
    }

    fn generate_block_header() -> OpenHeader {
        OpenHeader::default()
    }

    fn generate_block() -> OpenBlock {
        let block_body = generate_block_body();
        let block_header = generate_block_header();
        OpenBlock {
            body: block_body,
            header: block_header,
        }
    }

    #[test]
    fn test_fsm_initialize() {
        let executor = helpers::init_executor(vec![]);
        let open_block = generate_block();

        let executed_block = executor.to_executed_block(open_block.clone());
        let status_after_init = executor.fsm_initialize(open_block.clone());
        assert_eq!(
            format!("{}", StatusOfFSM::Pause(executed_block.clone(), 0)),
            format!("{}", status_after_init)
        );

        let status_after_pause_2 = executor.fsm_pause(executed_block.clone(), 2);
        assert_eq!(
            format!("{}", StatusOfFSM::Execute(executed_block.clone(), 2 + 1)),
            format!("{}", status_after_pause_2)
        );

        let status_after_pause_200 = executor.fsm_pause(executed_block.clone(), 200);
        assert_eq!(
            format!("{}", StatusOfFSM::Finalize(executed_block.clone())),
            format!("{}", status_after_pause_200)
        );
    }

    #[test]
    fn test_fsm_pause_recv_diff_block() {
        let (fsm_req_sender, fsm_req_receiver) = crossbeam_channel::unbounded();
        let (fsm_resp_sender, _fsm_resp_receiver) = crossbeam_channel::unbounded();
        let (_command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
        let (command_resp_sender, _command_resp_receiver) = crossbeam_channel::bounded(0);
        let executor = helpers::init_executor2(
            vec![],
            fsm_req_receiver.clone(),
            fsm_resp_sender,
            command_req_receiver,
            command_resp_sender,
        );
        let mut open_block = generate_block();
        let executed_block = executor.to_executed_block(open_block.clone());

        thread::spawn(move || {
            let mut new_open_block = generate_block();
            new_open_block.header.set_timestamp(2);
            // new_open_block is different from outside open_block
            fsm_req_sender.send(new_open_block);
        });
        ::std::thread::sleep(Duration::new(2, 0));
        let status_after_pause_2 = executor.fsm_pause(executed_block.clone(), 2);

        open_block.header.set_timestamp(2);

        assert_eq!(
            format!("{}", StatusOfFSM::Initialize(open_block)),
            format!("{}", status_after_pause_2)
        );
    }

     #[test]
    fn test_fsm_pause_recv_same_block() {
        let (fsm_req_sender, fsm_req_receiver) = crossbeam_channel::unbounded();
        let (fsm_resp_sender, _fsm_resp_receiver) = crossbeam_channel::unbounded();
        let (_command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
        let (command_resp_sender, _command_resp_receiver) = crossbeam_channel::bounded(0);
        let executor = helpers::init_executor2(
            vec![],
            fsm_req_receiver.clone(),
            fsm_resp_sender,
            command_req_receiver,
            command_resp_sender,
        );
        let mut open_block = generate_block();
        let executed_block = executor.to_executed_block(open_block.clone());

        thread::spawn(move || {
            let new_open_block = generate_block();
            // new_open_block the same as outside open_block
            fsm_req_sender.send(new_open_block);
        });
        ::std::thread::sleep(Duration::new(2, 0));
        let status_after_pause_2 = executor.fsm_pause(executed_block.clone(), 2);

        assert_eq!(
            format!("{}", StatusOfFSM::Pause(executed_block, 2)),
            format!("{}", status_after_pause_2)
        );
    }
}
