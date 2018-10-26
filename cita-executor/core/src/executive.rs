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
use contracts::{
    grpc::{
        self,
        contract::{
            create_grpc_contract, invoke_grpc_contract, is_create_grpc_address, is_grpc_contract,
        },
        grpc_vm::extract_logs_from_response,
        service_registry,
    },
    native::factory::{Contract as NativeContract, Factory as NativeFactory},
    solc::{permission_management::contains_resource, Resource},
};
use crossbeam;
use engines::Engine;
use error::ExecutionError;
use evm::action_params::{ActionParams, ActionValue};
use evm::call_type::CallType;
use evm::env_info::EnvInfo;
use evm::{self, Factory, FinalizationResult, Finalize, ReturnData, Schedule};
pub use executed::{Executed, ExecutionResult};
use externalities::*;
use libexecutor::executor::EconomicalModel;
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
///amend account's balance
const AMEND_ACCOUNT_BALANCE: u32 = 5;

/// Returns new address created from address and given nonce.
pub fn contract_address(address: &Address, nonce: &U256) -> Address {
    use rlp::RlpStream;

    let mut stream = RlpStream::new_list(2);
    stream.append(address);
    stream.append(nonce);
    From::from(stream.out().crypt_hash())
}

/// Check the sender's permission
#[allow(unknown_lints, clippy::implicit_hasher)] // TODO clippy
pub fn check_permission(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    t: &SignedTransaction,
    options: TransactOptions,
) -> Result<(), ExecutionError> {
    let sender = *t.sender();

    if options.check_send_tx_permission {
        check_send_tx(group_accounts, account_permissions, &sender)?;
    }

    match t.action {
        Action::Create => {
            if options.check_create_contract_permission {
                check_create_contract(group_accounts, account_permissions, &sender)?;
            }
        }
        Action::Call(address) => {
            if options.check_permission {
                let group_management_addr =
                    Address::from_str(reserved_addresses::GROUP_MANAGEMENT).unwrap();
                trace!("t.data {:?}", t.data);

                if t.data.is_empty() {
                    // Transfer transaction, no function call
                    return Ok(());
                }

                if t.data.len() < 4 {
                    return Err(ExecutionError::TransactionMalformed(
                        "The length of transaction data is less than four bytes".to_string(),
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
                        &t.data[0..4],
                        &H160::from(&t.data[16..36]),
                    )?;
                }

                check_call_contract(
                    group_accounts,
                    account_permissions,
                    &sender,
                    &address,
                    &t.data[0..4],
                )?;
            }
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
    let has_permission = has_resource(
        group_accounts,
        account_permissions,
        account,
        &cont,
        &func[..],
    );

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
    let has_permission = has_resource(
        group_accounts,
        account_permissions,
        account,
        &cont,
        &func[..],
    );

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
    func: &[u8],
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
    func: &[u8],
    param: &Address,
) -> Result<(), ExecutionError> {
    let has_permission = contains_resource(account_permissions, account, *cont, func);

    trace!("Sender has call contract permission: {:?}", has_permission);

    if !has_permission && !contains_resource(account_permissions, param, *cont, func) {
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
    func: &[u8],
) -> bool {
    let groups = get_groups(group_accounts, account);

    if !contains_resource(account_permissions, account, *cont, func) {
        for group in groups {
            if contains_resource(account_permissions, &group, *cont, func) {
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
    /// Check sender's send_tx permission
    pub check_send_tx_permission: bool,
    /// Check sender's create_contract permission
    pub check_create_contract_permission: bool,
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
    check_fee_back_platform: bool,
    chain_owner: Address,
}

impl<'a, B: 'a + StateBackend> Executive<'a, B> {
    /// Basic constructor.
    #[allow(unknown_lints, clippy::too_many_arguments)] // TODO clippy
    pub fn new(
        state: &'a mut State<B>,
        info: &'a EnvInfo,
        engine: &'a Engine,
        vm_factory: &'a Factory,
        native_factory: &'a NativeFactory,
        static_flag: bool,
        economical_model: EconomicalModel,
        check_fee_back_platform: bool,
        chain_owner: Address,
    ) -> Self {
        Executive {
            state,
            info,
            engine,
            vm_factory,
            native_factory,
            depth: 0,
            static_flag,
            economical_model,
            check_fee_back_platform,
            chain_owner,
        }
    }

    pub fn payment_required(&self) -> bool {
        self.economical_model == EconomicalModel::Charge
    }

    /// Populates executive from parent properties. Increments executive depth.
    #[allow(unknown_lints, clippy::too_many_arguments)] // TODO clippy
    pub fn from_parent(
        state: &'a mut State<B>,
        info: &'a EnvInfo,
        engine: &'a Engine,
        vm_factory: &'a Factory,
        native_factory: &'a NativeFactory,
        parent_depth: usize,
        static_flag: bool,
        economical_model: EconomicalModel,
        check_fee_back_platform: bool,
        chain_owner: Address,
    ) -> Self {
        Executive {
            state,
            info,
            engine,
            vm_factory,
            native_factory,
            depth: parent_depth + 1,
            static_flag,
            economical_model,
            check_fee_back_platform,
            chain_owner,
        }
    }

    /// Creates `Externalities` from `Executive`.
    #[allow(unknown_lints, clippy::too_many_arguments)] // TODO clippy
    pub fn as_externalities<'any, T, V>(
        &'any mut self,
        origin_info: OriginInfo,
        substate: &'any mut Substate,
        output: OutputPolicy<'any, 'any>,
        tracer: &'any mut T,
        vm_tracer: &'any mut V,
        static_call: bool,
        economical_model: EconomicalModel,
        check_fee_back_platform: bool,
        chain_owner: Address,
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
            check_fee_back_platform,
            chain_owner,
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

    fn transact_set_balance(&mut self, data: &[u8]) -> bool {
        if data.len() < 52 {
            return false;
        }
        let account = H160::from(&data[0..20]);
        let balance = U256::from(&data[20..52]);
        self.state
            .balance(&account)
            .and_then(|now_val| {
                if now_val >= balance {
                    self.state.sub_balance(&account, &(now_val - balance))
                } else {
                    self.state.add_balance(&account, &(balance - now_val))
                }
            })
            .is_ok()
    }

    fn transact_set_kv_h256(&mut self, data: &[u8]) -> bool {
        let len = data.len();
        if len < 84 {
            return false;
        }
        let loop_num: usize = (len - 20) / (32 * 2);
        let account = H160::from(&data[0..20]);

        for i in 0..loop_num {
            let base = 20 + 32 * 2 * i;
            let key = H256::from_slice(&data[base..base + 32]);
            let val = H256::from_slice(&data[base + 32..base + 32 * 2]);
            if self.state.set_storage(&account, key, val).is_err() {
                return false;
            }
        }
        true
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

        check_permission(
            &self.state.group_accounts,
            &self.state.account_permissions,
            t,
            options,
        )?;

        let schedule = Schedule::new_v1();

        let base_gas_required = match t.action {
            Action::Create => schedule.tx_create_gas,
            Action::GoCreate => schedule.tx_create_gas,
            _ => schedule.tx_gas,
        };

        if sender != Address::zero() && t.gas < U256::from(base_gas_required) {
            return Err(ExecutionError::NotEnoughBaseGas {
                required: U256::from(base_gas_required),
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

        if t.action == Action::AbiStore && !self.transact_set_abi(&t.data) {
            return Err(ExecutionError::TransactionMalformed(
                "Account doesn't exist".to_owned(),
            ));
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

        let init_gas = t.gas - U256::from(base_gas_required);
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
                        required: U256::from(base_gas_required).saturating_add(store_gas_used),
                        got: t.gas,
                    });
                }
            }
            Action::GoCreate => {
                let address = Address::default();
                let params = ActionParams {
                    code_address: address,
                    address,
                    sender,
                    origin: sender,
                    gas: init_gas,
                    gas_price: t.gas_price(),
                    value: ActionValue::Transfer(t.value),
                    code: self.state.code(&address)?,
                    code_hash: self.state.code_hash(&address)?,
                    data: Some(t.data.clone()),
                    call_type: CallType::Call,
                };
                trace!(target: "executive", "call: {:?}", params);
                let mut out = vec![];
                (
                    self.call_grpc_contract(&params, &mut substate, &BytesRef::Flexible(&mut out)),
                    out,
                )
            }
            Action::Create => {
                let new_address = contract_address(&sender, &nonce);
                let params = ActionParams {
                    code_address: new_address,
                    code_hash: t.data.crypt_hash(),
                    address: new_address,
                    sender,
                    origin: sender,
                    gas: init_gas,
                    gas_price: t.gas_price(),
                    value: ActionValue::Transfer(t.value),
                    code: Some(Arc::new(t.data.clone())),
                    data: None,
                    call_type: CallType::None,
                };
                (
                    self.create(&params, &mut substate, &mut tracer, &mut vm_tracer),
                    vec![],
                )
            }
            Action::AmendData => {
                let amend_data_address: Address = reserved_addresses::AMEND_ADDRESS.into();
                let params = ActionParams {
                    code_address: amend_data_address,
                    address: amend_data_address,
                    sender,
                    origin: sender,
                    gas: init_gas,
                    gas_price: t.gas_price(),
                    value: ActionValue::Apparent(t.value),
                    code: None,
                    code_hash: HASH_EMPTY,
                    data: Some(t.data.clone()),
                    call_type: CallType::Call,
                };
                trace!(target: "executive", "amend data: {:?}", params);
                let mut out = vec![];
                (
                    self.call(
                        &params,
                        &mut substate,
                        BytesRef::Flexible(&mut out),
                        &mut tracer,
                        &mut vm_tracer,
                    ),
                    out,
                )
            }
            Action::Call(ref address) => {
                let params = ActionParams {
                    code_address: *address,
                    address: *address,
                    sender,
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
                        &params,
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
        params: &ActionParams,
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
            let check_fee_back_platform = self.check_fee_back_platform;
            let chain_owner = self.chain_owner;
            let mut ext = self.as_externalities(
                OriginInfo::from(params),
                unconfirmed_substate,
                output_policy,
                tracer,
                vm_tracer,
                static_call,
                economical_model,
                check_fee_back_platform,
                chain_owner,
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
            let check_fee_back_platform = self.check_fee_back_platform;
            let chain_owner = self.chain_owner;
            let mut ext = self.as_externalities(
                OriginInfo::from(params),
                unconfirmed_substate,
                output_policy,
                tracer,
                vm_tracer,
                static_call,
                economical_model,
                check_fee_back_platform,
                chain_owner,
            );

            scope.spawn(move || {
                vm_factory
                    .create(params.gas)
                    .exec(params, &mut ext)
                    .finalize(ext)
            })
        })
        .join()
    }

    /// Calls contract function with given contract params.
    /// NOTE. It does not finalize the transaction (doesn't do refunds, nor suicides).
    /// Modifies the substate and the output.
    /// Returns either gas_left or `evm::Error`.
    pub fn call<T, V>(
        &mut self,
        params: &ActionParams,
        substate: &mut Substate,
        output: BytesRef,
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

        if let Some(native_contract) = self.native_factory.new_contract(params.code_address) {
            // check and call Native Contract
            self.call_native_contract(
                params,
                substate,
                output,
                tracer,
                static_call,
                native_contract,
            )
        } else if self.is_amend_data_address(params.code_address) {
            let res = self.call_amend_data(params, substate, &output);
            self.enact_self_defined_res(&res);
            res
        } else if is_create_grpc_address(params.code_address)
            || is_grpc_contract(params.code_address)
        {
            let res = self.call_grpc_contract(params, substate, &output);
            self.enact_self_defined_res(&res);
            res
        } else if let Some(builtin) = self.engine.builtin(&params.code_address, self.info.number) {
            // check and call Builtin contract
            self.call_builtin_contract(params, output, tracer, builtin)
        } else {
            // call EVM contract
            self.call_evm_contract(params, substate, output, tracer, vm_tracer)
        }
    }

    fn is_amend_data_address(&self, address: Address) -> bool {
        let amend_address: Address = reserved_addresses::AMEND_ADDRESS.into();
        amend_address == address
    }

    fn enact_self_defined_res(&mut self, result: &evm::Result<FinalizationResult>) {
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
            }
        }
    }

    fn call_amend_data(
        &mut self,
        params: &ActionParams,
        _substate: &mut Substate,
        _output: &BytesRef,
    ) -> evm::Result<FinalizationResult> {
        // Must send from admin address
        if Some(params.origin) != self.state.super_admin_account {
            return Err(evm::error::Error::Internal("no permission".to_owned()));
        }
        let atype = params.value.value().low_u32();
        let mut result = FinalizationResult {
            gas_left: params.gas,
            apply_state: true,
            return_data: ReturnData::empty(),
        };
        match atype {
            AMEND_ABI => {
                if self.transact_set_abi(&(params.data.to_owned().unwrap())) {
                    Ok(result)
                } else {
                    Err(evm::error::Error::Internal(
                        "Account doesn't exist".to_owned(),
                    ))
                }
            }
            AMEND_CODE => {
                if self.transact_set_code(&(params.data.to_owned().unwrap())) {
                    Ok(result)
                } else {
                    Err(evm::error::Error::Internal(
                        "Account doesn't exist".to_owned(),
                    ))
                }
            }
            AMEND_KV_H256 => {
                if self.transact_set_kv_h256(&(params.data.to_owned().unwrap())) {
                    Ok(result)
                } else {
                    Err(evm::error::Error::Internal(
                        "Account doesn't exist".to_owned(),
                    ))
                }
            }
            AMEND_GET_KV_H256 => {
                if let Some(v) = self.transact_get_kv_h256(&(params.data.to_owned().unwrap())) {
                    let data = v.to_vec();
                    let size = data.len();
                    result.return_data = ReturnData::new(data, 0, size);
                    Ok(result)
                } else {
                    Err(evm::error::Error::Internal(
                        "May be incomplete trie error".to_owned(),
                    ))
                }
            }

            AMEND_ACCOUNT_BALANCE => {
                if self.transact_set_balance(&(params.data.to_owned().unwrap())) {
                    Ok(result)
                } else {
                    Err(evm::error::Error::Internal(
                        "Account doesn't exist or incomplete trie error".to_owned(),
                    ))
                }
            }

            _ => Ok(result),
        }
    }

    fn call_grpc_contract(
        &mut self,
        params: &ActionParams,
        substate: &mut Substate,
        _output: &BytesRef,
    ) -> evm::Result<FinalizationResult> {
        let is_create = is_create_grpc_address(params.code_address);
        let address = if is_create {
            match params.data.clone() {
                Some(data) => Address::from_slice(&data),
                _ => {
                    return Err(evm::error::Error::Internal(
                        "GRPC contract creation without data field".to_string(),
                    ));
                }
            }
        } else {
            params.code_address
        };

        let connect_info = match service_registry::find_contract(address, !is_create) {
            Some(contract_state) => contract_state.conn_info,
            None => {
                return Err(evm::error::Error::Internal(format!(
                    "can't find grpc contract from address: {:?}",
                    address
                )));
            }
        };

        let response = if is_create {
            service_registry::enable_contract(address);
            create_grpc_contract(self.info, &params, self.state, true, true, &connect_info)
        } else {
            invoke_grpc_contract(self.info, &params, self.state, true, true, &connect_info)
        };
        match response {
            Ok(invoke_response) => {
                // store grpc return storages to stateDB
                for storage in &invoke_response.get_storages()[..] {
                    let key = storage.get_key();
                    let value = storage.get_value();
                    trace!("recv resp: {:?}", storage);
                    trace!("key: {:?}, value: {:?}", key, value);
                    grpc::storage::set_storage(self.state, params.address, key, value).unwrap();
                }

                // update contract_state.height
                service_registry::set_enable_contract_height(params.address, self.info.number);
                substate.logs = extract_logs_from_response(params.address, &invoke_response);
                let message = invoke_response.get_message();

                Ok(FinalizationResult {
                    gas_left: U256::from_str(invoke_response.get_gas_left()).unwrap(),
                    apply_state: true,
                    return_data: ReturnData::new(message.as_bytes().to_vec(), 0, message.len()),
                })
            }
            Err(e) => Err(evm::error::Error::Internal(e.description().to_string())),
        }
    }

    fn call_evm_contract<T, V>(
        &mut self,
        params: &ActionParams,
        substate: &mut Substate,
        output: BytesRef,
        tracer: &mut T,
        vm_tracer: &mut V,
    ) -> evm::Result<FinalizationResult>
    where
        T: Tracer,
        V: VMTracer,
    {
        let trace_info = tracer.prepare_trace_call(params);
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
        params: &ActionParams,
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
        let trace_info = tracer.prepare_trace_call(params);
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
        params: &ActionParams,
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
            let check_fee_back_platform = self.check_fee_back_platform;
            let chain_owner = self.chain_owner;
            let mut ext = self.as_externalities(
                OriginInfo::from(&params),
                &mut unconfirmed_substate,
                output_policy,
                &mut tracer,
                &mut vmtracer,
                static_call,
                economical_model,
                check_fee_back_platform,
                chain_owner,
            );
            contract.exec(&params, &mut ext).finalize(ext)
        };
        self.enact_result(&res, substate, unconfirmed_substate);
        trace!(target: "executive", "enacted: substate={:?}\n", substate);
        res
    }

    /// Creates contract with given contract params.
    /// NOTE. It does not finalize the transaction (doesn't do refunds, nor suicides).
    /// Modifies the substate.
    pub fn create<T, V>(
        &mut self,
        params: &ActionParams,
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
                &params,
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
    #[allow(unknown_lints, clippy::too_many_arguments)] // TODO clippy
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
            if self.check_fee_back_platform {
                // check_fee_back_platform is true, but chain_owner not set, fee still back to author(miner)
                if self.chain_owner == Address::from(0) {
                    self.state
                        .add_balance(&self.info.author, &fees_value)
                        .expect("Add balance to author(miner) must success");
                } else {
                    self.state
                        .add_balance(&self.chain_owner, &fees_value)
                        .expect("Add balance to chain owner must success");
                }
            } else {
                self.state
                    .add_balance(&self.info.author, &fees_value)
                    .expect("Add balance to author(miner) must success");
            }
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
                output,
                trace,
                vm_trace,
                state_diff: None,
                account_nonce,
            }),
            Ok(r) => Ok(Executed {
                exception: if r.apply_state {
                    None
                } else {
                    Some(evm::Error::Reverted)
                },
                gas: t.gas,
                gas_used,
                refunded,
                cumulative_gas_used: self.info.gas_used + gas_used,
                logs: substate.logs,
                contracts_created: substate.contracts_created,
                output,
                trace,
                vm_trace,
                state_diff: None,
                account_nonce,
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
    use evm::Schedule;
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
            chain_id: 1.into(),
            version: 1,
        }
        .fake_sign(keypair.address().clone());
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
                false,
                Address::from(0),
            );
            let opts = TransactOptions {
                tracing: false,
                vm_tracing: false,
                check_permission: false,
                check_quota: true,
                check_send_tx_permission: false,
                check_create_contract_permission: false,
            };
            ex.transact(&t, opts)
        };

        let schedule = Schedule::new_v1();
        let expected = {
            let base_gas_required = U256::from(schedule.tx_gas);
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
            chain_id: 1.into(),
            version: 1,
        }
        .fake_sign(keypair.address().clone());
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
                false,
                Address::from(0),
            );
            let opts = TransactOptions {
                tracing: false,
                vm_tracing: false,
                check_permission: false,
                check_quota: true,
                check_send_tx_permission: false,
                check_create_contract_permission: false,
            };
            ex.transact(&t, opts).unwrap()
        };

        let schedule = Schedule::new_v1();
        assert_eq!(executed.gas, U256::from(100_000));

        // Actually, this is an Action::Create transaction
        assert_eq!(executed.gas_used, U256::from(schedule.tx_create_gas));
        assert_eq!(executed.refunded, U256::from(0));
        assert_eq!(executed.logs.len(), 0);
        assert_eq!(executed.contracts_created.len(), 0);
        assert_eq!(
            state.balance(&sender).unwrap(),
            U256::from(18 + 100_000 - 17 - schedule.tx_create_gas)
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
            chain_id: 1.into(),
            version: 1,
        }
        .fake_sign(keypair.address().clone());

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
                false,
                Address::from(0),
            );
            let opts = TransactOptions {
                tracing: false,
                vm_tracing: false,
                check_permission: false,
                check_quota: true,
                check_send_tx_permission: false,
                check_create_contract_permission: false,
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
            chain_id: 1.into(),
            version: 1,
        }
        .fake_sign(keypair.address().clone());

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
                false,
                Address::from(0),
            );
            let opts = TransactOptions {
                tracing: false,
                vm_tracing: false,
                check_permission: false,
                check_quota: true,
                check_send_tx_permission: false,
                check_create_contract_permission: false,
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
        let schedule = Schedule::new_v1();
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let nonce = U256::zero();
        let gas_required = U256::from(schedule.tx_gas + 1000);

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
            false,
            Address::from(0),
        );
        let res = ex.create(&params, &mut substate, &mut tracer, &mut vm_tracer);
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
        let schedule = Schedule::new_v1();
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let nonce = U256::zero();
        let gas_required = U256::from(schedule.tx_gas + 100_000);

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
                false,
                Address::from(0),
            );
            let _ = ex.create(&params, &mut substate, &mut tracer, &mut vm_tracer);
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
        let schedule = Schedule::new_v1();
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let gas_required = U256::from(schedule.tx_gas + 100_000);
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
                false,
                Address::from(0),
            );
            let mut out = vec![];
            let _ = ex.call(
                &params,
                &mut substate,
                BytesRef::Fixed(&mut out),
                &mut tracer,
                &mut vm_tracer,
            );
        };

        // it was supposed that value's address is balance.
        assert_eq!(
            state
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
        let schedule = Schedule::new_v1();
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let gas_required = U256::from(schedule.tx_gas + 100_000);
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
                false,
                Address::from(0),
            );
            let mut out = vec![];
            let res = ex.call(
                &params,
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

        // it was supposed that value's address is balance.
        assert_eq!(
            state
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
        let schedule = Schedule::new_v1();
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let gas_required = U256::from(schedule.tx_gas + 100_000);
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
                false,
                Address::from(0),
            );
            let mut out = vec![];
            let res = ex.call(
                &params,
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

        // it was supposed that value's address is balance.
        assert_eq!(
            state
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
        let schedule = Schedule::new_v1();
        let sender = Address::from_str("cd1722f3947def4cf144679da39c4c32bdc35681").unwrap();
        let gas_required = U256::from(schedule.tx_gas + 100_000);
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
                false,
                Address::from(0),
            );
            let mut out = vec![];
            let res = ex.call(
                &params,
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
