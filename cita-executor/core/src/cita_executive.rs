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

use crate::data_provider::{BlockDataProvider, DataProvider, Store as VMSubState};
use cita_trie::DB;
use cita_types::{Address, H160, H256, U256, U512};
use cita_vm::{
    evm::{
        self, Context as EVMContext, Contract, Error as EVMError, InterpreterParams,
        InterpreterResult, Log as EVMLog,
    },
    state::{State, StateObjectInfo},
    summary, Error as VMError,
};
use rlp::RlpStream;
use std::cell::RefCell;
use std::sync::Arc;
use types::Bytes;

use crate::authentication::check_permission;
use crate::cita_vm_helper::{call_pure, get_interpreter_conf};
use crate::contracts::native::factory::Factory as NativeFactory;
use crate::exception::ExecutedException;
use crate::libexecutor::economical_model::EconomicalModel;
use crate::libexecutor::sys_config::BlockSysConfig;
use crate::tx_gas_schedule::TxGasSchedule;
use crate::types::context::Context;
use crate::types::errors::AuthenticationError;
use crate::types::errors::ExecutionError;
use crate::types::log::Log;
use crate::types::transaction::{Action, SignedTransaction};
use ethbloom::{Bloom, Input as BloomInput};

// use rs_contracts::factory::ContractsFactory;
// use rs_contracts::storage::db_contracts::ContractsDB;
use crate::rs_contracts::factory::ContractsFactory;
use crate::rs_contracts::storage::db_contracts::ContractsDB;

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

/// See: https://github.com/ethereum/EIPs/issues/659
const MAX_CREATE_CODE_SIZE: u64 = std::u64::MAX;

// FIXME: CITAExecutive need rename to Executive after all works ready.
pub struct CitaExecutive<'a, B> {
    block_provider: Arc<BlockDataProvider>,
    state_provider: Arc<RefCell<State<B>>>,
    contracts_db: Arc<ContractsDB>,
    context: &'a Context,
    economical_model: EconomicalModel,
}

impl<'a, B: DB + 'static> CitaExecutive<'a, B> {
    pub fn new(
        block_provider: Arc<BlockDataProvider>,
        state: Arc<RefCell<State<B>>>,
        contracts_db: Arc<ContractsDB>,
        context: &'a Context,
        economical_model: EconomicalModel,
    ) -> Self {
        Self {
            block_provider,
            state_provider: state,
            contracts_db,
            context,
            economical_model,
        }
    }

    pub fn exec(
        &mut self,
        t: &SignedTransaction,
        conf: &BlockSysConfig,
    ) -> Result<ExecutedResult, ExecutionError> {
        let sender = *t.sender();
        let nonce = self.state_provider.borrow_mut().nonce(&sender)?;
        trace!("transaction sender: {:?}, nonce: {:?}", sender, nonce);
        self.state_provider.borrow_mut().inc_nonce(&sender)?;

        trace!(
            "call contract permission should be check: {}",
            (*conf).check_options.call_permission
        );

        check_permission(
            &conf.group_accounts,
            &conf.account_permissions,
            t,
            conf.check_options,
        )?;

        let tx_gas_schedule = TxGasSchedule::default();
        let base_gas_required = match t.action {
            Action::Create => tx_gas_schedule.tx_create_gas,
            _ => tx_gas_schedule.tx_gas,
        } + match t.version {
            0...2 => 0,
            _ => t.data.len() * tx_gas_schedule.tx_data_non_zero_gas,
        };
        if sender != Address::zero() && t.gas < U256::from(base_gas_required) {
            // FIXME: It is better to change NotEnoughBaseGas to
            //    NotEnoughBaseGas {
            //        required: U256,
            //        got: U256,
            //    }
            // Need to change VMError defined in cita-vm.
            return Err(ExecutionError::NotEnoughBaseGas);
        }

        if t.action == Action::AbiStore && !self.transact_set_abi(&t.data) {
            return Err(ExecutionError::InvalidTransaction);
        }

        // Prepaid t.gas for the transaction.
        self.prepaid(t.sender(), t.gas, t.gas_price, t.value)?;
        let init_gas = t.gas - U256::from(base_gas_required);

        let mut store = VMSubState::default();
        store.evm_context = build_evm_context(&self.context.clone());
        store.evm_cfg = get_interpreter_conf();
        let store = Arc::new(RefCell::new(store));

        let result = match t.action {
            Action::Store | Action::AbiStore => {
                // Maybe use tx_gas_schedule.tx_data_non_zero_gas for each byte store, it is more reasonable.
                // But for the data compatible, just let it as tx_gas_schedule.create_data_gas for now.
                let store_gas_used = U256::from(t.data.len() * tx_gas_schedule.create_data_gas);
                if let Some(gas_left) = init_gas.checked_sub(store_gas_used) {
                    Ok(InterpreterResult::Normal(vec![], gas_left.as_u64(), vec![]))
                } else {
                    // FIXME: Should not return an error after self.prepaid().
                    // But for compatibility, should keep this. Need to be upgrade in new version.
                    return Err(ExecutionError::NotEnoughBaseGas);
                }
            }
            Action::Create => {
                // Note: Fees has been handle in cita_vm.
                let params = ExecutiveParams {
                    code_address: None,
                    sender,
                    to_address: None,
                    gas: init_gas,
                    gas_price: t.gas_price(),
                    value: t.value,
                    nonce,
                    data: Some(t.data.clone()),
                };

                let mut vm_exec_params = build_vm_exec_params(&params, self.state_provider.clone());
                if !self.payment_required() {
                    vm_exec_params.disable_transfer_value = true;
                }
                create(
                    self.block_provider.clone(),
                    self.state_provider.clone(),
                    store.clone(),
                    self.contracts_db.clone(),
                    &vm_exec_params.into(),
                    CreateKind::FromAddressAndNonce,
                )
            }

            Action::AmendData => {
                trace!("amend action, conf admin {:?}", conf.super_admin_account);
                if let Some(admin) = conf.super_admin_account {
                    if *t.sender() != admin {
                        return Err(ExecutionError::Authentication(
                            AuthenticationError::NoTransactionPermission,
                        ));
                    }
                } else {
                    return Err(ExecutionError::Authentication(
                        AuthenticationError::NoTransactionPermission,
                    ));
                }

                // Backup used in case of running error
                self.state_provider.borrow_mut().checkpoint();

                match self.call_amend_data(t.value, Some(t.data.clone())) {
                    Ok(Some(val)) => {
                        // Discard the checkpoint because of amend data ok.
                        self.state_provider.borrow_mut().discard_checkpoint();
                        Ok(InterpreterResult::Normal(
                            val.to_vec(),
                            init_gas.as_u64(),
                            vec![],
                        ))
                    }
                    Ok(None) => {
                        // Discard the checkpoint because of amend data ok.
                        self.state_provider.borrow_mut().discard_checkpoint();
                        Ok(InterpreterResult::Normal(vec![], init_gas.as_u64(), vec![]))
                    }
                    Err(e) => {
                        // Need to revert the state.
                        self.state_provider.borrow_mut().revert_checkpoint();
                        Err(e)
                    }
                }
            }
            Action::Call(ref address) => {
                let params = ExecutiveParams {
                    code_address: Some(*address),
                    sender,
                    to_address: Some(*address),
                    gas: init_gas,
                    gas_price: t.gas_price(),
                    value: t.value,
                    nonce,
                    data: Some(t.data.clone()),
                };
                let mut vm_exec_params = build_vm_exec_params(&params, self.state_provider.clone());
                if !self.payment_required() {
                    vm_exec_params.disable_transfer_value = true;
                }
                call(
                    self.block_provider.clone(),
                    self.state_provider.clone(),
                    store.clone(),
                    self.contracts_db.clone(),
                    &vm_exec_params.into(),
                )
            }
        };

        let mut finalize_result = self.finalize(result, store, t.gas, sender, t.gas_price());
        finalize_result.account_nonce = nonce;
        Ok(finalize_result)
    }

    fn finalize(
        &mut self,
        result: Result<InterpreterResult, VMError>,
        store: Arc<RefCell<VMSubState>>,
        gas_limit: U256,
        sender: Address,
        gas_price: U256,
    ) -> ExecutedResult {
        let mut finalize_result = ExecutedResult::default();

        match result {
            Ok(InterpreterResult::Normal(output, gas_left, logs)) => {
                if self.payment_required() {
                    let refund = get_refund(store.clone(), sender, gas_limit.as_u64(), gas_left);
                    if let Err(e) = liquidtion(
                        self.state_provider.clone(),
                        store.clone(),
                        sender,
                        gas_price,
                        gas_limit.as_u64(),
                        gas_left,
                        refund,
                    ) {
                        finalize_result.exception = Some(ExecutedException::VM(e));
                        return finalize_result;
                    }
                }
                // Handle self destruct: Kill it.
                // Note: must after ends of the transaction.
                for e in store.borrow_mut().selfdestruct.drain() {
                    self.state_provider.borrow_mut().kill_contract(&e);
                }
                self.state_provider
                    .borrow_mut()
                    .kill_garbage(&store.borrow().inused.clone());
                finalize_result.quota_used = gas_limit - U256::from(gas_left);
                finalize_result.quota_left = U256::from(gas_left);
                finalize_result.logs = transform_logs(logs.clone());
                finalize_result.logs_bloom = logs_to_bloom(&finalize_result.logs);

                trace!(
                    "Get data after executed the transaction [Normal]: output {:?}, logs {:?}",
                    output,
                    logs
                );
                finalize_result.output = output;
            }
            Ok(InterpreterResult::Revert(output, gas_left)) => {
                if self.payment_required() {
                    if let Err(e) = liquidtion(
                        self.state_provider.clone(),
                        store.clone(),
                        sender,
                        gas_price,
                        gas_limit.as_u64(),
                        gas_left,
                        0,
                    ) {
                        finalize_result.exception = Some(ExecutedException::VM(e));
                        return finalize_result;
                    }
                }
                self.state_provider
                    .borrow_mut()
                    .kill_garbage(&store.borrow().inused.clone());

                finalize_result.quota_used = gas_limit - U256::from(gas_left);
                finalize_result.quota_left = U256::from(gas_left);
                finalize_result.exception = Some(ExecutedException::Reverted);
                trace!(
                    "Get data after executed the transaction [Revert]: {:?}",
                    output
                );
            }
            Ok(InterpreterResult::Create(output, gas_left, logs, addr)) => {
                if self.payment_required() {
                    let refund = get_refund(store.clone(), sender, gas_limit.as_u64(), gas_left);
                    if let Err(e) = liquidtion(
                        self.state_provider.clone(),
                        store.clone(),
                        sender,
                        gas_price,
                        gas_limit.as_u64(),
                        gas_left,
                        refund,
                    ) {
                        finalize_result.exception = Some(ExecutedException::VM(e));
                        return finalize_result;
                    }
                }

                for e in store.borrow_mut().selfdestruct.drain() {
                    self.state_provider.borrow_mut().kill_contract(&e);
                }
                self.state_provider
                    .borrow_mut()
                    .kill_garbage(&store.borrow().inused.clone());
                finalize_result.quota_used = gas_limit - U256::from(gas_left);
                finalize_result.quota_left = U256::from(gas_left);
                finalize_result.logs = transform_logs(logs);
                finalize_result.logs_bloom = logs_to_bloom(&finalize_result.logs);
                finalize_result.contract_address = Some(addr);

                trace!(
                "Get data after executed the transaction [Create], contract address: {:?}, contract data : {:?}",
                finalize_result.contract_address, output
                );
            }
            Err(e) => {
                if self.payment_required() {
                    if let Err(e) = liquidtion(
                        self.state_provider.clone(),
                        store.clone(),
                        sender,
                        gas_price,
                        gas_limit.as_u64(),
                        0,
                        0,
                    ) {
                        finalize_result.exception = Some(ExecutedException::VM(e));
                        return finalize_result;
                    }
                }
                self.state_provider
                    .borrow_mut()
                    .kill_garbage(&store.borrow().inused.clone());

                finalize_result.exception = Some(ExecutedException::VM(e));
                finalize_result.quota_used = gas_limit;
                finalize_result.quota_left = U256::from(0);
            }
        }

        finalize_result
    }

    fn payment_required(&self) -> bool {
        self.economical_model == EconomicalModel::Charge
    }

    fn prepaid(
        &mut self,
        sender: &H160,
        gas: U256,
        gas_price: U256,
        value: U256,
    ) -> Result<(), ExecutionError> {
        if self.payment_required() {
            let balance = self.state_provider.borrow_mut().balance(&sender)?;
            let gas_cost = gas.full_mul(gas_price);
            let total_cost = U512::from(value) + gas_cost;

            // Avoid unaffordable transactions
            let balance512 = U512::from(balance);
            if balance512 < total_cost {
                return Err(ExecutionError::NotEnoughBalance);
            }
            self.state_provider
                .borrow_mut()
                .sub_balance(&sender, U256::from(gas_cost))?;
        }
        Ok(())
    }

    fn transact_set_abi(&mut self, data: &[u8]) -> bool {
        if data.len() <= 20 {
            return false;
        }
        let account = H160::from(&data[0..20]);
        let abi = &data[20..];

        let account_exist = self
            .state_provider
            .borrow_mut()
            .exist(&account)
            .unwrap_or(false);
        info!("Account-{:?} in state is {:?}", account, account_exist);

        account_exist
            && self
                .state_provider
                .borrow_mut()
                .set_abi(&account, abi.to_vec())
                .is_ok()
    }

    fn transact_set_code(&mut self, data: &[u8]) -> bool {
        if data.len() <= 20 {
            return false;
        }
        let account = H160::from(&data[0..20]);
        let code = &data[20..];
        self.state_provider
            .borrow_mut()
            .set_code(&account, code.to_vec())
            .is_ok()
    }

    fn transact_set_balance(&mut self, data: &[u8]) -> bool {
        if data.len() < 52 {
            return false;
        }
        let account = H160::from(&data[0..20]);
        let balance = U256::from(&data[20..52]);

        let now_val = self
            .state_provider
            .borrow_mut()
            .balance(&account)
            .unwrap_or_default();
        if now_val > balance {
            self.state_provider
                .borrow_mut()
                .sub_balance(&account, now_val - balance)
                .is_ok()
        } else {
            self.state_provider
                .borrow_mut()
                .add_balance(&account, balance - now_val)
                .is_ok()
        }
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
            if self
                .state_provider
                .borrow_mut()
                .set_storage(&account, key, val)
                .is_err()
            {
                return false;
            }
        }
        true
    }

    fn transact_get_kv_h256(&mut self, data: &[u8]) -> Option<H256> {
        let account = H160::from(&data[0..20]);
        let key = H256::from_slice(&data[20..52]);
        self.state_provider
            .borrow_mut()
            .get_storage(&account, &key)
            .ok()
    }

    fn call_amend_data(
        &mut self,
        value: U256,
        data: Option<Bytes>,
    ) -> Result<Option<H256>, VMError> {
        let amend_type = value.low_u32();
        match amend_type {
            AMEND_ABI => {
                if self.transact_set_abi(&(data.to_owned().unwrap())) {
                    Ok(None)
                } else {
                    Err(VMError::Evm(EVMError::Internal(
                        "Account doesn't exist".to_owned(),
                    )))
                }
            }
            AMEND_CODE => {
                if self.transact_set_code(&(data.to_owned().unwrap())) {
                    Ok(None)
                } else {
                    Err(VMError::Evm(EVMError::Internal(
                        "Account doesn't exist".to_owned(),
                    )))
                }
            }
            AMEND_KV_H256 => {
                if self.transact_set_kv_h256(&(data.to_owned().unwrap())) {
                    Ok(None)
                } else {
                    Err(VMError::Evm(EVMError::Internal(
                        "Account doesn't exist".to_owned(),
                    )))
                }
            }
            AMEND_GET_KV_H256 => {
                if let Some(v) = self.transact_get_kv_h256(&(data.to_owned().unwrap())) {
                    Ok(Some(v))
                } else {
                    Err(VMError::Evm(EVMError::Internal(
                        "May be incomplete trie error".to_owned(),
                    )))
                }
            }
            AMEND_ACCOUNT_BALANCE => {
                if self.transact_set_balance(&(data.to_owned().unwrap())) {
                    Ok(None)
                } else {
                    Err(VMError::Evm(EVMError::Internal(
                        "Account doesn't exist or incomplete trie error".to_owned(),
                    )))
                }
            }
            _ => Ok(None),
        }
    }
}

/// Function create creates a new contract.
pub fn create<B: DB + 'static>(
    block_provider: Arc<BlockDataProvider>,
    state_provider: Arc<RefCell<State<B>>>,
    store: Arc<RefCell<VMSubState>>,
    contracts_db: Arc<ContractsDB>,
    request: &InterpreterParams,
    create_kind: CreateKind,
) -> Result<evm::InterpreterResult, VMError> {
    debug!("create request={:?}", request);
    let address = match create_kind {
        CreateKind::FromAddressAndNonce => {
            // Generate new address created from address, nonce
            trace!(
                "Create address from addess {:?} and nonce {:?}",
                &request.sender,
                &request.nonce
            );
            create_address_from_address_and_nonce(&request.sender, &request.nonce)
        }
        CreateKind::FromSaltAndCodeHash => {
            // Generate new address created from sender salt and code hash
            create_address_from_salt_and_code_hash(
                &request.sender,
                request.extra,
                request.input.clone(),
            )
        }
    };
    debug!("create address={:?}", address);
    // Ensure there's no existing contract already at the designated address
    if !can_create(state_provider.clone(), &address)? {
        return Err(VMError::ContractAlreadyExist);
    }
    // Make a checkpoint here
    state_provider.borrow_mut().checkpoint();
    // Create a new contract
    let balance = state_provider.borrow_mut().balance(&address)?;
    state_provider.borrow_mut().new_contract(
        &address,
        balance,
        // The init nonce for a new contract is one, see above documents.
        U256::zero(),
        // The init code should be none. Consider a situation: ContractA will create
        // ContractB with address 0x1ff...fff, but ContractB's init code contains some
        // op like "get code hash from 0x1ff..fff or get code size form 0x1ff...fff",
        // The right result should be "summary(none)" and "0".
        vec![],
    );
    let mut reqchan = request.clone();
    reqchan.address = address;
    reqchan.receiver = address;
    reqchan.is_create = false;
    reqchan.input = vec![];
    reqchan.contract = evm::Contract {
        code_address: address,
        code_data: request.input.clone(),
    };
    let r = call(
        block_provider.clone(),
        state_provider.clone(),
        store.clone(),
        contracts_db.clone(),
        &reqchan,
    );
    match r {
        Ok(evm::InterpreterResult::Normal(output, gas_left, logs)) => {
            // Ensure code size
            if output.len() as u64 > MAX_CREATE_CODE_SIZE {
                state_provider.borrow_mut().revert_checkpoint();
                return Err(VMError::ExccedMaxCodeSize);
            }
            let tx_gas_schedule = TxGasSchedule::default();
            // Pay every byte returnd from CREATE
            let gas_code_deposit: u64 =
                tx_gas_schedule.create_data_gas as u64 * output.len() as u64;
            if gas_left < gas_code_deposit {
                state_provider.borrow_mut().revert_checkpoint();
                return Err(VMError::Evm(evm::Error::OutOfGas));
            }
            let gas_left = gas_left - gas_code_deposit;
            state_provider
                .borrow_mut()
                .set_code(&address, output.clone())?;
            state_provider.borrow_mut().discard_checkpoint();
            let r = Ok(evm::InterpreterResult::Create(
                output, gas_left, logs, address,
            ));
            debug!("create result={:?}", r);
            debug!("create gas_left={:?}", gas_left);
            r
        }
        Ok(evm::InterpreterResult::Revert(output, gas_left)) => {
            state_provider.borrow_mut().revert_checkpoint();
            let r = Ok(evm::InterpreterResult::Revert(output, gas_left));
            debug!("create gas_left={:?}", gas_left);
            debug!("create result={:?}", r);
            r
        }
        Err(e) => {
            debug!("create err={:?}", e);
            state_provider.borrow_mut().revert_checkpoint();
            Err(e)
        }
        _ => unimplemented!(),
    }
}

/// Function call enters into the specific contract.
pub fn call<B: DB + 'static>(
    block_provider: Arc<BlockDataProvider>,
    state_provider: Arc<RefCell<State<B>>>,
    store: Arc<RefCell<VMSubState>>,
    contracts_db: Arc<ContractsDB>,
    request: &InterpreterParams,
) -> Result<evm::InterpreterResult, VMError> {
    // Here not need check twice,becauce prepay is subed ,but need think call_static
    /*if !request.disable_transfer_value && state_provider.borrow_mut().balance(&request.sender)? < request.value {
        return Err(err::Error::NotEnoughBalance);
    }*/
    // Run
    state_provider.borrow_mut().checkpoint();
    let store_son = Arc::new(RefCell::new(store.borrow_mut().clone()));
    let native_factory = NativeFactory::default();
    let rs_contracts_factory = ContractsFactory::new(state_provider.clone(), contracts_db.clone());
    // Check and call Native Contract.
    if let Some(mut native_contract) = native_factory.new_contract(request.contract.code_address) {
        let mut vm_data_provider = DataProvider::new(
            block_provider.clone(),
            state_provider.clone(),
            store.clone(),
            contracts_db.clone(),
        );
        let context = store.borrow().evm_context.clone();
        match native_contract.exec(
            &VmExecParams::from(request.to_owned()),
            &Context::from(context),
            &mut vm_data_provider,
        ) {
            Ok(ret) => {
                // Discard the checkpoint
                state_provider.borrow_mut().discard_checkpoint();
                Ok(ret)
            }
            Err(e) => {
                // If error, revert the checkpoint
                state_provider.borrow_mut().revert_checkpoint();
                Err(e.into())
            }
        }
    } else if rs_contracts_factory.is_rs_contract(&request.contract.code_address) {
        trace!(
            "===> enter rust contracts, address {:?}",
            request.contract.code_address
        );
        let context = store.borrow().evm_context.clone();
        // rust system contracts
        match rs_contracts_factory.works(&request.to_owned(), &Context::from(context)) {
            Ok(ret) => {
                state_provider.borrow_mut().discard_checkpoint();
                trace!(
                    "Contracts factory execute request {:?} successfully",
                    request
                );
                Ok(ret)
            }
            Err(e) => {
                state_provider.borrow_mut().revert_checkpoint();
                trace!("Contracts factory execute request {:?} failed", request);
                Err(e.into())
            }
        }
    } else {
        let r = call_pure(
            block_provider.clone(),
            state_provider.clone(),
            store_son.clone(),
            contracts_db.clone(),
            request,
        );
        debug!("call result={:?}", r);
        match r {
            Ok(evm::InterpreterResult::Normal(output, gas_left, logs)) => {
                state_provider.borrow_mut().discard_checkpoint();
                store.borrow_mut().merge(store_son);
                Ok(evm::InterpreterResult::Normal(output, gas_left, logs))
            }
            Ok(evm::InterpreterResult::Revert(output, gas_left)) => {
                state_provider.borrow_mut().revert_checkpoint();
                Ok(evm::InterpreterResult::Revert(output, gas_left))
            }
            Err(e) => {
                state_provider.borrow_mut().revert_checkpoint();
                Err(e)
            }
            _ => unimplemented!(),
        }
    }
}

pub fn build_evm_context(context: &Context) -> EVMContext {
    EVMContext {
        gas_limit: context.block_quota_limit.as_u64(),
        coinbase: context.coin_base,
        number: U256::from(context.block_number),
        timestamp: context.timestamp,
        difficulty: context.difficulty,
    }
}

/// Function get_refund returns the real ammount to refund for a transaction.
fn get_refund(
    store: Arc<RefCell<VMSubState>>,
    origin: Address,
    gas_limit: u64,
    gas_left: u64,
) -> u64 {
    let refunds_bound = match store.borrow().refund.get(&origin) {
        Some(&data) => data,
        None => 0u64,
    };
    // Get real ammount to refund
    std::cmp::min(refunds_bound, (gas_limit - gas_left) >> 1)
}

/// Liquidtion for a transaction.
fn liquidtion<B: DB + 'static>(
    state_provider: Arc<RefCell<State<B>>>,
    store: Arc<RefCell<VMSubState>>,
    sender: Address,
    gas_price: U256,
    gas_limit: u64,
    gas_left: u64,
    refund: u64,
) -> Result<(), VMError> {
    trace!(
        "gas_price: {:?}, gas limit:{:?}, gas left: {:?}, refund: {:?}",
        gas_price,
        gas_limit,
        gas_left,
        refund
    );
    state_provider
        .borrow_mut()
        .add_balance(&sender, gas_price * (gas_left + refund))?;
    state_provider.borrow_mut().add_balance(
        &store.borrow().evm_context.coinbase,
        gas_price * (gas_limit - gas_left - refund),
    )?;
    Ok(())
}

fn transform_logs(logs: Vec<EVMLog>) -> Vec<Log> {
    logs.into_iter()
        .map(|log| {
            let EVMLog(address, topics, data) = log;

            Log {
                address,
                topics,
                data,
            }
        })
        .collect()
}

fn logs_to_bloom(logs: &[Log]) -> Bloom {
    let mut bloom = Bloom::default();

    logs.iter().for_each(|log| accrue_log(&mut bloom, log));
    bloom
}

fn accrue_log(bloom: &mut Bloom, log: &Log) {
    bloom.accrue(BloomInput::Raw(&log.address.0));
    for topic in &log.topics {
        let input = BloomInput::Hash(&topic.0);
        bloom.accrue(input);
    }
}

/// Returns new address created from address and nonce.
pub fn create_address_from_address_and_nonce(address: &Address, nonce: &U256) -> Address {
    let mut stream = RlpStream::new_list(2);
    stream.append(address);
    stream.append(nonce);
    Address::from(H256::from(summary(stream.as_raw()).as_slice()))
}

/// Returns new address created from sender salt and code hash.
/// See: EIP 1014.
pub fn create_address_from_salt_and_code_hash(
    address: &Address,
    salt: H256,
    code: Vec<u8>,
) -> Address {
    let code_hash = &summary(&code[..])[..];
    let mut buffer = [0u8; 1 + 20 + 32 + 32];
    buffer[0] = 0xff;
    buffer[1..=20].copy_from_slice(&address[..]);
    buffer[(1 + 20)..(1 + 20 + 32)].copy_from_slice(&salt[..]);
    buffer[(1 + 20 + 32)..].copy_from_slice(code_hash);
    Address::from(H256::from(summary(&buffer[..]).as_slice()))
}

/// If a contract creation is attempted, due to either a creation transaction
/// or the CREATE (or future CREATE2) opcode, and the destination address
/// already has either nonzero nonce, or nonempty code, then the creation
/// throws immediately, with exactly the same behavior as would arise if the
/// first byte in the init code were an invalid opcode. This applies
/// retroactively starting from genesis.
///
/// See: EIP 684
pub fn can_create<B: DB + 'static>(
    state_provider: Arc<RefCell<State<B>>>,
    address: &Address,
) -> Result<bool, VMError> {
    let a = state_provider.borrow_mut().nonce(&address)?;
    let b = state_provider.borrow_mut().code(&address)?;
    Ok(a.is_zero() && b.is_empty())
}

#[derive(Clone, Debug)]
pub struct ExecutiveParams {
    /// Address of currently executed code.
    pub code_address: Option<Address>,
    /// Sender of current part of the transaction.
    pub sender: Address,
    /// Receive address. Usually equal to code_address,
    pub to_address: Option<Address>,
    /// Gas paid up front for transaction execution
    pub gas: U256,
    /// Gas price.
    pub gas_price: U256,
    /// Transaction value.
    pub value: U256,
    /// nonce
    pub nonce: U256,
    /// Input data.
    pub data: Option<Bytes>,
}

impl Default for ExecutiveParams {
    /// Returns default ActionParams initialized with zeros
    fn default() -> ExecutiveParams {
        ExecutiveParams {
            code_address: None,
            sender: Address::new(),
            to_address: None,
            gas: U256::zero(),
            gas_price: U256::zero(),
            value: U256::zero(),
            nonce: U256::zero(),
            data: None,
        }
    }
}

pub fn build_vm_exec_params<B: DB + 'static>(
    params: &ExecutiveParams,
    state_provider: Arc<RefCell<State<B>>>,
) -> VmExecParams {
    let mut vm_exec_params = VmExecParams::default();
    vm_exec_params.origin = params.sender;
    vm_exec_params.sender = params.sender;
    if let Some(data) = params.to_address {
        vm_exec_params.to_address = data;
        vm_exec_params.storage_address = data;
        vm_exec_params.code_address = data;
        vm_exec_params.code_data = state_provider.borrow_mut().code(&data).unwrap_or_default();
    }

    vm_exec_params.gas_price = params.gas_price;
    vm_exec_params.gas = params.gas.as_u64();
    vm_exec_params.value = params.value;
    vm_exec_params.data = params.data.clone().unwrap_or_default();
    vm_exec_params.nonce = params.nonce;
    vm_exec_params
}

#[derive(Clone, Debug, Default)]
pub struct VmExecParams {
    pub origin: Address,
    pub storage_address: Address,
    /// Address of currently executed code.
    pub code_address: Address,
    pub code_data: Vec<u8>,
    /// Sender of current part of the transaction.
    pub sender: Address,
    /// Receive address. Usually equal to code_address,
    pub to_address: Address,
    /// Gas paid up front for transaction execution
    pub gas: u64,
    /// Gas price.
    pub gas_price: U256,
    /// Transaction value.
    pub value: U256,
    /// nonce
    pub nonce: U256,
    /// Input data.
    pub data: Bytes,
    pub read_only: bool,
    pub extra: H256,
    pub depth: u64,
    pub disable_transfer_value: bool,
}

impl From<InterpreterParams> for VmExecParams {
    fn from(params: InterpreterParams) -> Self {
        Self {
            origin: params.origin,
            storage_address: params.address,
            code_address: params.contract.code_address,
            code_data: params.contract.code_data,
            sender: params.sender,
            to_address: params.receiver,
            gas: params.gas_limit,
            gas_price: params.gas_price,
            value: params.value,
            nonce: params.nonce,
            data: params.input.clone(),
            read_only: params.read_only,
            extra: params.extra,
            depth: params.depth,
            disable_transfer_value: params.disable_transfer_value,
        }
    }
}

impl Into<InterpreterParams> for VmExecParams {
    fn into(self) -> InterpreterParams {
        InterpreterParams {
            origin: self.origin,
            address: self.storage_address,
            contract: Contract {
                code_address: self.code_address,
                code_data: self.code_data,
            },
            sender: self.sender,
            receiver: self.to_address,
            gas_limit: self.gas,
            gas_price: self.gas_price,
            value: self.value,
            nonce: self.nonce,
            input: self.data.clone(),
            read_only: self.read_only,
            extra: self.extra,
            depth: self.depth,
            is_create: false,
            disable_transfer_value: self.disable_transfer_value,
        }
    }
}

/// A selector for func create_address_from_address_and_nonce() and
/// create_address_from_salt_and_code_hash()
pub enum CreateKind {
    FromAddressAndNonce, // use create_address_from_address_and_nonce
    FromSaltAndCodeHash, // use create_address_from_salt_and_code_hash
}

#[derive(Default, Debug)]
pub struct ExecutedResult {
    pub state_root: H256,
    pub transaction_hash: H256,
    pub quota_used: U256,
    pub quota_left: U256,
    pub logs_bloom: Bloom,
    pub logs: Vec<Log>,
    pub exception: Option<ExecutedException>,
    pub contract_address: Option<Address>,
    pub account_nonce: U256,

    // Note: if the transaction is a cita-evm call, needn't to handle the refund.
    // FIXME: Maybe it is better to handle refund out of evm.
    pub is_evm_call: bool,

    /// Transaction output.
    pub output: Bytes,
}

#[cfg(test)]
mod tests {
    use super::{CitaExecutive, Context, ExecutionError, TxGasSchedule};
    use crate::libexecutor::economical_model::EconomicalModel;
    use crate::libexecutor::{block::EVMBlockDataProvider, sys_config::BlockSysConfig};
    use crate::tests::helpers::*;
    use crate::types::transaction::Action;
    use crate::types::transaction::Transaction;
    use cita_crypto::{CreateKey, KeyPair};
    use cita_types::{Address, H256, U256};
    use cita_vm::state::StateObjectInfo;
    use rustc_hex::FromHex;
    use std::cell::RefCell;
    use std::str::FromStr;
    use std::sync::Arc;

    #[cfg(feature = "sha3hash")]
    pub fn contract_address(address: &Address, nonce: &U256) -> Address {
        use hashable::Hashable;
        use rlp::RlpStream;

        let mut stream = RlpStream::new_list(2);
        stream.append(address);
        stream.append(nonce);
        From::from(stream.out().crypt_hash())
    }

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
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let sender = t.sender();
        let mut state = get_temp_state();
        state
            .add_balance(&sender, U256::from(18 + 100_000))
            .unwrap();

        let mut context = Context::default();
        context.block_quota_limit = U256::from(100_000);

        let block_data_provider = EVMBlockDataProvider::new(context.clone());

        let state = Arc::new(RefCell::new(state));

        let result = {
            CitaExecutive::new(
                Arc::new(block_data_provider),
                state,
                &context,
                EconomicalModel::Charge,
            )
            .exec(&t, &BlockSysConfig::default())
        };

        let expected = ExecutionError::NotEnoughBaseGas;

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), expected);
    }

    #[test]
    #[cfg(feature = "sha3hash")]
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
            version: 2,
        }
        .fake_sign(keypair.address().clone());
        let sender = t.sender();
        let contract = contract_address(t.sender(), &U256::zero());

        let mut state = get_temp_state();

        state
            .add_balance(&sender, U256::from(18 + 100_000))
            .unwrap();

        let mut context = Context::default();
        context.block_quota_limit = U256::from(100_000);

        let block_data_provider = EVMBlockDataProvider::new(context.clone());

        let conf = BlockSysConfig::default();

        let state = Arc::new(RefCell::new(state));

        let executed = {
            CitaExecutive::new(
                Arc::new(block_data_provider),
                state.clone(),
                &context,
                EconomicalModel::Charge,
            )
            .exec(&t, &conf)
            .unwrap()
        };

        let schedule = TxGasSchedule::default();

        // Actually, this is an Action::Create transaction
        assert_eq!(executed.quota_used, U256::from(schedule.tx_create_gas));
        assert_eq!(executed.logs.len(), 0);
        assert_eq!(
            state.borrow_mut().balance(&sender).unwrap(),
            U256::from(18 + 100_000 - 17 - schedule.tx_create_gas)
        );
        assert_eq!(
            state.borrow_mut().balance(&contract).unwrap(),
            U256::from(17)
        );
        assert_eq!(state.borrow_mut().nonce(&sender).unwrap(), U256::from(1));
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
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let mut state = get_temp_state();
        state.add_balance(t.sender(), U256::from(100_042)).unwrap();
        let mut context = Context::default();
        context.block_quota_limit = U256::from(100_000);
        let conf = BlockSysConfig::default();

        let block_data_provider = EVMBlockDataProvider::new(context.clone());
        let state = Arc::new(RefCell::new(state));

        let result = {
            CitaExecutive::new(
                Arc::new(block_data_provider),
                state.clone(),
                &context,
                EconomicalModel::Charge,
            )
            .exec(&t, &conf)
        };

        match result {
            Err(ExecutionError::NotEnoughBalance) => {}
            _ => assert!(false, "Expected not enough cash error. {:?}", result),
        }
    }

    #[test]
    fn test_not_enough_base_gas() {
        let keypair = KeyPair::gen_keypair();
        let t = Transaction {
            action: Action::Create,
            value: U256::from(43),
            data: vec![],
            gas: U256::from(100),
            gas_price: U256::one(),
            nonce: U256::zero().to_string(),
            block_limit: 100u64,
            chain_id: 1.into(),
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let mut state = get_temp_state();
        state.add_balance(t.sender(), U256::from(100_042)).unwrap();
        let mut context = Context::default();
        context.block_quota_limit = U256::from(100);
        let conf = BlockSysConfig::default();

        let block_data_provider = EVMBlockDataProvider::new(context.clone());
        let state = Arc::new(RefCell::new(state));

        let result = {
            CitaExecutive::new(
                Arc::new(block_data_provider),
                state.clone(),
                &context,
                EconomicalModel::Charge,
            )
            .exec(&t, &conf)
        };

        match result {
            Err(ExecutionError::NotEnoughBaseGas) => {}
            _ => assert!(false, "Expected not enough base gas error. {:?}", result),
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
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let state = get_temp_state();
        let mut context = Context::default();
        context.block_quota_limit = U256::from(100_000);
        let conf = BlockSysConfig::default();

        let block_data_provider = EVMBlockDataProvider::new(context.clone());
        let state = Arc::new(RefCell::new(state));

        let result = {
            CitaExecutive::new(
                Arc::new(block_data_provider),
                state.clone(),
                &context,
                EconomicalModel::Quota,
            )
            .exec(&t, &conf)
        };

        // It's ok for not enough cash for quota.
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
        let schedule = TxGasSchedule::default();

        let gas_required = U256::from(schedule.tx_gas + 1000);

        let (deploy_code, _runtime_code) = solc("HelloWorld", source);

        let keypair = KeyPair::gen_keypair();
        let t = Transaction {
            action: Action::Create,
            value: U256::from(0),
            data: deploy_code,
            gas: gas_required,
            gas_price: U256::one(),
            nonce: U256::zero().to_string(),
            block_limit: 100u64,
            chain_id: 1.into(),
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let state = get_temp_state();

        let context = Context::default();

        let conf = BlockSysConfig::default();

        let block_data_provider = EVMBlockDataProvider::new(context.clone());
        let state = Arc::new(RefCell::new(state));

        let res = {
            CitaExecutive::new(
                Arc::new(block_data_provider),
                state.clone(),
                &context,
                EconomicalModel::Quota,
            )
            .exec(&t, &conf)
        };

        assert!(res.is_err());
        let expected = ExecutionError::NotEnoughBaseGas;
        assert_eq!(res.err().unwrap(), expected);
    }

    #[test]
    #[cfg(feature = "sha3hash")]
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
        let schedule = TxGasSchedule::default();

        let gas_required = U256::from(schedule.tx_gas + 100_000);

        let (deploy_code, runtime_code) = solc("AbiTest", source);

        let keypair = KeyPair::gen_keypair();
        let t = Transaction {
            action: Action::Create,
            value: U256::from(0),
            data: deploy_code,
            gas: gas_required,
            gas_price: U256::one(),
            nonce: U256::zero().to_string(),
            block_limit: 100u64,
            chain_id: 1.into(),
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let sender = keypair.address().clone();
        let nonce = U256::zero();
        let contract_address = contract_address(&sender, &nonce);

        let state = get_temp_state();

        let context = Context::default();

        let conf = BlockSysConfig::default();

        let block_data_provider = EVMBlockDataProvider::new(context.clone());
        let state = Arc::new(RefCell::new(state));

        {
            let _ = CitaExecutive::new(
                Arc::new(block_data_provider),
                state.clone(),
                &context,
                EconomicalModel::Quota,
            )
            .exec(&t, &conf);
        }

        assert_eq!(
            &state.borrow_mut().code(&contract_address).unwrap(),
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
        let schedule = TxGasSchedule::default();
        let gas_required = U256::from(schedule.tx_gas + 100_000);
        let contract_addr = Address::from_str("62f4b16d67b112409ab4ac87274926382daacfac").unwrap();
        let (_, runtime_code) = solc("AbiTest", source);

        // big endian: value=0x12345678
        let data = "552410770000000000000000000000000000000000000000000000000000000012345678"
            .from_hex()
            .unwrap();
        let mut state = get_temp_state();
        state
            .set_code(&contract_addr, runtime_code.clone())
            .unwrap();

        let keypair = KeyPair::gen_keypair();
        let t = Transaction {
            action: Action::Call(contract_addr),
            value: U256::from(0),
            data,
            gas: gas_required,
            gas_price: U256::one(),
            nonce: U256::zero().to_string(),
            block_limit: 100u64,
            chain_id: 1.into(),
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let context = Context::default();

        let conf = BlockSysConfig::default();

        let block_data_provider = EVMBlockDataProvider::new(context.clone());
        let state = Arc::new(RefCell::new(state));

        {
            let _ = CitaExecutive::new(
                Arc::new(block_data_provider),
                state.clone(),
                &context,
                EconomicalModel::Quota,
            )
            .exec(&t, &conf);
        }

        // it was supposed that value's address is balance.
        assert_eq!(
            state
                .borrow_mut()
                .get_storage(&contract_addr, &H256::from(&U256::from(0)))
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
        let schedule = TxGasSchedule::default();
        let gas_required = U256::from(schedule.tx_gas + 100_000);
        let contract_addr = Address::from_str("62f4b16d67b112409ab4ac87274926382daacfac").unwrap();
        let (_, runtime_code) = solc("AbiTest", source);
        // big endian: value=0x12345678
        let data = "552410770000000000000000000000000000000000000000000000000000000012345678"
            .from_hex()
            .unwrap();

        let mut state = get_temp_state();
        state
            .set_code(&contract_addr, runtime_code.clone())
            .unwrap();

        let keypair = KeyPair::gen_keypair();
        let t = Transaction {
            action: Action::Call(contract_addr),
            value: U256::from(0),
            data,
            gas: gas_required,
            gas_price: U256::one(),
            nonce: U256::zero().to_string(),
            block_limit: 100u64,
            chain_id: 1.into(),
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let context = Context::default();

        let conf = BlockSysConfig::default();

        let block_data_provider = EVMBlockDataProvider::new(context.clone());
        let state = Arc::new(RefCell::new(state));

        {
            let res = CitaExecutive::new(
                Arc::new(block_data_provider),
                state.clone(),
                &context,
                EconomicalModel::Quota,
            )
            .exec(&t, &conf);
            assert!(res.is_ok());
            match res {
                Ok(result) => println!("quota used: {:?}", result.quota_used),
                Err(e) => println!("e: {:?}", e),
            }
        };

        // it was supposed that value's address is balance.
        assert_eq!(
            state
                .borrow_mut()
                .get_storage(&contract_addr, &H256::from(&U256::from(0)))
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
        let schedule = TxGasSchedule::default();
        let gas_required = U256::from(schedule.tx_gas + 100_000);
        let contract_addr = Address::from_str("62f4b16d67b112409ab4ac87274926382daacfac").unwrap();
        let (_, runtime_code) = solc("AbiTest", source);
        // big endian: value=0x12345678
        let data = "552410770000000000000000000000000000000000000000000000000000000012345678"
            .from_hex()
            .unwrap();

        let mut state = get_temp_state();
        state
            .set_code(&contract_addr, runtime_code.clone())
            .unwrap();

        let keypair = KeyPair::gen_keypair();
        let t = Transaction {
            action: Action::Call(contract_addr),
            value: U256::from(0),
            data,
            gas: gas_required,
            gas_price: U256::one(),
            nonce: U256::zero().to_string(),
            block_limit: 100u64,
            chain_id: 1.into(),
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let context = Context::default();

        let conf = BlockSysConfig::default();

        let block_data_provider = EVMBlockDataProvider::new(context.clone());
        let state = Arc::new(RefCell::new(state));

        {
            let res = CitaExecutive::new(
                Arc::new(block_data_provider),
                state.clone(),
                &context,
                EconomicalModel::Quota,
            )
            .exec(&t, &conf);
            assert!(res.is_ok());
            match res {
                Ok(result) => println!("quota used: {:?}", result.quota_used),
                Err(e) => println!("e: {:?}", e),
            }
        };

        // it was supposed that value's address is balance.
        assert_eq!(
            state
                .borrow_mut()
                .get_storage(&contract_addr, &H256::from(&U256::from(0)))
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
        let schedule = TxGasSchedule::default();
        let gas_required = U256::from(schedule.tx_gas + 100_000);
        let auth_addr = Address::from_str("27ec3678e4d61534ab8a87cf8feb8ac110ddeda5").unwrap();
        let permission_addr =
            Address::from_str("33f4b16d67b112409ab4ac87274926382daacfac").unwrap();

        let mut state = get_temp_state();
        let (_, runtime_code) = solc("FakeAuth", fake_auth);
        state.set_code(&auth_addr, runtime_code.clone()).unwrap();

        let (_, runtime_code) = solc("FakePermissionManagement", fake_permission_manager);
        state
            .set_code(&permission_addr, runtime_code.clone())
            .unwrap();

        // 2b2e05c1: setAuth(address)
        let data = "2b2e05c100000000000000000000000027ec3678e4d61534ab8a87cf8feb8ac110ddeda5"
            .from_hex()
            .unwrap();

        let keypair = KeyPair::gen_keypair();
        let t = Transaction {
            action: Action::Call(permission_addr),
            value: U256::from(0),
            data,
            gas: gas_required,
            gas_price: U256::one(),
            nonce: U256::zero().to_string(),
            block_limit: 100u64,
            chain_id: 1.into(),
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let context = Context::default();

        let conf = BlockSysConfig::default();

        let block_data_provider = EVMBlockDataProvider::new(context.clone());
        let state = Arc::new(RefCell::new(state));

        {
            let res = CitaExecutive::new(
                Arc::new(block_data_provider),
                state.clone(),
                &context,
                EconomicalModel::Quota,
            )
            .exec(&t, &conf);
            assert!(res.is_ok());
            match res {
                Ok(result) => println!("quota used: {:?}", result.quota_used),
                Err(e) => println!("e: {:?}", e),
            }
        };
    }
}
