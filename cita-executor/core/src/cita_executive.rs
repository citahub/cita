use cita_trie::DB;
use cita_types::{Address, H160, H256, U256, U512};
use cita_vm::{
    evm::{Context as EVMContext, InterpreterResult, Log as EVMLog},
    state::{Error as StateError, State, StateObjectInfo},
    BlockDataProvider, Config as VMConfig, DataProvider, Error as VMError, Store as VmSubState,
    Transaction as EVMTransaction,
};
use hashable::Hashable;
use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use util::Bytes;

use crate::authentication::{check_permission, AuthenticationError};
use crate::contracts::native::factory::{Factory as NativeFactory, NativeError};
use crate::core_types::{Bloom, BloomInput, Hash, TypesError};
use crate::error::CallError;
use crate::libexecutor::economical_model::EconomicalModel;
use crate::libexecutor::sys_config::BlockSysConfig;
use crate::tx_gas_schedule::TxGasSchedule;
use crate::types::log_entry::LogEntry;
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
    env_info: &'a EnvInfo,
    economical_model: EconomicalModel,
}

impl<'a, B: DB + 'static> CitaExecutive<'a, B> {
    pub fn new(
        block_provider: Arc<BlockDataProvider>,
        state_db: State<B>,
        native_factory: &'a NativeFactory,
        env_info: &'a EnvInfo,
        economical_model: EconomicalModel,
    ) -> Self {
        Self {
            block_provider,
            state_provider: Arc::new(RefCell::new(state_db)),
            native_factory,
            env_info,
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
            return Err(ExecutionError::VM(VMError::NotEnoughBaseGas));
        }

        if t.action == Action::AbiStore && !self.transact_set_abi(&t.data) {
            return Err(ExecutionError::TransactionMalformed(
                "Account doesn't exist".to_owned(),
            ));
        }

        let init_gas = t.gas - U256::from(base_gas_required);

        let result = match t.action {
            Action::Store | Action::AbiStore => {
                // Prepaid t.gas for the transaction.
                self.prepaid(t.sender(), t.gas, t.gas_price, t.value)?;

                // Maybe use tx_gas_schedule.tx_data_non_zero_gas for each byte store, it is more reasonable.
                // But for the data compatible, just let it as tx_gas_schedule.create_data_gas for now.
                let store_gas_used = U256::from(t.data.len() * tx_gas_schedule.create_data_gas);
                if let Some(gas_left) = init_gas.checked_sub(store_gas_used) {
                    let refund_value = gas_left * t.gas_price;
                    // Note: should not be error at refund.
                    self.refund(t.sender(), refund_value)
                        .expect("refund balance to sender must success");

                    let gas_used = t.gas - gas_left;
                    let fee_value = gas_used * t.gas_price;
                    self.handle_tx_fee(&self.env_info.coin_base, fee_value)
                        .expect("Add balance to coin_base must success");

                    let mut result = ExecutedResult::default();
                    result.quota_used = gas_used.as_u64();

                    Ok(result)
                } else {
                    // FIXME: Should not return an error after self.prepaid().
                    // But for compatibility, should keep this. Need to be upgrade in new version.
                    Err(ExecutionError::VM(VMError::NotEnoughBaseGas))
                }
            }
            Action::Create => {
                // Note: Fees has been handle in cita_vm.
                let params = VmExecParams {
                    code_address: None,
                    sender,
                    to_address: None,
                    gas: t.gas,
                    gas_price: t.gas_price(),
                    value: t.value,
                    nonce,
                    data: Some(t.data.clone()),
                };

                self.call_evm(&params)
            }

            Action::AmendData => {
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

                // Prepaid for the transaction
                self.prepaid(t.sender(), t.gas, t.gas_price, t.value)?;

                // Backup used in case of running error
                self.state_provider.borrow_mut().checkpoint();

                // Note: Do not need a checkpoint for amend data.
                match self.call_amend_data(t.value, Some(t.data.clone())) {
                    Ok(_) => {
                        // Refund gas, AmendData do not use any additional gas.
                        let refund_value = init_gas * t.gas_price;
                        self.refund(t.sender(), refund_value)
                            .expect("refund balance to sender must success");

                        let fee_value = U256::from(base_gas_required) * t.gas_price;
                        self.handle_tx_fee(&self.env_info.coin_base, fee_value)
                            .expect("Add balance to coin_base must success");

                        // Discard the checkpoint because of amend data ok.
                        self.state_provider.borrow_mut().discard_checkpoint();
                        Ok(ExecutedResult::default())
                    }
                    Err(e) => {
                        let mut result = ExecutedResult::default();

                        // Need to revert the state.
                        self.state_provider.borrow_mut().revert_checkpoint();
                        result.exception = Some(e);
                        Ok(result)
                    }
                }
            }
            Action::Call(ref address) => {
                let params = VmExecParams {
                    code_address: Some(*address),
                    sender,
                    to_address: Some(*address),
                    gas: t.gas,
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
                return Err(ExecutionError::VM(VMError::NotEnoughBalance));
            }
            self.state_provider
                .borrow_mut()
                .sub_balance(&sender, U256::from(gas_cost))?;
        }
        Ok(())
    }

    fn refund(&mut self, address: &Address, value: U256) -> Result<(), ExecutionError> {
        if self.payment_required() {
            self.state_provider
                .borrow_mut()
                .add_balance(address, value)
                .map_err(|e| ExecutionError::State(e))
        } else {
            Ok(())
        }
    }

    fn handle_tx_fee(
        &mut self,
        coin_base: &Address,
        fee_value: U256,
    ) -> Result<(), ExecutionError> {
        if self.payment_required() {
            self.state_provider
                .borrow_mut()
                .add_balance(coin_base, fee_value)
                .map_err(|e| ExecutionError::State(e))
        } else {
            Ok(())
        }
    }

    fn transfer_balance(
        &mut self,
        from: &Address,
        to: &Address,
        value: U256,
    ) -> Result<(), ExecutionError> {
        if self.payment_required() {
            self.state_provider
                .borrow_mut()
                .transfer_balance(from, to, value)
                .map_err(|e| ExecutionError::State(e))
        } else {
            Ok(())
        }
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

    fn call_evm(&mut self, params: &VmExecParams) -> Result<ExecutedResult, ExecutionError> {
        let evm_transaction = build_evm_transaction(params);
        let evm_config = build_evm_config(self.env_info.gas_limit.clone().as_u64());
        let evm_context = build_evm_context(&self.env_info);
        let result = match cita_vm::exec(
            self.block_provider.clone(),
            self.state_provider.clone(),
            evm_context,
            evm_config,
            evm_transaction,
        ) {
            Ok(evm_result) => build_result_with_ok(params.gas, evm_result),
            Err(e) => build_result_with_err(e),
        };
        Ok(result)
    }

    fn call(&mut self, params: &VmExecParams) -> Result<ExecutedResult, ExecutionError> {
        // Check and call Native Contract.
        if let Some(mut native_contract) = self
            .native_factory
            .new_contract(params.code_address.unwrap())
        {
            self.prepaid(&params.sender, params.gas, params.gas_price, params.value)?;

            // Backup used in case of running out of gas
            self.state_provider.borrow_mut().checkpoint();

            // At first, transfer value to destination.
            if self.payment_required() {
                if self
                    .transfer_balance(&params.sender, &params.to_address.unwrap(), params.value)
                    .is_err()
                {
                    // Discard the checkpoint
                    self.state_provider.borrow_mut().revert_checkpoint();
                    return Err(ExecutionError::Internal(
                        "Transfer balance failed while calling native contract.".to_string(),
                    ));
                }
            }

            let store = VmSubState::default();

            let store = Arc::new(RefCell::new(store));

            let mut vm_data_provider = DataProvider::new(
                self.block_provider.clone(),
                self.state_provider.clone(),
                store,
            );
            let result = match native_contract.exec(params, &self.env_info, &mut vm_data_provider) {
                Ok(ret) => {
                    // Discard the checkpoint
                    self.state_provider.borrow_mut().discard_checkpoint();

                    // FIXME: Fix refund later.
                    //                    let refund_value = get_quota_left(ret) * params.gas_price;
                    //                    self.refund(&params.sender, refund_value).expect("refund balance to sender must success");

                    let result = build_result_with_ok(params.gas, ret);
                    let fee_value = U256::from(result.quota_used) * params.gas_price;
                    self.handle_tx_fee(&self.env_info.coin_base, fee_value)
                        .expect("Add balance to coin_base must success");
                    result
                }
                Err(e) => {
                    // If error, revert the checkpoint
                    self.state_provider.borrow_mut().revert_checkpoint();

                    let mut result = ExecutedResult::default();
                    result.exception = Some(ExecutionError::NativeContract(e));
                    result
                }
            };
            Ok(result)
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
        coinbase: env_info.coin_base,
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

fn build_result_with_ok(init_gas: U256, ret: InterpreterResult) -> ExecutedResult {
    let mut result = ExecutedResult::default();

    match ret {
        InterpreterResult::Normal(data, quota_left, logs) => {
            result.quota_used = init_gas.as_u64() - quota_left;
            result.logs = transform_logs(logs);
            result.logs_bloom = logs_to_bloom(&result.logs);
            result.output = data;
        }
        InterpreterResult::Revert(_data, quota_left) => {
            result.quota_used = init_gas.as_u64() - quota_left;
        }
        InterpreterResult::Create(_data, quota_left, logs, contract_address) => {
            result.quota_used = init_gas.as_u64() - quota_left;
            result.logs = transform_logs(logs);
            result.logs_bloom = logs_to_bloom(&result.logs);

            let address_slice: &[u8] = contract_address.as_ref();
            result.contract_address = Some(H160::from(address_slice));
        }
    };
    result
}

fn build_result_with_err(err: VMError) -> ExecutedResult {
    let mut result = ExecutedResult::default();
    result.exception = Some(ExecutionError::VM(err));
    result
}

fn transform_logs(logs: Vec<EVMLog>) -> Vec<LogEntry> {
    logs.into_iter()
        .map(|log| {
            let EVMLog(address, topics, data) = log;

            LogEntry {
                address,
                topics,
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
        let input = BloomInput::Hash(&topic.0);
        bloom.accrue(input);
    }
}

/// Returns new address created from address and given nonce.
pub fn contract_address(address: &Address, nonce: &U256) -> Address {
    use rlp::RlpStream;

    let mut stream = RlpStream::new_list(2);
    stream.append(address);
    stream.append(nonce);
    From::from(stream.out().crypt_hash())
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

impl Into<CallError> for ExecutionError {
    fn into(self) -> CallError {
        CallError::Exceptional
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
#[derive(Debug, Clone)]
pub struct EnvInfo {
    /// The block number.
    pub number: BlockNumber,
    /// The fee refund address.
    pub coin_base: Address,
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
            coin_base: Address::default(),
            timestamp: 0,
            difficulty: 0.into(),
            gas_limit: U256::from(u64::max_value()),
            last_hashes: Arc::new(vec![]),
            gas_used: 0.into(),
            account_gas_limit: 0.into(),
        }
    }
}

#[derive(Default, Debug)]
pub struct ExecutedResult {
    pub state_root: Hash,
    pub transaction_hash: Hash,
    pub quota_used: u64,
    pub logs_bloom: Bloom,
    pub logs: Vec<LogEntry>,
    pub exception: Option<ExecutionError>,
    pub contract_address: Option<Address>,
    pub account_nonce: U256,

    /// Transaction output.
    pub output: Bytes,
}
