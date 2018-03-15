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

use action_params::{ActionParams, ActionValue};
use contracts::permission_management::contains_resource;
use crossbeam;
use engines::Engine;
use env_info::EnvInfo;
use error::ExecutionError;
use ethcore_io as io;
use evm::{self, Factory, FinalizationResult, Finalize, ReturnData, Schedule};
pub use executed::{Executed, ExecutionResult};
use executed::CallType;
use externalities::*;
use native::Factory as NativeFactory;
use state::{State, Substate};
use state::backend::Backend as StateBackend;
use std::cmp;
use std::sync::Arc;
use trace::{ExecutiveTracer, ExecutiveVMTracer, FlatTrace, NoopTracer, NoopVMTracer, Tracer, VMTrace, VMTracer};
use types::transaction::{Action, SignedTransaction};
use util::*;

/// Roughly estimate what stack size each level of evm depth will use
/// TODO [todr] We probably need some more sophisticated calculations here
///      (limit on my machine 132)
/// Maybe something like here:
/// `https://github.com/ethereum/libethereum/blob/4db169b8504f2b87f7d5a481819cfb959fc65f6c/libethereum/ExtVM.cpp`
const STACK_SIZE_PER_DEPTH: usize = 24 * 1024;

/// Returns new address created from address and given nonce.
pub fn contract_address(address: &Address, nonce: &U256) -> Address {
    use rlp::RlpStream;

    let mut stream = RlpStream::new_list(2);
    stream.append(address);
    stream.append(nonce);
    From::from(stream.out().crypt_hash())
}

/// Transaction execution options.
#[derive(Default, Copy, Clone, PartialEq)]
pub struct TransactOptions {
    /// Enable call tracing.
    pub tracing: bool,
    /// Enable VM tracing.
    pub vm_tracing: bool,
    /// Check permission before execution.
    pub check_permission: bool,
    /// Check account gas limit
    pub check_quota: bool,
}

/// Transaction executor.
pub struct Executive<'a, B: 'a + StateBackend> {
    state: &'a mut State<B>,
    info: &'a EnvInfo,
    engine: &'a Engine,
    vm_factory: &'a Factory,
    depth: usize,
    static_flag: bool,
    native_factory: &'a NativeFactory,
}

impl<'a, B: 'a + StateBackend> Executive<'a, B> {
    /// Basic constructor.
    pub fn new(
        state: &'a mut State<B>,
        info: &'a EnvInfo,
        engine: &'a Engine,
        vm_factory: &'a Factory,
        native_factory: &'a NativeFactory,
    ) -> Self {
        Executive {
            state: state,
            info: info,
            engine: engine,
            vm_factory: vm_factory,
            native_factory: native_factory,
            depth: 0,
            static_flag: false,
        }
    }

    /// Populates executive from parent properties. Increments executive depth.
    pub fn from_parent(
        state: &'a mut State<B>,
        info: &'a EnvInfo,
        engine: &'a Engine,
        vm_factory: &'a Factory,
        native_factory: &'a NativeFactory,
        parent_depth: usize,
        static_flag: bool,
    ) -> Self {
        Executive {
            state: state,
            info: info,
            engine: engine,
            vm_factory: vm_factory,
            native_factory: native_factory,
            depth: parent_depth + 1,
            static_flag: static_flag,
        }
    }

    /// Creates `Externalities` from `Executive`.
    pub fn as_externalities<'any, T, V>(
        &'any mut self,
        origin_info: OriginInfo,
        substate: &'any mut Substate,
        output: OutputPolicy<'any, 'any>,
        tracer: &'any mut T,
        vm_tracer: &'any mut V,
        static_call: bool,
    ) -> Externalities<'any, T, V, B>
    where
        T: Tracer,
        V: VMTracer,
    {
        let is_static = self.static_flag || static_call;
        Externalities::new(
            self.state,
            self.info,
            self.engine,
            self.vm_factory,
            self.native_factory,
            self.depth,
            origin_info,
            substate,
            output,
            tracer,
            vm_tracer,
            is_static,
        )
    }

    /// This function should be used to execute transaction.
    pub fn transact(&'a mut self, t: &SignedTransaction, options: TransactOptions) -> Result<Executed, ExecutionError> {
        match (options.tracing, options.vm_tracing) {
            (true, true) => self.transact_with_tracer(
                t,
                options,
                ExecutiveTracer::default(),
                ExecutiveVMTracer::toplevel(),
            ),
            (true, false) => self.transact_with_tracer(t, options, ExecutiveTracer::default(), NoopVMTracer),
            (false, true) => self.transact_with_tracer(t, options, NoopTracer, ExecutiveVMTracer::toplevel()),
            (false, false) => self.transact_with_tracer(t, options, NoopTracer, NoopVMTracer),
        }
    }

    /// Execute transaction/call with tracing enabled
    pub fn transact_with_tracer<T, V>(
        &'a mut self,
        t: &SignedTransaction,
        options: TransactOptions,
        mut tracer: T,
        mut vm_tracer: V,
    ) -> Result<Executed, ExecutionError>
    where
        T: Tracer,
        V: VMTracer,
    {
        let sender = *t.sender();
        let nonce = self.state.nonce(&sender)?;

        // NOTE: there can be no invalid transactions from this point
        self.state.inc_nonce(&sender)?;

        trace!("permission should be check: {}", options.check_permission);
        if options.check_permission {
            self.check_permission(t)?;
        }

        let base_gas_required = U256::from(100); // `CREATE` transaction cost

        if sender != Address::zero() && t.action != Action::Store && t.gas < base_gas_required {
            return Err(From::from(ExecutionError::NotEnoughBaseGas {
                required: base_gas_required,
                got: t.gas,
            }));
        }

        trace!("quota should be checked: {}", options.check_quota);
        if options.check_quota {
            self.check_quota(t)?;
        }

        if t.action == Action::AbiStore {
            let account = H160::from(&t.data[0..20]);
            let abi = &t.data[20..];
            info!("contract address: {:?}, abi: {:?}", account, abi);
            match self.state.exists(&account) {
                Ok(true) => {
                    self.state.init_abi(&account, abi.to_vec())?;
                }
                _ => {
                    return Err(From::from(ExecutionError::TransactionMalformed(
                        "Account doesn't exist".to_string(),
                    )));
                }
            }
        }

        // TODO: we might need checking balance here in future. The relationship between BALANCE & GAS needs discussing.

        // NOTE: there can be no invalid transactions from this point

        let mut substate = Substate::new();

        // TODO: we might need sub_balance in future. The relationship between BALANCE & GAS needs discussing.

        let (result, output) = match t.action {
            Action::Store | Action::AbiStore | Action::GoCreate => (
                Ok(FinalizationResult {
                    gas_left: t.gas,
                    return_data: ReturnData::empty(),
                    apply_state: true,
                }),
                vec![],
            ),
            Action::Create => {
                let new_address = contract_address(&sender, &nonce);
                let params = ActionParams {
                    code_address: new_address,
                    code_hash: t.data.crypt_hash(),
                    address: new_address,
                    sender: sender,
                    origin: sender,
                    gas: t.gas - base_gas_required,
                    gas_price: t.gas_price,
                    value: ActionValue::Transfer(t.value),
                    code: Some(Arc::new(t.data.clone())),
                    data: None,
                    call_type: CallType::None,
                };
                (
                    self.create(params, &mut substate, &mut tracer, &mut vm_tracer),
                    vec![],
                )
            }
            Action::Call(ref address) => {
                let params = ActionParams {
                    code_address: *address,
                    address: *address,
                    sender: sender,
                    origin: sender,
                    gas: t.gas - base_gas_required,
                    gas_price: t.gas_price,
                    value: ActionValue::Transfer(t.value),
                    code: self.state.code(address)?,
                    code_hash: self.state.code_hash(address)?,
                    data: Some(t.data.clone()),
                    call_type: CallType::Call,
                };
                trace!(target: "executive", "call: {:?}", params);
                let mut out = vec![];
                (
                    self.call(
                        params,
                        &mut substate,
                        BytesRef::Flexible(&mut out),
                        &mut tracer,
                        &mut vm_tracer,
                    ),
                    out,
                )
            }
        };

        // finalize here!
        Ok(self.finalize(
            t,
            nonce,
            substate,
            result,
            output,
            tracer.traces(),
            vm_tracer.drain(),
        )?)
    }

    /// Check the sender's permission
    fn check_permission(&self, t: &SignedTransaction) -> Result<(), ExecutionError> {
        let sender = *t.sender();
        self.check_send_tx(&sender)?;

        match t.action {
            Action::Create => {
                self.check_create_contract(&sender)?;
            }
            Action::Call(address) => {
                let group_management_addr = Address::from(0x13241c2);
                trace!("t.data {:?}", t.data);

                if t.data.len() < 4 {
                    return Err(From::from(ExecutionError::TransactionMalformed(
                        "The length of transation data is less than four bytes".to_string(),
                    )));
                }

                if address == group_management_addr {
                    if t.data.len() < 36 {
                        return Err(From::from(ExecutionError::TransactionMalformed(
                            "Data should have at least one parameter".to_string(),
                        )));
                    }
                    self.check_origin_group(
                        &sender,
                        &address,
                        t.data[0..4].to_vec(),
                        &H160::from(&t.data[16..36]),
                    )?;
                }

                self.check_call_contract(&sender, &address, t.data[0..4].to_vec())?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Check permission: send transaction
    fn check_send_tx(&self, account: &Address) -> Result<(), ExecutionError> {
        let cont = Address::from(0x1);
        let func = vec![0; 4];
        let has_permission = self.has_resource(account, &cont, func);

        trace!("has send tx permission: {:?}", has_permission);

        if *account != Address::zero() && !has_permission {
            return Err(From::from(ExecutionError::NoTransactionPermission));
        }

        Ok(())
    }

    /// Check permission: create contract
    fn check_create_contract(&self, account: &Address) -> Result<(), ExecutionError> {
        let cont = Address::from(0x2);
        let func = vec![0; 4];
        let has_permission = self.has_resource(account, &cont, func);

        trace!("has create contract permission: {:?}", has_permission);

        if *account != Address::zero() && !has_permission {
            return Err(From::from(ExecutionError::NoContractPermission));
        }

        Ok(())
    }

    /// Check permission: call contract
    fn check_call_contract(&self, account: &Address, cont: &Address, func: Vec<u8>) -> Result<(), ExecutionError> {
        let has_permission = self.has_resource(account, cont, func);

        trace!("has call contract permission: {:?}", has_permission);

        if !has_permission {
            return Err(From::from(ExecutionError::NoCallPermission));
        }

        Ok(())
    }

    /// Check permission with parameter: origin group
    fn check_origin_group(
        &self,
        account: &Address,
        cont: &Address,
        func: Vec<u8>,
        param: &Address,
    ) -> Result<(), ExecutionError> {
        let has_permission = !contains_resource(
            &self.state.account_permissions,
            account,
            *cont,
            func.clone(),
        );

        trace!("Sender has call contract permission: {:?}", has_permission);

        if !has_permission && !contains_resource(&self.state.account_permissions, param, *cont, func.clone()) {
            return Err(From::from(ExecutionError::NoCallPermission));
        }

        Ok(())
    }

    /// Check the account has resource
    /// 1. Check the account has resource
    /// 2. Check all account's groups has resource
    fn has_resource(&self, account: &Address, cont: &Address, func: Vec<u8>) -> bool {
        let groups = self.get_groups(account);

        if !contains_resource(
            &self.state.account_permissions,
            account,
            *cont,
            func.clone(),
        ) {
            for group in groups {
                if contains_resource(&self.state.account_permissions, &group, *cont, func.clone()) {
                    return true;
                }
            }

            return false;
        }

        true
    }

    /// Get all sender's groups
    fn get_groups(&self, account: &Address) -> Vec<Address> {
        let mut groups: Vec<Address> = vec![];

        for (group, accounts) in &self.state.group_accounts {
            if accounts.contains(account) {
                groups.push(*group);
            }
        }

        groups
    }

    /// Check the quota while processing the transaction
    fn check_quota(&self, t: &SignedTransaction) -> Result<(), ExecutionError> {
        let sender = *t.sender();

        // validate if transaction fits into given block
        if sender != Address::zero() && self.info.gas_used + t.gas > self.info.gas_limit {
            return Err(From::from(ExecutionError::BlockGasLimitReached {
                gas_limit: self.info.gas_limit,
                gas_used: self.info.gas_used,
                gas: t.gas,
            }));
        }
        if sender != Address::zero() && t.gas > self.info.account_gas_limit {
            return Err(From::from(ExecutionError::AccountGasLimitReached {
                gas_limit: self.info.account_gas_limit,
                gas: t.gas,
            }));
        }

        Ok(())
    }

    fn exec_vm<T, V>(
        &mut self,
        params: ActionParams,
        unconfirmed_substate: &mut Substate,
        output_policy: OutputPolicy,
        tracer: &mut T,
        vm_tracer: &mut V,
    ) -> evm::Result<FinalizationResult>
    where
        T: Tracer,
        V: VMTracer,
    {
        let depth_threshold = io::LOCAL_STACK_SIZE.with(|sz| sz.get() / STACK_SIZE_PER_DEPTH);
        let static_call = params.call_type == CallType::StaticCall;

        // Ordinary execution - keep VM in same thread
        if (self.depth + 1) % depth_threshold != 0 {
            let vm_factory = self.vm_factory;
            let mut ext = self.as_externalities(
                OriginInfo::from(&params),
                unconfirmed_substate,
                output_policy,
                tracer,
                vm_tracer,
                static_call,
            );
            return vm_factory
                .create(params.gas)
                .exec(params, &mut ext)
                .finalize(ext);
        }

        // Start in new thread to reset stack
        // TODO [todr] No thread builder yet, so we need to reset once for a while
        // https://github.com/aturon/crossbeam/issues/16
        crossbeam::scope(|scope| {
            let vm_factory = self.vm_factory;
            let mut ext = self.as_externalities(
                OriginInfo::from(&params),
                unconfirmed_substate,
                output_policy,
                tracer,
                vm_tracer,
                static_call,
            );

            scope.spawn(move || {
                vm_factory
                    .create(params.gas)
                    .exec(params, &mut ext)
                    .finalize(ext)
            })
        }).join()
    }

    /// Calls contract function with given contract params.
    /// NOTE. It does not finalize the transaction (doesn't do refunds, nor suicides).
    /// Modifies the substate and the output.
    /// Returns either gas_left or `evm::Error`.
    pub fn call<T, V>(
        &mut self,
        params: ActionParams,
        substate: &mut Substate,
        mut output: BytesRef,
        tracer: &mut T,
        vm_tracer: &mut V,
    ) -> evm::Result<FinalizationResult>
    where
        T: Tracer,
        V: VMTracer,
    {
        trace!(
            "Executive::call(params={:?}) self.env_info={:?}, static={}",
            params,
            self.info,
            self.static_flag
        );
        if (params.call_type == CallType::StaticCall || (params.call_type == CallType::Call && self.static_flag))
            && params.value.value() > 0.into()
        {
            return Err(evm::Error::MutableCallInStaticContext);
        }

        // backup used in case of running out of gas
        self.state.checkpoint();

        let static_call = params.call_type == CallType::StaticCall;

        // at first, transfer value to destination
        if let ActionValue::Transfer(val) = params.value {
            self.state
                .transfer_balance(&params.sender, &params.address, &val)?;
        }

        if let Some(mut contract) = self.native_factory.new_contract(params.code_address) {
            let cost = U256::from(100);
            if cost <= params.gas {
                let mut unconfirmed_substate = Substate::new();
                let mut trace_output = tracer.prepare_trace_output();
                let output_policy = OutputPolicy::Return(output, trace_output.as_mut());
                let res = {
                    let mut tracer = NoopTracer;
                    let mut vmtracer = NoopVMTracer;
                    let mut ext = self.as_externalities(
                        OriginInfo::from(&params),
                        &mut unconfirmed_substate,
                        output_policy,
                        &mut tracer,
                        &mut vmtracer,
                        static_call,
                    );
                    contract.exec(params, &mut ext).finalize(ext)
                };
                self.enact_result(&res, substate, unconfirmed_substate);
                trace!(target: "executive", "enacted: substate={:?}\n", substate);
                return res;
            }
        }
        if self.engine.is_builtin(&params.code_address) {
            // if destination is builtin, try to execute it

            let default = [];
            let data = if let Some(ref d) = params.data {
                d as &[u8]
            } else {
                &default as &[u8]
            };

            let trace_info = tracer.prepare_trace_call(&params);

            let cost = self.engine.cost_of_builtin(&params.code_address, data);
            if cost <= params.gas {
                self.engine
                    .execute_builtin(&params.code_address, data, &mut output);
                self.state.discard_checkpoint();

                // trace only top level calls to builtins to avoid DDoS attacks
                if self.depth == 0 {
                    let mut trace_output = tracer.prepare_trace_output();
                    if let Some(out) = trace_output.as_mut() {
                        *out = output.to_owned();
                    }

                    tracer.trace_call(trace_info, cost, trace_output, vec![]);
                }
                Ok(FinalizationResult {
                    gas_left: params.gas - cost,
                    return_data: ReturnData::new(output.to_owned(), 0, output.len()),
                    apply_state: true,
                })
            } else {
                // just drain the whole gas
                self.state.revert_to_checkpoint();

                tracer.trace_failed_call(trace_info, vec![], evm::Error::OutOfGas.into());

                Err(evm::Error::OutOfGas)
            }
        } else {
            let trace_info = tracer.prepare_trace_call(&params);
            let mut trace_output = tracer.prepare_trace_output();
            let mut subtracer = tracer.subtracer();

            let gas = params.gas;

            if params.code.is_some() {
                // part of substate that may be reverted
                let mut unconfirmed_substate = Substate::new();

                // TODO: make ActionParams pass by ref then avoid copy altogether.
                let mut subvmtracer = vm_tracer.prepare_subtrace(
                    params
                        .code
                        .as_ref()
                        .expect("scope is conditional on params.code.is_some(); qed"),
                );

                let res = {
                    self.exec_vm(
                        params,
                        &mut unconfirmed_substate,
                        OutputPolicy::Return(output, trace_output.as_mut()),
                        &mut subtracer,
                        &mut subvmtracer,
                    )
                };

                vm_tracer.done_subtrace(subvmtracer);

                trace!(target: "executive", "res={:?}", res);

                let traces = subtracer.traces();
                match res {
                    Ok(ref res) => tracer.trace_call(trace_info, gas - res.gas_left, trace_output, traces),
                    Err(ref e) => tracer.trace_failed_call(trace_info, traces, e.into()),
                };

                trace!(target: "executive", "substate={:?}; unconfirmed_substate={:?}\n",
                       substate, unconfirmed_substate);

                self.enact_result(&res, substate, unconfirmed_substate);
                trace!(target: "executive", "enacted: substate={:?}\n", substate);
                res
            } else {
                // otherwise it's just a basic transaction, only do tracing, if necessary.
                self.state.discard_checkpoint();

                tracer.trace_call(trace_info, U256::zero(), trace_output, vec![]);
                Ok(FinalizationResult {
                    gas_left: params.gas,
                    return_data: ReturnData::empty(),
                    apply_state: true,
                })
            }
        }
    }

    /// Creates contract with given contract params.
    /// NOTE. It does not finalize the transaction (doesn't do refunds, nor suicides).
    /// Modifies the substate.
    pub fn create<T, V>(
        &mut self,
        params: ActionParams,
        substate: &mut Substate,
        tracer: &mut T,
        vm_tracer: &mut V,
    ) -> evm::Result<FinalizationResult>
    where
        T: Tracer,
        V: VMTracer,
    {
        if self.state.exists_and_has_code_or_nonce(&params.address)? {
            return Err(evm::Error::OutOfGas);
        }
        trace!(
            "Executive::create(params={:?}) self.env_info={:?}, static={}",
            params,
            self.info,
            self.static_flag
        );
        if params.call_type == CallType::StaticCall || self.static_flag {
            let trace_info = tracer.prepare_trace_create(&params);
            tracer.trace_failed_create(
                trace_info,
                vec![],
                evm::Error::MutableCallInStaticContext.into(),
            );
            return Err(evm::Error::MutableCallInStaticContext);
        }

        // backup used in case of running out of gas
        self.state.checkpoint();

        // part of substate that may be reverted
        let mut unconfirmed_substate = Substate::new();

        // create contract and transfer value to it if necessary
        /*let schedule = self.engine.schedule(self.info);
        let nonce_offset = if schedule.no_empty {1} else {0}.into();*/
        let nonce_offset = U256::from(0);
        let prev_bal = self.state.balance(&params.address)?;
        if let ActionValue::Transfer(val) = params.value {
            self.state.sub_balance(&params.sender, &val)?;
            self.state
                .new_contract(&params.address, val + prev_bal, nonce_offset);
        } else {
            self.state
                .new_contract(&params.address, prev_bal, nonce_offset);
        }
        //self.state.new_contract(&params.address, 0.into(), nonce_offset);

        let trace_info = tracer.prepare_trace_create(&params);
        let mut trace_output = tracer.prepare_trace_output();
        let mut subtracer = tracer.subtracer();
        let gas = params.gas;
        let created = params.address;

        let mut subvmtracer = vm_tracer.prepare_subtrace(params.code.as_ref().expect(
            "two ways into create (Externalities::create and Executive::transact_with_tracer)
                                       ; both place `Some(...)` `code` in `params`; qed",
        ));

        let res = {
            self.exec_vm(
                params,
                &mut unconfirmed_substate,
                OutputPolicy::InitContract(trace_output.as_mut()),
                &mut subtracer,
                &mut subvmtracer,
            )
        };

        vm_tracer.done_subtrace(subvmtracer);

        match res {
            Ok(ref res) => tracer.trace_create(
                trace_info,
                gas - res.gas_left,
                trace_output,
                created,
                subtracer.traces(),
            ),
            Err(ref e) => tracer.trace_failed_create(trace_info, subtracer.traces(), e.into()),
        };

        self.enact_result(&res, substate, unconfirmed_substate);
        res
    }

    /// Finalizes the transaction (does refunds and suicides).
    fn finalize(
        &mut self,
        t: &SignedTransaction,
        account_nonce: U256,
        substate: Substate,
        result: evm::Result<FinalizationResult>,
        output: Bytes,
        trace: Vec<FlatTrace>,
        vm_trace: Option<VMTrace>,
    ) -> ExecutionResult {
        /*
        let schedule = self.engine.schedule(self.info);
         */
        let schedule = Schedule::new_v1();
        // refunds from SSTORE nonzero -> zero
        let sstore_refunds = U256::from(schedule.sstore_refund_gas) * substate.sstore_clears_count;
        // refunds from contract suicides
        let suicide_refunds = U256::from(schedule.suicide_refund_gas) * U256::from(substate.suicides.len());
        let refunds_bound = sstore_refunds + suicide_refunds;

        // real ammount to refund
        let gas_left_prerefund = match result {
            Ok(FinalizationResult { gas_left, .. }) => gas_left,
            _ => 0.into(),
        };
        let refunded = cmp::min(refunds_bound, (t.gas - gas_left_prerefund) >> 1);
        let gas_left = gas_left_prerefund + refunded;

        let gas_used = t.gas - gas_left;
        let refund_value = gas_left * t.gas_price;
        let fees_value = gas_used * t.gas_price;

        trace!(
            "exec::finalize: t.gas={}, sstore_refunds={}, suicide_refunds={}, refunds_bound={},
                gas_left_prerefund={}, refunded={}, gas_left={}, gas_used={}, refund_value={}, fees_value={}\n",
            t.gas,
            sstore_refunds,
            suicide_refunds,
            refunds_bound,
            gas_left_prerefund,
            refunded,
            gas_left,
            gas_used,
            refund_value,
            fees_value
        );

        let sender = t.sender();
        trace!(
            "exec::finalize: Refunding refund_value={}, sender={}\n",
            refund_value,
            sender
        );

        //TODO: we might need to add_balance here in future. The relationship between BALANCE & GAS needs discussing.

        trace!(
            "exec::finalize: Compensating author: fees_value={}, author={}\n",
            fees_value,
            &self.info.author
        );

        //TODO: we might need to add_balance here in future. The relationship between BALANCE & GAS needs discussing.

        // perform suicides
        for address in &substate.suicides {
            self.state.kill_account(address);
        }

        // TODO: kill_garbage might be used here in future.
        // perform garbage-collection
        for address in &substate.garbage {
            if self.state.exists(address)? && !self.state.exists_and_not_null(address)? {
                self.state.kill_account(address);
            }
        }

        match result {
            Err(evm::Error::Internal(msg)) => Err(ExecutionError::Internal(msg)),
            Err(exception) => Ok(Executed {
                exception: Some(exception),
                gas: t.gas,
                gas_used: t.gas,
                refunded: U256::zero(),
                cumulative_gas_used: self.info.gas_used + t.gas,
                logs: vec![],
                contracts_created: vec![],
                output: output,
                trace: trace,
                vm_trace: vm_trace,
                state_diff: None,
                account_nonce: account_nonce,
            }),
            Ok(r) => Ok(Executed {
                exception: if r.apply_state {
                    None
                } else {
                    Some(evm::Error::Reverted)
                },
                gas: t.gas,
                gas_used: gas_used,
                refunded: refunded,
                cumulative_gas_used: self.info.gas_used + gas_used,
                logs: substate.logs,
                contracts_created: substate.contracts_created,
                output: output,
                trace: trace,
                vm_trace: vm_trace,
                state_diff: None,
                account_nonce: account_nonce,
            }),
        }
    }

    fn enact_result(
        &mut self,
        result: &evm::Result<FinalizationResult>,
        substate: &mut Substate,
        un_substate: Substate,
    ) {
        match *result {
            Err(evm::Error::OutOfGas)
            | Err(evm::Error::BadJumpDestination { .. })
            | Err(evm::Error::BadInstruction { .. })
            | Err(evm::Error::StackUnderflow { .. })
            | Err(evm::Error::OutOfStack { .. })
            | Err(evm::Error::MutableCallInStaticContext)
            | Err(evm::Error::OutOfBounds)
            | Err(evm::Error::Reverted)
            | Ok(FinalizationResult {
                apply_state: false, ..
            }) => {
                self.state.revert_to_checkpoint();
            }
            Ok(_) | Err(evm::Error::Internal(_)) => {
                self.state.discard_checkpoint();
                substate.accrue(un_substate);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate rustc_hex;
    ////////////////////////////////////////////////////////////////////////////////

    use self::rustc_hex::FromHex;
    use super::*;
    use action_params::{ActionParams, ActionValue};
    use engines::NullEngine;
    use env_info::EnvInfo;
    use evm::{Factory, VMType};
    use state::Substate;
    use std::ops::Deref;
    use std::str::FromStr;
    use std::sync::Arc;
    use tests::helpers::*;
    use trace::{ExecutiveTracer, ExecutiveVMTracer};
    use util::{Address, H256, U256};
    #[test]
    fn test_create_contract() {
        logger::silent();
        let source = r#"
pragma solidity ^0.4.8;
contract AbiTest {
  uint balance;
  function AbiTest() {}
  function setValue(uint value) {
    balance = value;
  }
}
"#;
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let nonce = U256::zero();
        let gas_required = U256::from(100_000);

        let (deploy_code, runtime_code) = solc("AbiTest", source);
        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let native_factory = NativeFactory::default();
        let contract_address = contract_address(&sender, &nonce);
        let mut params = ActionParams::default();
        params.address = contract_address.clone();
        params.sender = sender.clone();
        params.origin = sender.clone();
        params.gas = gas_required;
        params.code = Some(Arc::new(deploy_code));
        params.value = ActionValue::Apparent(0.into());
        let mut state = get_temp_state();

        let info = EnvInfo::default();
        let engine = NullEngine::default();
        let mut substate = Substate::new();
        let mut tracer = ExecutiveTracer::default();
        let mut vm_tracer = ExecutiveVMTracer::toplevel();

        {
            let mut ex = Executive::new(&mut state, &info, &engine, &factory, &native_factory);
            let _ = ex.create(params.clone(), &mut substate, &mut tracer, &mut vm_tracer);
        }

        assert_eq!(
            state.code(&contract_address).unwrap().unwrap().deref(),
            &runtime_code
        );
    }

    #[test]
    fn test_call_contract() {
        logger::silent();
        let source = r#"
pragma solidity ^0.4.8;
contract AbiTest {
  uint balance;
  function AbiTest() {}
  function setValue(uint value) {
    balance = value;
  }
}
"#;
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let gas_required = U256::from(100_000);
        let contract_addr = Address::from_str("62f4b16d67b112409ab4ac87274926382daacfac").unwrap();
        let (_, runtime_code) = solc("AbiTest", source);
        // big endian: value=0x12345678
        let data = "552410770000000000000000000000000000000000000000000000000000000012345678"
            .from_hex()
            .unwrap();
        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let native_factory = NativeFactory::default();
        let mut tracer = ExecutiveTracer::default();
        let mut vm_tracer = ExecutiveVMTracer::toplevel();

        let mut state = get_temp_state();
        state
            .init_code(&contract_addr, runtime_code.clone())
            .unwrap();
        let mut params = ActionParams::default();
        params.address = contract_addr.clone();
        params.sender = sender.clone();
        params.gas = gas_required;
        params.code = state.code(&contract_addr).unwrap();
        params.code_hash = state.code_hash(&contract_addr).unwrap();
        params.value = ActionValue::Transfer(U256::from(0));
        params.data = Some(data);

        let info = EnvInfo::default();
        let engine = NullEngine::default();
        let mut substate = Substate::new();
        {
            let mut ex = Executive::new(&mut state, &info, &engine, &factory, &native_factory);
            let mut out = vec![];
            let _ = ex.call(
                params,
                &mut substate,
                BytesRef::Fixed(&mut out),
                &mut tracer,
                &mut vm_tracer,
            );
        };

        assert_eq!(
            state
                // it was supposed that value's address is balance.
                .storage_at(&contract_addr, &H256::from(&U256::from(0)))
                .unwrap(),
            H256::from(&U256::from(0x12345678))
        );
    }

    #[test]
    fn test_revert_instruction() {
        logger::silent();
        let source = r#"
pragma solidity ^0.4.8;
contract AbiTest {
  uint balance;

  modifier Never {
    require(false);
      _;
  }

  function AbiTest() {}
  function setValue(uint value) Never {
    balance = value;
  }
}
"#;
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let gas_required = U256::from(100_000);
        let contract_addr = Address::from_str("62f4b16d67b112409ab4ac87274926382daacfac").unwrap();
        let (_, runtime_code) = solc("AbiTest", source);
        // big endian: value=0x12345678
        let data = "552410770000000000000000000000000000000000000000000000000000000012345678"
            .from_hex()
            .unwrap();
        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let native_factory = NativeFactory::default();
        let mut tracer = ExecutiveTracer::default();
        let mut vm_tracer = ExecutiveVMTracer::toplevel();

        let mut state = get_temp_state();
        state
            .init_code(&contract_addr, runtime_code.clone())
            .unwrap();
        let mut params = ActionParams::default();
        params.address = contract_addr.clone();
        params.sender = sender.clone();
        params.gas = gas_required;
        params.code = state.code(&contract_addr).unwrap();
        params.code_hash = state.code_hash(&contract_addr).unwrap();
        params.value = ActionValue::Transfer(U256::from(0));
        params.data = Some(data);

        let info = EnvInfo::default();
        let engine = NullEngine::default();
        let mut substate = Substate::new();
        {
            let mut ex = Executive::new(&mut state, &info, &engine, &factory, &native_factory);
            let mut out = vec![];
            let res = ex.call(
                params,
                &mut substate,
                BytesRef::Fixed(&mut out),
                &mut tracer,
                &mut vm_tracer,
            );
            assert!(res.is_ok());
            match res {
                Ok(gas_used) => println!("gas used: {:?}", gas_used),
                Err(e) => println!("e: {:?}", e),
            }
        };

        assert_eq!(
            state
                // it was supposed that value's address is balance.
                .storage_at(&contract_addr, &H256::from(&U256::from(0)))
                .unwrap(),
            H256::from(&U256::from(0x0))
        );
    }

    #[test]
    fn test_require_instruction() {
        logger::silent();
        let source = r#"
pragma solidity ^0.4.8;
contract AbiTest {
  uint balance;

  modifier Never {
    require(true);
      _;
  }

  function AbiTest() {}
  function setValue(uint value) Never {
    balance = value;
  }
}
"#;
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let gas_required = U256::from(100_000);
        let contract_addr = Address::from_str("62f4b16d67b112409ab4ac87274926382daacfac").unwrap();
        let (_, runtime_code) = solc("AbiTest", source);
        // big endian: value=0x12345678
        let data = "552410770000000000000000000000000000000000000000000000000000000012345678"
            .from_hex()
            .unwrap();
        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let native_factory = NativeFactory::default();
        let mut tracer = ExecutiveTracer::default();
        let mut vm_tracer = ExecutiveVMTracer::toplevel();

        let mut state = get_temp_state();
        state
            .init_code(&contract_addr, runtime_code.clone())
            .unwrap();
        let mut params = ActionParams::default();
        params.address = contract_addr.clone();
        params.sender = sender.clone();
        params.gas = gas_required;
        params.code = state.code(&contract_addr).unwrap();
        params.code_hash = state.code_hash(&contract_addr).unwrap();
        params.value = ActionValue::Transfer(U256::from(0));
        params.data = Some(data);

        let info = EnvInfo::default();
        let engine = NullEngine::default();
        let mut substate = Substate::new();
        {
            let mut ex = Executive::new(&mut state, &info, &engine, &factory, &native_factory);
            let mut out = vec![];
            let res = ex.call(
                params,
                &mut substate,
                BytesRef::Fixed(&mut out),
                &mut tracer,
                &mut vm_tracer,
            );
            assert!(res.is_ok());
            match res {
                Ok(gas_used) => println!("gas used: {:?}", gas_used),
                Err(e) => println!("e: {:?}", e),
            }
        };

        assert_eq!(
            state
                // it was supposed that value's address is balance.
                .storage_at(&contract_addr, &H256::from(&U256::from(0)))
                .unwrap(),
            H256::from(&U256::from(0x12345678))
        );
    }
}
