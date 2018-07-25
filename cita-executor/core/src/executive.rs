// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Transaction Execution environment.

use builtin::Builtin;
use cita_types::{Address, H160, H256, U256, U512};
use contracts::permission_management::contains_resource;
use contracts::Resource;
use crossbeam;
use engines::Engine;
use error::ExecutionError;
use evm::action_params::{ActionParams, ActionValue};
use evm::call_type::CallType;
use evm::env_info::EnvInfo;
use evm::{self, Factory, FinalizationResult, Finalize, ReturnData, Schedule};
pub use executed::{Executed, ExecutionResult};
use externalities::*;
use grpc_contracts;
use grpc_contracts::contract::{invoke_grpc_contract, is_grpc_contract};
use grpc_contracts::service_registry;
use libexecutor::executor::EconomicalModel;
use libproto::citacode::InvokeResponse;
use log_entry::LogEntry;
use native::factory::Contract as NativeContract;
use native::factory::Factory as NativeFactory;
use state::backend::Backend as StateBackend;
use state::{State, Substate};
use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use trace::{
    ExecutiveTracer, ExecutiveVMTracer, FlatTrace, NoopTracer, NoopVMTracer, Tracer, VMTrace,
    VMTracer,
};
use types::reserved_addresses;
use types::transaction::{Action, SignedTransaction};
use util::*;

/// Roughly estimate what stack size each level of evm depth will use
/// TODO [todr] We probably need some more sophisticated calculations here
///      (limit on my machine 132)
/// Maybe something like here:
/// `https://github.com/ethereum/libethereum/blob/4db169b8504f2b87f7d5a481819cfb959fc65f6c/libethereum/ExtVM.cpp`
const STACK_SIZE_PER_DEPTH: usize = 24 * 1024;

thread_local! {
    /// Stack size
    /// Should be modified if it is changed in Rust since it is no way
    /// to know or get it
    pub static LOCAL_STACK_SIZE: ::std::cell::Cell<usize> = ::std::cell::Cell::new(
        ::std::env::var("RUST_MIN_STACK").ok().and_then(
            |s| s.parse().ok()).unwrap_or(2 * 1024 * 1024));
}

///amend the abi data
const AMEND_ABI: u32 = 1;
///amend the account code
const AMEND_CODE: u32 = 2;
///amend the kv of db
const AMEND_KV_H256: u32 = 3;
///amend get the value of db
const AMEND_GET_KV_H256: u32 = 4;

// minimum required gas, just for check
const MIN_GAS_REQUIRED: u32 = 100;

/// Returns new address created from address and given nonce.
pub fn contract_address(address: &Address, nonce: &U256) -> Address {
    use rlp::RlpStream;

    let mut stream = RlpStream::new_list(2);
    stream.append(address);
    stream.append(nonce);
    From::from(stream.out().crypt_hash())
}

/// Check the sender's permission
pub fn check_permission(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    t: &SignedTransaction,
) -> Result<(), ExecutionError> {
    let sender = *t.sender();
    check_send_tx(group_accounts, account_permissions, &sender)?;

    match t.action {
        Action::Create => {
            check_create_contract(group_accounts, account_permissions, &sender)?;
        }
        Action::Call(address) => {
            let group_management_addr =
                Address::from_str(reserved_addresses::GROUP_MANAGEMENT).unwrap();
            trace!("t.data {:?}", t.data);

            if t.data.len() < 4 {
                return Err(ExecutionError::TransactionMalformed(
                    "The length of transation data is less than four bytes".to_string(),
                ));
            }

            if address == group_management_addr {
                if t.data.len() < 36 {
                    return Err(ExecutionError::TransactionMalformed(
                        "Data should have at least one parameter".to_string(),
                    ));
                }
                check_origin_group(
                    account_permissions,
                    &sender,
                    &address,
                    t.data[0..4].to_vec(),
                    &H160::from(&t.data[16..36]),
                )?;
            }

            check_call_contract(
                group_accounts,
                account_permissions,
                &sender,
                &address,
                t.data[0..4].to_vec(),
            )?;
        }
        _ => {}
    }

    Ok(())
}

/// Check permission: send transaction
fn check_send_tx(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
) -> Result<(), ExecutionError> {
    let cont = Address::from_str(reserved_addresses::PERMISSION_SEND_TX).unwrap();
    let func = vec![0; 4];
    let has_permission = has_resource(group_accounts, account_permissions, account, &cont, func);

    trace!("has send tx permission: {:?}", has_permission);

    if *account != Address::zero() && !has_permission {
        return Err(ExecutionError::NoTransactionPermission);
    }

    Ok(())
}

/// Check permission: create contract
fn check_create_contract(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
) -> Result<(), ExecutionError> {
    let cont = Address::from_str(reserved_addresses::PERMISSION_CREATE_CONTRACT).unwrap();
    let func = vec![0; 4];
    let has_permission = has_resource(group_accounts, account_permissions, account, &cont, func);

    trace!("has create contract permission: {:?}", has_permission);

    if *account != Address::zero() && !has_permission {
        return Err(ExecutionError::NoContractPermission);
    }

    Ok(())
}

/// Check permission: call contract
fn check_call_contract(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
    cont: &Address,
    func: Vec<u8>,
) -> Result<(), ExecutionError> {
    let has_permission = has_resource(group_accounts, account_permissions, account, cont, func);

    trace!("has call contract permission: {:?}", has_permission);

    if !has_permission {
        return Err(ExecutionError::NoCallPermission);
    }

    Ok(())
}

/// Check permission with parameter: origin group
fn check_origin_group(
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
    cont: &Address,
    func: Vec<u8>,
    param: &Address,
) -> Result<(), ExecutionError> {
    let has_permission = contains_resource(account_permissions, account, *cont, func.clone());

    trace!("Sender has call contract permission: {:?}", has_permission);

    if !has_permission && !contains_resource(account_permissions, param, *cont, func.clone()) {
        return Err(ExecutionError::NoCallPermission);
    }

    Ok(())
}

/// Check the account has resource
/// 1. Check the account has resource
/// 2. Check all account's groups has resource
fn has_resource(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
    cont: &Address,
    func: Vec<u8>,
) -> bool {
    let groups = get_groups(group_accounts, account);

    if !contains_resource(account_permissions, account, *cont, func.clone()) {
        for group in groups {
            if contains_resource(account_permissions, &group, *cont, func.clone()) {
                return true;
            }
        }

        return false;
    }

    true
}

/// Get all sender's groups
fn get_groups(group_accounts: &HashMap<Address, Vec<Address>>, account: &Address) -> Vec<Address> {
    let mut groups: Vec<Address> = vec![];

    for (group, accounts) in group_accounts {
        if accounts.contains(account) {
            groups.push(*group);
        }
    }

    groups
}

/// Check the quota while processing the transaction
/*pub fn check_quota(
    gas_used: U256,
    gas_limit: U256,
    account_gas_limit: U256,
    t: &SignedTransaction,
) -> Result<(), ExecutionError> {
    let sender = *t.sender();

    // validate if transaction fits into given block
    if sender != Address::zero() && gas_used + t.gas > gas_limit {
        return Err(ExecutionError::BlockGasLimitReached {
            gas_limit: gas_limit,
            gas_used: gas_used,
            gas: t.gas,
        });
    }
    if sender != Address::zero() && t.gas > account_gas_limit {
        return Err(ExecutionError::AccountGasLimitReached {
            gas_limit: account_gas_limit,
            gas: t.gas,
        });
    }

    Ok(())
}*/

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
    /// Check EconomicalModel
    economical_model: EconomicalModel,
}

impl<'a, B: 'a + StateBackend> Executive<'a, B> {
    /// Basic constructor.
    pub fn new(
        state: &'a mut State<B>,
        info: &'a EnvInfo,
        engine: &'a Engine,
        vm_factory: &'a Factory,
        native_factory: &'a NativeFactory,
        static_flag: bool,
        economical_model: EconomicalModel,
    ) -> Self {
        Executive {
            state: state,
            info: info,
            engine: engine,
            vm_factory: vm_factory,
            native_factory: native_factory,
            depth: 0,
            static_flag: static_flag,
            economical_model: economical_model,
        }
    }

    pub fn payment_required(&self) -> bool {
        self.economical_model == EconomicalModel::Charge
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
        economical_model: EconomicalModel,
    ) -> Self {
        Executive {
            state: state,
            info: info,
            engine: engine,
            vm_factory: vm_factory,
            native_factory: native_factory,
            depth: parent_depth + 1,
            static_flag: static_flag,
            economical_model: economical_model,
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
        economical_model: EconomicalModel,
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
            economical_model,
        )
    }

    /// This function should be used to execute transaction.
    pub fn transact(
        &'a mut self,
        t: &SignedTransaction,
        options: TransactOptions,
    ) -> Result<Executed, ExecutionError> {
        match (options.tracing, options.vm_tracing) {
            (true, true) => self.transact_with_tracer(
                t,
                options,
                ExecutiveTracer::default(),
                ExecutiveVMTracer::toplevel(),
            ),
            (true, false) => {
                self.transact_with_tracer(t, options, ExecutiveTracer::default(), NoopVMTracer)
            }
            (false, true) => {
                self.transact_with_tracer(t, options, NoopTracer, ExecutiveVMTracer::toplevel())
            }
            (false, false) => self.transact_with_tracer(t, options, NoopTracer, NoopVMTracer),
        }
    }

    fn transact_set_abi(&mut self, data: &[u8]) -> bool {
        let account = H160::from(&data[0..20]);
        let abi = &data[20..];
        info!("set abi of contract address: {:?}", account);
        self.state
            .exists(&account)
            .map(|exists| exists && self.state.init_abi(&account, abi.to_vec()).is_ok())
            .unwrap_or(false)
    }

    fn transact_set_code(&mut self, data: &[u8]) -> bool {
        let account = H160::from(&data[0..20]);
        let code = &data[20..];
        self.state.reset_code(&account, code.to_vec()).is_ok()
    }

    fn transact_set_kv_h256(&mut self, data: &[u8]) -> bool {
        let account = H160::from(&data[0..20]);
        let key = H256::from_slice(&data[20..52]);
        let val = H256::from_slice(&data[52..84]);
        self.state.set_storage(&account, key, val).is_ok()
    }

    fn transact_get_kv_h256(&mut self, data: &[u8]) -> Option<H256> {
        let account = H160::from(&data[0..20]);
        let key = H256::from_slice(&data[20..52]);
        self.state.storage_at(&account, &key).ok()
    }

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

        self.state.inc_nonce(&sender)?;

        trace!("permission should be check: {}", options.check_permission);
        if options.check_permission {
            check_permission(
                &self.state.group_accounts,
                &self.state.account_permissions,
                t,
            )?;
        }

        if sender != Address::zero() && t.gas < U256::from(MIN_GAS_REQUIRED) {
            return Err(ExecutionError::NotEnoughBaseGas {
                required: U256::from(MIN_GAS_REQUIRED),
                got: t.gas,
            });
        }

        if t.action == Action::AmendData {
            if let Some(admin) = self.state.super_admin_account {
                if *t.sender() != admin {
                    return Err(ExecutionError::NoTransactionPermission);
                }
            } else {
                return Err(ExecutionError::NoTransactionPermission);
            }
        }

        /*trace!("quota should be checked: {}", options.check_quota);
        if options.check_quota {
            check_quota(
                self.info.gas_used,
                self.info.gas_limit,
                self.info.account_gas_limit,
                t,
            )?;
        }*/

        let mut need_output: Vec<u8> = vec![];
        if t.action == Action::AbiStore {
            if !self.transact_set_abi(&t.data) {
                return Err(ExecutionError::TransactionMalformed(
                    "Account doesn't exist".to_string(),
                ));
            }
        } else if t.action == Action::AmendData {
            let atype = t.value.low_u32();
            match atype {
                AMEND_ABI => {
                    if !self.transact_set_abi(&t.data) {
                        return Err(ExecutionError::TransactionMalformed(
                            "Account doesn't exist".to_string(),
                        ));
                    }
                }
                AMEND_CODE => {
                    if !self.transact_set_code(&t.data) {
                        return Err(ExecutionError::TransactionMalformed(
                            "Account doesn't exist".to_string(),
                        ));
                    }
                }
                AMEND_KV_H256 => {
                    if !self.transact_set_kv_h256(&t.data) {
                        return Err(ExecutionError::TransactionMalformed(
                            "Account doesn't exist".to_string(),
                        ));
                    }
                }
                AMEND_GET_KV_H256 => {
                    if let Some(v) = self.transact_get_kv_h256(&t.data) {
                        need_output = v.to_vec();
                    } else {
                        return Err(ExecutionError::TransactionMalformed(
                            "May be incomplete trie error".to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(ExecutionError::TransactionMalformed(
                        "amend type if error".to_string(),
                    ));
                }
            }
        }

        // NOTE: there can be no invalid transactions from this point
        let balance = self.state.balance(&sender)?;
        let gas_cost = t.gas.full_mul(t.gas_price());
        let total_cost = U512::from(t.value) + gas_cost;

        // avoid unaffordable transactions
        if self.payment_required() {
            let balance512 = U512::from(balance);
            if balance512 < total_cost {
                return Err(ExecutionError::NotEnoughCash {
                    required: total_cost,
                    got: balance512,
                });
            }
            self.state.sub_balance(&sender, &U256::from(gas_cost))?;
        }

        let mut substate = Substate::new();

        let init_gas = t.gas - U256::from(MIN_GAS_REQUIRED);
        let (result, output) = match t.action {
            Action::Store | Action::AbiStore => {
                let schedule = Schedule::new_v1();
                let store_gas_used = U256::from(t.data.len() * schedule.create_data_gas);
                if let Some(gas_left) = init_gas.checked_sub(store_gas_used) {
                    (
                        Ok(FinalizationResult {
                            gas_left,
                            return_data: ReturnData::empty(),
                            apply_state: true,
                        }),
                        vec![],
                    )
                } else {
                    return Err(ExecutionError::NotEnoughBaseGas {
                        required: U256::from(MIN_GAS_REQUIRED).saturating_add(store_gas_used),
                        got: t.gas,
                    });
                }
            }
            Action::GoCreate => (
                Ok(FinalizationResult {
                    gas_left: init_gas,
                    return_data: ReturnData::empty(),
                    apply_state: true,
                }),
                vec![],
            ),
            Action::AmendData => (
                Ok(FinalizationResult {
                    // Super admin operations do not cost gas
                    gas_left: t.gas,
                    return_data: {
                        if need_output.is_empty() {
                            ReturnData::empty()
                        } else {
                            let len = need_output.len();
                            ReturnData::new(need_output, 0, len)
                        }
                    },
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
                    gas: init_gas,
                    gas_price: t.gas_price(),
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
                    gas: init_gas,
                    gas_price: t.gas_price(),
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
        let depth_threshold = LOCAL_STACK_SIZE.with(|sz| sz.get() / STACK_SIZE_PER_DEPTH);
        let static_call = params.call_type == CallType::StaticCall;

        // Ordinary execution - keep VM in same thread
        if (self.depth + 1) % depth_threshold != 0 {
            let vm_factory = self.vm_factory;
            let economical_model = self.economical_model;
            let mut ext = self.as_externalities(
                OriginInfo::from(&params),
                unconfirmed_substate,
                output_policy,
                tracer,
                vm_tracer,
                static_call,
                economical_model,
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
            let economical_model = self.economical_model;
            let mut ext = self.as_externalities(
                OriginInfo::from(&params),
                unconfirmed_substate,
                output_policy,
                tracer,
                vm_tracer,
                static_call,
                economical_model,
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
        if (params.call_type == CallType::StaticCall
            || (params.call_type == CallType::Call && self.static_flag))
            && params.value.value() > 0.into()
        {
            return Err(evm::Error::MutableCallInStaticContext);
        }

        // backup used in case of running out of gas
        self.state.checkpoint();

        let static_call = params.call_type == CallType::StaticCall;

        // at first, transfer value to destination
        if let (true, ActionValue::Transfer(val)) = (self.payment_required(), &params.value) {
            self.state
                .transfer_balance(&params.sender, &params.address, &val)?
        }

        if let Some(mut native_contract) = self.native_factory.new_contract(params.code_address) {
            // check and call Native Contract
            self.call_native_contract(
                params,
                substate,
                output,
                tracer,
                static_call,
                native_contract,
            )
        } else if is_grpc_contract(params.code_address) {
            self.call_grpc_contract(params, substate, output)
        } else if let Some(builtin) = self.engine.builtin(&params.code_address, self.info.number) {
            // check and call Builtin contract
            self.call_builtin_contract(params, output, tracer, builtin)
        } else {
            // call EVM contract
            self.call_evm_contract(params, substate, output, tracer, vm_tracer)
        }
    }

    fn call_grpc_contract(
        &mut self,
        params: ActionParams,
        substate: &mut Substate,
        output: BytesRef,
    ) -> evm::Result<FinalizationResult> {
        let connect_info = match service_registry::find_contract(params.code_address, true) {
            Some(contract_state) => contract_state.conn_info,
            None => {
                return Err(evm::error::Error::Internal(format!(
                    "can't find grpc contract from address: {:?}",
                    params.code_address
                )));
            }
        };
        let response = invoke_grpc_contract(
            self.info,
            params.clone(),
            self.state,
            true,
            true,
            connect_info,
        );
        match response {
            Ok(invoke_response) => {
                // store grpc return storages to stateDB
                for storage in invoke_response.get_storages().into_iter() {
                    let key = storage.get_key();
                    let value = storage.get_value();
                    trace!("recv resp: {:?}", storage);
                    trace!("key: {:?}, value: {:?}", key, value);
                    grpc_contracts::storage::set_storage(
                        self.state,
                        params.address,
                        key.to_vec(),
                        value.to_vec(),
                    );
                }

                substate.logs = invoke_response
                    .get_logs()
                    .into_iter()
                    .map(|log| {
                        let mut topics = Vec::new();
                        let tdata = log.get_topic();

                        for chunk in tdata.chunks(32) {
                            let value = H256::from(chunk);
                            topics.push(value);
                        }

                        let data = Bytes::from(log.get_data());
                        LogEntry {
                            address: params.address,
                            topics: topics,
                            data: data.to_vec(),
                        }
                    })
                    .collect();

                Ok(FinalizationResult {
                    gas_left: U256::from_str(invoke_response.get_gas_left()).unwrap(),
                    apply_state: true,
                    return_data: ReturnData::empty(),
                })
            }
            Err(e) => Err(evm::error::Error::Internal(e.description().to_string())),
        }
    }

    fn call_evm_contract<T, V>(
        &mut self,
        params: ActionParams,
        substate: &mut Substate,
        output: BytesRef,
        tracer: &mut T,
        vm_tracer: &mut V,
    ) -> evm::Result<FinalizationResult>
    where
        T: Tracer,
        V: VMTracer,
    {
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
                Ok(ref res) => {
                    tracer.trace_call(trace_info, gas - res.gas_left, trace_output, traces)
                }
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

    fn call_builtin_contract<T>(
        &mut self,
        params: ActionParams,
        mut output: BytesRef,
        tracer: &mut T,
        builtin: &Builtin,
    ) -> evm::Result<FinalizationResult>
    where
        T: Tracer,
    {
        // if destination is builtin, try to execute it
        if !builtin.is_active(self.info.number) {
            panic!(
                "Consensus failure: engine implementation prematurely enabled built-in at {}",
                params.code_address
            );
        }
        let default = [];
        let data = if let Some(ref d) = params.data {
            d as &[u8]
        } else {
            &default as &[u8]
        };
        let trace_info = tracer.prepare_trace_call(&params);
        let cost = builtin.cost(data);
        if cost <= params.gas {
            builtin.execute(data, &mut output);
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
    }

    fn call_native_contract<T>(
        &mut self,
        params: ActionParams,
        substate: &mut Substate,
        output: BytesRef,
        tracer: &mut T,
        static_call: bool,
        mut contract: Box<NativeContract>,
    ) -> evm::Result<FinalizationResult>
    where
        T: Tracer,
    {
        let mut unconfirmed_substate = Substate::new();
        let mut trace_output = tracer.prepare_trace_output();
        let output_policy = OutputPolicy::Return(output, trace_output.as_mut());
        let res = {
            let mut tracer = NoopTracer;
            let mut vmtracer = NoopVMTracer;
            let economical_model = self.economical_model;
            let mut ext = self.as_externalities(
                OriginInfo::from(&params),
                &mut unconfirmed_substate,
                output_policy,
                &mut tracer,
                &mut vmtracer,
                static_call,
                economical_model,
            );
            contract.exec(params, &mut ext).finalize(ext)
        };
        self.enact_result(&res, substate, unconfirmed_substate);
        trace!(target: "executive", "enacted: substate={:?}\n", substate);
        return res;
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
        if let (true, &ActionValue::Transfer(val)) = (self.payment_required(), &params.value) {
            self.state.sub_balance(&params.sender, &val)?;
            self.state
                .new_contract(&params.address, val + prev_bal, nonce_offset);
        } else {
            self.state
                .new_contract(&params.address, prev_bal, nonce_offset);
        }

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
        let suicide_refunds =
            U256::from(schedule.suicide_refund_gas) * U256::from(substate.suicides.len());
        let refunds_bound = sstore_refunds + suicide_refunds;

        // real ammount to refund
        let gas_left_prerefund = match result {
            Ok(FinalizationResult { gas_left, .. }) => gas_left,
            _ => 0.into(),
        };
        let refunded = cmp::min(refunds_bound, (t.gas - gas_left_prerefund) >> 1);
        let gas_left = gas_left_prerefund + refunded;

        let gas_used = t.gas - gas_left;
        let refund_value = gas_left * t.gas_price();
        let fees_value = gas_used * t.gas_price();

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

        if let EconomicalModel::Charge = self.economical_model {
            self.state.add_balance(&sender, &refund_value)?;
        }

        trace!(
            "exec::finalize: Compensating author: fees_value={}, author={}\n",
            fees_value,
            &self.info.author
        );

        if let EconomicalModel::Charge = self.economical_model {
            self.state.add_balance(&self.info.author, &fees_value)?;
        }

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
            Err(evm::Error::Internal(msg)) => Err(ExecutionError::ExecutionInternal(msg)),
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
    use cita_crypto::{CreateKey, KeyPair};
    use cita_types::{Address, H256, U256};
    use engines::NullEngine;
    use evm::action_params::{ActionParams, ActionValue};
    use evm::env_info::EnvInfo;
    use evm::{Factory, VMType};
    use state::Substate;
    use std::ops::Deref;
    use std::str::FromStr;
    use std::sync::Arc;
    use tests::helpers::*;
    use trace::{ExecutiveTracer, ExecutiveVMTracer};
    use types::transaction::Transaction;

    #[test]
    fn test_transfer_for_store() {
        let keypair = KeyPair::gen_keypair();
        let data_len = 4096;
        let provided_gas = U256::from(100_000);
        let t = Transaction {
            action: Action::Store,
            value: U256::from(0),
            data: vec![0; data_len],
            gas: provided_gas,
            gas_price: U256::one(),
            nonce: U256::zero().to_string(),
            block_limit: 100u64,
            chain_id: 1,
            version: 1,
        }.fake_sign(keypair.address().clone());
        let sender = t.sender();

        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let native_factory = NativeFactory::default();
        let engine = NullEngine::default();
        let mut state = get_temp_state();
        state
            .add_balance(&sender, &U256::from(18 + 100_000))
            .unwrap();
        let mut info = EnvInfo::default();
        info.gas_limit = U256::from(100_000);

        let result = {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Charge,
            );
            let opts = TransactOptions {
                tracing: false,
                vm_tracing: false,
                check_permission: false,
                check_quota: true,
            };
            ex.transact(&t, opts)
        };

        let expected = {
            let base_gas_required = U256::from(100);
            let schedule = Schedule::new_v1();
            let store_gas_used = U256::from(data_len * schedule.create_data_gas);
            let required = base_gas_required.saturating_add(store_gas_used);
            let got = provided_gas;
            ExecutionError::NotEnoughBaseGas { required, got }
        };

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), expected);
    }

    #[test]
    fn test_transfer_for_charge() {
        let keypair = KeyPair::gen_keypair();
        let t = Transaction {
            action: Action::Create,
            value: U256::from(17),
            data: vec![],
            gas: U256::from(100_000),
            gas_price: U256::one(),
            nonce: U256::zero().to_string(),
            block_limit: 100u64,
            chain_id: 1,
            version: 1,
        }.fake_sign(keypair.address().clone());
        let sender = t.sender();
        let contract = contract_address(t.sender(), &U256::zero());

        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let native_factory = NativeFactory::default();
        let engine = NullEngine::default();
        let mut state = get_temp_state();
        state
            .add_balance(&sender, &U256::from(18 + 100_000))
            .unwrap();
        let mut info = EnvInfo::default();
        info.gas_limit = U256::from(100_000);

        let executed = {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Charge,
            );
            let opts = TransactOptions {
                tracing: false,
                vm_tracing: false,
                check_permission: false,
                check_quota: true,
            };
            ex.transact(&t, opts).unwrap()
        };

        assert_eq!(executed.gas, U256::from(100_000));
        assert_eq!(executed.gas_used, U256::from(100));
        assert_eq!(executed.refunded, U256::from(0));
        assert_eq!(executed.logs.len(), 0);
        assert_eq!(executed.contracts_created.len(), 0);
        assert_eq!(
            state.balance(&sender).unwrap(),
            U256::from(18 + 100_000 - 17 - 100)
        );
        assert_eq!(state.balance(&contract).unwrap(), U256::from(17));
        assert_eq!(state.nonce(&sender).unwrap(), U256::from(1));
        // assert_eq!(state.storage_at(&contract, &H256::new()).unwrap(), H256::from(&U256::from(1)));
    }

    #[test]
    fn test_not_enough_cash_for_charge() {
        let keypair = KeyPair::gen_keypair();
        let t = Transaction {
            action: Action::Create,
            value: U256::from(43),
            data: vec![],
            gas: U256::from(100_000),
            gas_price: U256::one(),
            nonce: U256::zero().to_string(),
            block_limit: 100u64,
            chain_id: 1,
            version: 1,
        }.fake_sign(keypair.address().clone());

        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let native_factory = NativeFactory::default();
        let engine = NullEngine::default();
        let mut state = get_temp_state();
        state.add_balance(t.sender(), &U256::from(100_042)).unwrap();
        let mut info = EnvInfo::default();
        info.gas_limit = U256::from(100_000);

        let result = {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Charge,
            );
            let opts = TransactOptions {
                tracing: false,
                vm_tracing: false,
                check_permission: false,
                check_quota: true,
            };
            ex.transact(&t, opts)
        };

        match result {
            Err(ExecutionError::NotEnoughCash { required, got })
                if required == U512::from(100_043) && got == U512::from(100_042) =>
            {
                ()
            }
            _ => assert!(false, "Expected not enough cash error. {:?}", result),
        }
    }

    #[test]
    fn test_not_enough_cash_for_quota() {
        let keypair = KeyPair::gen_keypair();
        let t = Transaction {
            action: Action::Create,
            value: U256::from(43),
            data: vec![],
            gas: U256::from(100_000),
            gas_price: U256::one(),
            nonce: U256::zero().to_string(),
            block_limit: 100u64,
            chain_id: 1,
            version: 1,
        }.fake_sign(keypair.address().clone());

        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let native_factory = NativeFactory::default();
        let engine = NullEngine::default();
        let mut state = get_temp_state();
        let mut info = EnvInfo::default();
        info.gas_limit = U256::from(100_000);

        let result = {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
            );
            let opts = TransactOptions {
                tracing: false,
                vm_tracing: false,
                check_permission: false,
                check_quota: true,
            };
            ex.transact(&t, opts)
        };

        assert!(result.is_ok());
    }

    #[test]
    fn test_create_contract_out_of_gas() {
        logger::silent();
        let source = r#"
pragma solidity ^0.4.19;

contract HelloWorld {
  uint balance;

  function update(uint amount) public returns (address, uint) {
    balance += amount;
    return (msg.sender, balance);
  }
}
"#;
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let nonce = U256::zero();
        let gas_required = U256::from(1000);

        let (deploy_code, _runtime_code) = solc("HelloWorld", source);
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

        let mut ex = Executive::new(
            &mut state,
            &info,
            &engine,
            &factory,
            &native_factory,
            false,
            EconomicalModel::Quota,
        );
        let res = ex.create(params.clone(), &mut substate, &mut tracer, &mut vm_tracer);
        assert!(res.is_err());
        match res {
            Err(e) => assert_eq!(e, evm::Error::OutOfGas),
            _ => unreachable!(),
        }
    }

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
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
            );
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
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
            );
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
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
            );
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
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
            );
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

    #[test]
    fn test_call_instruction() {
        logger::silent();
        let fake_auth = r#"
pragma solidity ^0.4.18;

contract FakeAuth {
    function setAuth() public pure returns(bool) {
        return true;
    }
}
"#;

        let fake_permission_manager = r#"
pragma solidity ^0.4.18;

contract FakeAuth {
    function setAuth() public returns(bool);
}

contract FakePermissionManagement {
    function setAuth(address _auth) public returns(bool) {
        FakeAuth auth = FakeAuth(_auth);
        require(auth.setAuth());
        return true;
    }
}
"#;
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let gas_required = U256::from(100_000);
        let auth_addr = Address::from_str("27ec3678e4d61534ab8a87cf8feb8ac110ddeda5").unwrap();
        let permission_addr =
            Address::from_str("33f4b16d67b112409ab4ac87274926382daacfac").unwrap();

        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let native_factory = NativeFactory::default();
        let mut tracer = ExecutiveTracer::default();
        let mut vm_tracer = ExecutiveVMTracer::toplevel();

        let mut state = get_temp_state();
        let (_, runtime_code) = solc("FakeAuth", fake_auth);
        state.init_code(&auth_addr, runtime_code.clone()).unwrap();

        let (_, runtime_code) = solc("FakePermissionManagement", fake_permission_manager);
        state
            .init_code(&permission_addr, runtime_code.clone())
            .unwrap();

        // 2b2e05c1: setAuth(address)
        let data = "2b2e05c100000000000000000000000027ec3678e4d61534ab8a87cf8feb8ac110ddeda5"
            .from_hex()
            .unwrap();
        let mut params = ActionParams::default();
        params.address = permission_addr.clone();
        params.sender = sender.clone();
        params.gas = gas_required;
        params.code = state.code(&permission_addr).unwrap();
        params.code_hash = state.code_hash(&permission_addr).unwrap();
        params.value = ActionValue::Transfer(U256::from(0));
        params.data = Some(data);

        let info = EnvInfo::default();
        let engine = NullEngine::default();
        let mut substate = Substate::new();
        {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
            );
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
    }
}
