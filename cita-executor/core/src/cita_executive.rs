use cita_trie::DB;
use cita_types::{Address, H160, H256, U256, U512};
use cita_vm::{
    evm::{Context as EVMContext, InterpreterResult, Log as EVMLog},
    state::{Error as StateError, State, StateObjectInfo},
    BlockDataProvider, Config as VMConfig, Error as VMError, Transaction as EVMTransaction,
};
use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use util::Bytes;

use crate::authentication::{check_permission, AuthenticationError};
use crate::contracts::native::factory::{Factory as NativeFactory, NativeError};
use crate::core_types::{Bloom, BloomInput, Hash, LogEntry, Receipt, TypesError};
use crate::libexecutor::economical_model::EconomicalModel;
use crate::libexecutor::sys_config::BlockSysConfig;
use crate::tx_gas_schedule::TxGasSchedule;
use crate::types::transaction::{Action, SignedTransaction};
use crate::types::BlockNumber;

/// Simple vector of hashes, should be at most 256 items large, can be smaller if being used
/// for a block whose number is less than 257.
pub type LastHashes = Vec<H256>;

///amend the abi data
const AMEND_ABI: u32 = 1;
///amend the account code
const AMEND_CODE: u32 = 2;
///amend the kv of db
const AMEND_KV_H256: u32 = 3;
///amend account's balance
const AMEND_ACCOUNT_BALANCE: u32 = 5;

// FIXME: CITAExecutive need rename to Executive after all works ready.
pub struct CitaExecutive<'a, B> {
    block_provider: Arc<BlockDataProvider>,
    state_provider: Arc<RefCell<State<B>>>,
    native_factory: &'a NativeFactory,
    env_info: EnvInfo,
    economical_model: EconomicalModel,
}

impl<'a, B: DB + 'static> CitaExecutive<'a, B> {
    pub fn new(
        block_provider: Arc<BlockDataProvider>,
        state_provider: State<B>,
        native_factory: &'a NativeFactory,
        env_info: EnvInfo,
        economical_model: EconomicalModel,
    ) -> Self {
        Self {
            block_provider,
            state_provider: Arc::new(RefCell::new(state_provider)),
            native_factory,
            env_info,
            economical_model,
        }
    }

    pub fn exec(
        &mut self,
        t: &SignedTransaction,
        conf: &BlockSysConfig,
    ) -> Result<Receipt, ExecutionError> {
        let sender = *t.sender();
        let nonce = self.state_provider.borrow_mut().nonce(&sender)?;
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
        };

        if sender != Address::zero() && t.gas < U256::from(base_gas_required) {
            // FIXME: It is better to change NotEnoughBaseGas to
            //    NotEnoughBaseGas {
            //        required: U256,
            //        got: U256,
            //    }
            // Need to change VMError defined in cita-vm.
            return Err(ExecutionError::VM(VMError::NotEnoughBaseGas));
        }

        if t.action == Action::AmendData {
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
        }

        if t.action == Action::AbiStore && !self.transact_set_abi(&t.data) {
            return Err(ExecutionError::TransactionMalformed(
                "Account doesn't exist".to_owned(),
            ));
        }

        let balance = self.state_provider.borrow_mut().balance(&sender)?;
        let gas_cost = t.gas.full_mul(t.gas_price());
        let total_cost = U512::from(t.value) + gas_cost;

        // Avoid unaffordable transactions
        if self.payment_required() {
            let balance512 = U512::from(balance);
            if balance512 < total_cost {
                return Err(ExecutionError::VM(VMError::NotEnoughBalance));
            }
            self.state_provider
                .borrow_mut()
                .sub_balance(&sender, U256::from(gas_cost))?;
        }

        let init_gas = t.gas - U256::from(base_gas_required);
        let result = match t.action {
            Action::Store | Action::AbiStore => {
                // Maybe use tx_gas_schedule.tx_data_non_zero_gas for each byte store, it is more reasonable.
                // But for the data compatible, just let it as tx_gas_schedule.create_data_gas for now.
                let store_gas_used = U256::from(t.data.len() * tx_gas_schedule.create_data_gas);
                if init_gas.checked_sub(store_gas_used).is_some() {
                    let mut receipt = Receipt::default();
                    receipt.quota_used = store_gas_used.as_u64();
                    Ok(receipt)
                } else {
                    Err(ExecutionError::VM(VMError::NotEnoughBaseGas))
                }
            }
            Action::Create => {
                let params = VmExecParams {
                    code_address: None,
                    sender,
                    to_address: None,
                    gas: init_gas,
                    gas_price: t.gas_price(),
                    value: t.value,
                    nonce,
                    data: Some(t.data.clone()),
                };

                self.call_evm(&params)
            }

            Action::AmendData => {
                // Note: Do not need a checkpoint for amend data.
                match self.call_amend_data(t.value, Some(t.data.clone())) {
                    Ok(_) => Ok(Receipt::default()),
                    Err(e) => Err(e),
                }
            }
            Action::Call(ref address) => {
                let params = VmExecParams {
                    code_address: Some(*address),
                    sender,
                    to_address: Some(*address),
                    gas: init_gas,
                    gas_price: t.gas_price(),
                    value: t.value,
                    nonce,
                    data: Some(t.data.clone()),
                };
                self.call(&params)
            }
        };

        result
    }

    fn payment_required(&self) -> bool {
        self.economical_model == EconomicalModel::Charge
    }

    fn transact_set_abi(&mut self, data: &[u8]) -> bool {
        if data.len() <= 20 {
            return false;
        }

        let account = H160::from(&data[0..20]);
        let abi = &data[20..];

        info!("Set abi for contract address: {:?}", account);

        self.state_provider
            .borrow_mut()
            .exist(&account)
            .map(|exists| {
                exists
                    && self
                        .state_provider
                        .borrow_mut()
                        .set_abi(&account, abi.to_vec())
                        .is_ok()
            })
            .unwrap_or(false)
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
        self.state_provider
            .borrow_mut()
            .balance(&account)
            .and_then(|now_val| {
                if now_val >= balance {
                    self.state_provider
                        .borrow_mut()
                        .sub_balance(&account, now_val - balance)
                } else {
                    self.state_provider
                        .borrow_mut()
                        .add_balance(&account, balance - now_val)
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

    fn call_amend_data(&mut self, value: U256, data: Option<Bytes>) -> Result<(), ExecutionError> {
        let amend_type = value.low_u32();
        match amend_type {
            AMEND_ABI => {
                if self.transact_set_abi(&(data.to_owned().unwrap())) {
                    Ok(())
                } else {
                    Err(ExecutionError::Internal("Account doesn't exist".to_owned()))
                }
            }
            AMEND_CODE => {
                if self.transact_set_code(&(data.to_owned().unwrap())) {
                    Ok(())
                } else {
                    Err(ExecutionError::Internal("Account doesn't exist".to_owned()))
                }
            }
            AMEND_KV_H256 => {
                if self.transact_set_kv_h256(&(data.to_owned().unwrap())) {
                    Ok(())
                } else {
                    Err(ExecutionError::Internal("Account doesn't exist".to_owned()))
                }
            }

            AMEND_ACCOUNT_BALANCE => {
                if self.transact_set_balance(&(data.to_owned().unwrap())) {
                    Ok(())
                } else {
                    Err(ExecutionError::Internal(
                        "Account doesn't exist or incomplete trie error".to_owned(),
                    ))
                }
            }

            _ => Ok(()),
        }
    }

    fn call_evm(&mut self, params: &VmExecParams) -> Result<Receipt, ExecutionError> {
        let evm_transaction = build_evm_transaction(params);
        let evm_config = build_evm_config(self.env_info.gas_limit.clone().as_u64());
        let evm_context = build_evm_context(&self.env_info);
        let receipt = match cita_vm::exec(
            self.block_provider.clone(),
            self.state_provider.clone(),
            evm_context,
            evm_config,
            evm_transaction,
        ) {
            Ok(evm_result) => build_receipt_with_ok(params.gas, evm_result),
            Err(e) => build_receipt_with_err(e),
        };
        Ok(receipt)
    }

    fn call(&mut self, params: &VmExecParams) -> Result<Receipt, ExecutionError> {
        // Backup used in case of running out of gas
        self.state_provider.borrow_mut().checkpoint();

        // At first, transfer value to destination.
        // TODO: Keep it for compatibility. Remove it later.
        if self.payment_required() {
            self.state_provider.borrow_mut().transfer_balance(
                &params.sender,
                &params.to_address.unwrap(),
                params.value,
            )?
        }

        // Check and call Native Contract.
        if let Some(_native_contract) = self
            .native_factory
            .new_contract(params.code_address.unwrap())
        {
            unimplemented!()
        } else {
            // Call EVM contract
            self.call_evm(params)
        }
    }
}

fn build_evm_transaction(params: &VmExecParams) -> EVMTransaction {
    EVMTransaction {
        from: params.sender,
        value: params.value,
        gas_limit: params.gas.as_u64(),
        gas_price: params.gas_price,
        input: params.data.clone().unwrap_or_default(),
        to: params.to_address,
        nonce: params.nonce,
    }
}

fn build_evm_context(env_info: &EnvInfo) -> EVMContext {
    EVMContext {
        gas_limit: env_info.gas_limit.as_u64(),
        coinbase: env_info.author,
        number: U256::from(env_info.number),
        timestamp: env_info.timestamp,
        difficulty: env_info.difficulty,
    }
}

fn build_evm_config(block_gas_limit: u64) -> VMConfig {
    VMConfig {
        // block_gas_limit is meaningless in cita_vm, so let it as default_block_quota_limit.
        block_gas_limit,
        ..Default::default()
    }
}

fn build_receipt_with_ok(init_gas: U256, result: InterpreterResult) -> Receipt {
    let mut receipt = Receipt::default();

    match result {
        InterpreterResult::Normal(_data, quota_left, logs) => {
            receipt.quota_used = init_gas.as_u64() - quota_left;
            receipt.logs = transform_logs(logs);
            receipt.logs_bloom = logs_to_bloom(&receipt.logs);
        }
        InterpreterResult::Revert(_data, quota_left) => {
            receipt.quota_used = init_gas.as_u64() - quota_left;
        }
        InterpreterResult::Create(_data, quota_left, logs, contract_address) => {
            receipt.quota_used = init_gas.as_u64() - quota_left;
            receipt.logs = transform_logs(logs);
            receipt.logs_bloom = logs_to_bloom(&receipt.logs);

            let address_slice: &[u8] = contract_address.as_ref();
            receipt.contract_address = Some(H160::from(address_slice));
        }
    };
    receipt
}

fn build_receipt_with_err(err: VMError) -> Receipt {
    let mut receipt = Receipt::default();
    receipt.receipt_error = err.to_string();
    receipt
}

fn transform_logs(logs: Vec<EVMLog>) -> Vec<LogEntry> {
    logs.into_iter()
        .map(|log| {
            let EVMLog(address, topics, data) = log;

            LogEntry {
                address,
                topics: topics
                    .into_iter()
                    .map(|topic| Hash::from_bytes(topic.as_ref()).expect("never returns an error"))
                    .collect(),
                data,
            }
        })
        .collect()
}

fn logs_to_bloom(logs: &[LogEntry]) -> Bloom {
    let mut bloom = Bloom::default();

    logs.iter().for_each(|log| accrue_log(&mut bloom, log));
    bloom
}

fn accrue_log(bloom: &mut Bloom, log: &LogEntry) {
    bloom.accrue(BloomInput::Raw(&log.address.0));
    for topic in &log.topics {
        let input = BloomInput::Hash(topic.as_fixed_bytes());
        bloom.accrue(input);
    }
}

#[derive(Debug)]
pub enum ExecutionError {
    VM(VMError),
    State(StateError),
    NotFound,
    Types(TypesError),
    Internal(String),
    TransactionMalformed(String),
    Authentication(AuthenticationError),
    NativeContract(NativeError),
}

impl Error for ExecutionError {}
impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ExecutionError::VM(ref err) => format!("vm error: {:?}", err),
            ExecutionError::State(ref err) => format!("state error: {:?}", err),
            ExecutionError::NotFound => "not found".to_owned(),
            ExecutionError::Types(ref err) => format!("types error: {:?}", err),
            ExecutionError::Internal(ref err) => format!("internal error: {:?}", err),
            ExecutionError::TransactionMalformed(ref err) => format!("internal error: {:?}", err),
            ExecutionError::Authentication(ref err) => format!("internal error: {:?}", err),
            ExecutionError::NativeContract(ref err) => format!("internal error: {:?}", err),
        };
        write!(f, "{}", printable)
    }
}

impl From<VMError> for ExecutionError {
    fn from(err: VMError) -> Self {
        ExecutionError::VM(err)
    }
}

impl From<StateError> for ExecutionError {
    fn from(err: StateError) -> Self {
        ExecutionError::State(err)
    }
}

impl From<TypesError> for ExecutionError {
    fn from(err: TypesError) -> Self {
        ExecutionError::Types(err)
    }
}

impl From<NativeError> for ExecutionError {
    fn from(err: NativeError) -> Self {
        ExecutionError::NativeContract(err)
    }
}

impl From<AuthenticationError> for ExecutionError {
    fn from(err: AuthenticationError) -> Self {
        match err {
            AuthenticationError::TransactionMalformed(err_str) => {
                ExecutionError::TransactionMalformed(err_str)
            }
            _ => ExecutionError::Authentication(err),
        }
    }
}

/// Transaction execute result.
#[derive(Debug)]
pub struct TxExecResult {
    /// Final amount of gas left.
    pub gas_left: U256,
}

#[derive(Clone, Debug)]
pub struct VmExecParams {
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

impl Default for VmExecParams {
    /// Returns default ActionParams initialized with zeros
    fn default() -> VmExecParams {
        VmExecParams {
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

/// Information concerning the execution environment for vm.
#[derive(Debug)]
pub struct EnvInfo {
    /// The block number.
    pub number: BlockNumber,
    /// The block author.
    pub author: Address,
    /// The block timestamp.
    pub timestamp: u64,
    /// The block difficulty.
    pub difficulty: U256,
    /// The block gas limit.
    pub gas_limit: U256,
    /// The last 256 block hashes.
    pub last_hashes: Arc<LastHashes>,
    /// The gas used.
    pub gas_used: U256,
    pub account_gas_limit: U256,
}

impl Default for EnvInfo {
    fn default() -> Self {
        EnvInfo {
            number: 0,
            author: Address::default(),
            timestamp: 0,
            difficulty: 0.into(),
            gas_limit: U256::from(u64::max_value()),
            last_hashes: Arc::new(vec![]),
            gas_used: 0.into(),
            account_gas_limit: 0.into(),
        }
    }
}
