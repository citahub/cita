// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Transaction Execution environment.


////////////////////////////////////////////////////////////////////////////////

use action_params::{ActionParams, ActionValue};
use engines::Engine;
use env_info::EnvInfo;
use evm::{self, MessageCallResult, Schedule, Factory};
use executed::CallType;
use executive::*;
use native::Factory as NativeFactory;
use state::State;
use state::backend::Backend as StateBackend;
use std::cmp;
use std::sync::Arc;
use substate::Substate;
use trace::{Tracer, VMTracer};
use util::*;
/// Policy for handling output data on `RETURN` opcode.
pub enum OutputPolicy<'a, 'b> {
    /// Return reference to fixed sized output.
    /// Used for message calls.
    Return(BytesRef<'a>, Option<&'b mut Bytes>),
    /// Init new contract as soon as `RETURN` is called.
    InitContract(Option<&'b mut Bytes>),
}

/// Transaction properties that externalities need to know about.
pub struct OriginInfo {
    address: Address,
    origin: Address,
    gas_price: U256,
    value: U256,
}

impl OriginInfo {
    /// Populates origin info from action params.
    pub fn from(params: &ActionParams) -> Self {
        OriginInfo {
            address: params.address.clone(),
            origin: params.origin.clone(),
            gas_price: params.gas_price,
            value: match params.value {
                ActionValue::Transfer(val) |
                ActionValue::Apparent(val) => val,
            },
        }
    }
}

/// Implementation of evm Externalities.
pub struct Externalities<'a, T: 'a, V: 'a, B: 'a>
where
    T: Tracer,
    V: VMTracer,
    B: StateBackend,
{
    state: &'a mut State<B>,
    env_info: &'a EnvInfo,
    engine: &'a Engine,
    vm_factory: &'a Factory,
    native_factory: &'a NativeFactory,
    depth: usize,
    origin_info: OriginInfo,
    substate: &'a mut Substate,
    schedule: Schedule,
    output: OutputPolicy<'a, 'a>,
    tracer: &'a mut T,
    vm_tracer: &'a mut V,
}


impl<'a, T: 'a, V: 'a, B: 'a> Externalities<'a, T, V, B>
where
    T: Tracer,
    V: VMTracer,
    B: StateBackend,
{
    /// Basic `Externalities` constructor.
    #[cfg_attr(feature = "dev", allow(too_many_arguments))]
    pub fn new(state: &'a mut State<B>, env_info: &'a EnvInfo, engine: &'a Engine, vm_factory: &'a Factory, native_factory: &'a NativeFactory, depth: usize, origin_info: OriginInfo, substate: &'a mut Substate, output: OutputPolicy<'a, 'a>, tracer: &'a mut T, vm_tracer: &'a mut V) -> Self {
        Externalities {
            state: state,
            env_info: env_info,
            engine: engine,
            vm_factory: vm_factory,
            native_factory: native_factory,
            depth: depth,
            origin_info: origin_info,
            substate: substate,
            schedule: Schedule::new_frontier(),
            output: output,
            tracer: tracer,
            vm_tracer: vm_tracer,
        }
    }
}

impl<'a, T: 'a, V: 'a, B: 'a> evm::Ext for Externalities<'a, T, V, B>
where
    T: Tracer,
    V: VMTracer,
    B: StateBackend,
{
    fn storage_at(&self, key: &H256) -> trie::Result<H256> {
        self.state.storage_at(&self.origin_info.address, key)
    }

    fn set_storage(&mut self, key: H256, value: H256) -> trie::Result<()> {
        self.state.set_storage(&self.origin_info.address, key, value)
    }

    fn exists(&self, address: &Address) -> trie::Result<bool> {
        self.state.exists(address)
    }

    fn exists_and_not_null(&self, address: &Address) -> trie::Result<bool> {
        self.state.exists_and_not_null(address)
    }

    fn origin_balance(&self) -> trie::Result<U256> {
        self.balance(&self.origin_info.address)
    }

    #[allow(unused_variables)]
    fn balance(&self, address: &Address) -> trie::Result<U256> {
        Ok(U256::zero())
        //self.state.balance(address)
    }

    fn blockhash(&self, number: &U256) -> H256 {
        // TODO: comment out what this function expects from env_info, since it will produce panics if the latter is inconsistent
        match *number < U256::from(self.env_info.number) && number.low_u64() >= cmp::max(256, self.env_info.number) - 256 {
            true => {
                let index = self.env_info.number - number.low_u64() - 1;
                assert!(index < self.env_info.last_hashes.len() as u64, format!("Inconsistent env_info, should contain at least {:?} last hashes", index + 1));
                let r = self.env_info.last_hashes[index as usize].clone();
                trace!("ext: blockhash({}) -> {} self.env_info.number={}\n", number, r, self.env_info.number);
                r
            }
            false => {
                trace!("ext: blockhash({}) -> null self.env_info.number={}\n", number, self.env_info.number);
                H256::zero()
            }
        }
    }

    fn create(&mut self, gas: &U256, value: &U256, code: &[u8]) -> evm::ContractCreateResult {
        // create new contract address
        let address = match self.state.nonce(&self.origin_info.address) {
            Ok(nonce) => contract_address(&self.origin_info.address, &nonce),
            Err(e) => {
                debug!(target: "ext", "Database corruption encountered: {:?}", e);
                return evm::ContractCreateResult::Failed;
            }
        };

        // prepare the params
        let params = ActionParams {
            code_address: address.clone(),
            address: address.clone(),
            sender: self.origin_info.address.clone(),
            origin: self.origin_info.origin.clone(),
            gas: *gas,
            gas_price: self.origin_info.gas_price,
            value: ActionValue::Transfer(*value),
            code: Some(Arc::new(code.to_vec())),
            code_hash: code.crypt_hash(),
            data: None,
            call_type: CallType::None,
        };

        if let Err(e) = self.state.inc_nonce(&self.origin_info.address) {
            debug!(target: "ext", "Database corruption encountered: {:?}", e);
            return evm::ContractCreateResult::Failed;
        }
        let mut ex = Executive::from_parent(self.state, self.env_info, self.engine, self.vm_factory, self.native_factory, self.depth);

        // TODO: handle internal error separately
        match ex.create(params, self.substate, self.tracer, self.vm_tracer) {
            Ok(gas_left) => {
                self.substate.contracts_created.push(address.clone());
                evm::ContractCreateResult::Created(address, gas_left)
            }
            _ => evm::ContractCreateResult::Failed,
        }
    }

    fn call(&mut self, gas: &U256, sender_address: &Address, receive_address: &Address, value: Option<U256>, data: &[u8], code_address: &Address, output: &mut [u8], call_type: CallType) -> MessageCallResult {
        trace!(target: "externalities", "call");

        let code_res = self.state
                           .code(code_address)
                           .and_then(|code| self.state.code_hash(code_address).map(|hash| (code, hash)));

        let (code, code_hash) = match code_res {
            Ok((code, hash)) => (code, hash),
            Err(_) => return MessageCallResult::Failed,
        };

        let mut params = ActionParams {
            sender: sender_address.clone(),
            address: receive_address.clone(),
            value: ActionValue::Apparent(self.origin_info.value),
            code_address: code_address.clone(),
            origin: self.origin_info.origin.clone(),
            gas: *gas,
            gas_price: self.origin_info.gas_price,
            code: code,
            code_hash: code_hash,
            data: Some(data.to_vec()),
            call_type: call_type,
        };

        if let Some(value) = value {
            params.value = ActionValue::Transfer(value);
        }

        let mut ex = Executive::from_parent(self.state, self.env_info, self.engine, self.vm_factory, self.native_factory, self.depth);

        match ex.call(params, self.substate, BytesRef::Fixed(output), self.tracer, self.vm_tracer) {
            Ok(gas_left) => MessageCallResult::Success(gas_left),
            _ => MessageCallResult::Failed,
        }
    }

    fn extcode(&self, address: &Address) -> trie::Result<Arc<Bytes>> {
        Ok(self.state.code(address)?.unwrap_or_else(|| Arc::new(vec![])))
    }

    fn extcodesize(&self, address: &Address) -> trie::Result<usize> {
        Ok(self.state.code_size(address)?.unwrap_or(0))
    }

    #[cfg_attr(feature = "dev", allow(match_ref_pats))]
    fn ret(mut self, gas: &U256, data: &[u8]) -> evm::Result<U256>
    where
        Self: Sized,
    {
        trace!("ret gas={}, data={:?}", gas, data);
        let handle_copy = |to: &mut Option<&mut Bytes>| { to.as_mut().map(|b| **b = data.to_owned()); };
        match self.output {
            OutputPolicy::Return(BytesRef::Fixed(ref mut slice), ref mut copy) => {
                handle_copy(copy);

                let len = cmp::min(slice.len(), data.len());
                (&mut slice[..len]).copy_from_slice(&data[..len]);
                Ok(*gas)
            }
            OutputPolicy::Return(BytesRef::Flexible(ref mut vec), ref mut copy) => {
                handle_copy(copy);

                vec.clear();
                vec.extend_from_slice(data);
                Ok(*gas)
            }
            OutputPolicy::InitContract(ref mut copy) => {
                let return_cost = U256::from(data.len()) * U256::from(self.schedule.create_data_gas);
                if return_cost > *gas || data.len() > self.schedule.create_data_limit {
                    return match self.schedule.exceptional_failed_code_deposit {
                        true => Err(evm::Error::OutOfGas),
                        false => Ok(*gas),
                    };
                }

                handle_copy(copy);

                self.state.init_code(&self.origin_info.address, data.to_vec())?;
                Ok(*gas - return_cost)
            }
        }
    }

    fn log(&mut self, topics: Vec<H256>, data: &[u8]) {
        use log_entry::LogEntry;

        let address = self.origin_info.address.clone();
        self.substate.logs.push(LogEntry {
                                    address: address,
                                    topics: topics,
                                    data: data.to_vec(),
                                });
    }

    fn suicide(&mut self, refund_address: &Address) -> trie::Result<()> {
        let address = self.origin_info.address.clone();
        let balance = self.balance(&address)?;
        // if &address == refund_address {
        //     // TODO [todr] To be consistent with CPP client we set balance to 0 in that case.
        //     self.state.sub_balance(&address, &balance)?;
        // } else {
        //     trace!(target: "ext", "Suiciding {} -> {} (xfer: {})", address, refund_address, balance);
        //     self.state
        //         .transfer_balance(&address,
        //                           refund_address,
        //                           &balance,
        //                           self.substate.to_cleanup_mode(&self.schedule))?;
        // }

        self.tracer.trace_suicide(address, balance, refund_address.clone());
        self.substate.suicides.insert(address);

        Ok(())
    }

    fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    fn env_info(&self) -> &EnvInfo {
        self.env_info
    }

    fn depth(&self) -> usize {
        self.depth
    }

    fn inc_sstore_clears(&mut self) {
        self.substate.sstore_clears_count = self.substate.sstore_clears_count + U256::one();
    }

    fn trace_prepare_execute(&mut self, pc: usize, instruction: u8, gas_cost: &U256) -> bool {
        self.vm_tracer.trace_prepare_execute(pc, instruction, gas_cost)
    }

    fn trace_executed(&mut self, gas_used: U256, stack_push: &[U256], mem_diff: Option<(usize, &[u8])>, store_diff: Option<(U256, U256)>) {
        self.vm_tracer.trace_executed(gas_used, stack_push, mem_diff, store_diff)
    }
}
