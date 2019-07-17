use cita_trie::DB;
use cita_types::{Address, H160, H256, U256, U512};
use cita_vm::{
    state::{Error as StateError, State, StateObjectInfo},
    BlockDataProvider, Config, Error as VMError,
};
use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use util::Bytes;

use crate::authentication::{check_permission, AuthenticationError};
use crate::core_types::{receipt::Receipt, TypesError};
use crate::libexecutor::economical_model::EconomicalModel;
use crate::libexecutor::sys_config::BlockSysConfig;
use crate::tx_gas_schedule::TxGasSchedule;
use crate::types::transaction::{Action, SignedTransaction};

///amend the abi data
const AMEND_ABI: u32 = 1;
///amend the account code
const AMEND_CODE: u32 = 2;
///amend the kv of db
const AMEND_KV_H256: u32 = 3;
///amend account's balance
const AMEND_ACCOUNT_BALANCE: u32 = 5;

// FIXME: CITAExecutive need rename to Executive after all works ready.
pub struct CitaExecutive<B> {
    _block_provider: Arc<BlockDataProvider>,
    state_provider: Arc<RefCell<State<B>>>,
    _config: Config,
    economical_model: EconomicalModel,
}

impl<B: DB + 'static> CitaExecutive<B> {
    pub fn new(
        block_provider: Arc<BlockDataProvider>,
        state_provider: State<B>,
        config: Config,
        economical_model: EconomicalModel,
    ) -> Self {
        Self {
            _block_provider: block_provider,
            state_provider: Arc::new(RefCell::new(state_provider)),
            _config: config,
            economical_model,
        }
    }

    pub fn exec(
        &mut self,
        t: &SignedTransaction,
        conf: &BlockSysConfig,
    ) -> Result<Receipt, ExecutionError> {
        let sender = *t.sender();
        let _nonce = self.state_provider.borrow_mut().nonce(&sender)?;
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
        let _result = match t.action {
            Action::Store | Action::AbiStore => {
                // Maybe use tx_gas_schedule.tx_data_non_zero_gas for each byte store, it is more reasonable.
                // But for the data compatible, just let it as tx_gas_schedule.create_data_gas for now.
                let store_gas_used = U256::from(t.data.len() * tx_gas_schedule.create_data_gas);
                if let Some(gas_left) = init_gas.checked_sub(store_gas_used) {
                    Ok(TxExecResult { gas_left })
                } else {
                    return Err(ExecutionError::VM(VMError::NotEnoughBaseGas));
                }
            }
            Action::Create => {
                // Call cita-vm interface.
                //                let account_address = &H160::from(t.sender().into_fixed_bytes());
                //                let nonce = match self.state_provider.borrow_mut().get_state_object(account_address) {
                //                    Ok(opt_account) => {
                //                        if let Some(account) = opt_account {
                //                            account.nonce
                //                        } else {
                //                            U256::zero()
                //                        }
                //                    }
                //                    Err(e) => {
                //                        log::error!(target: "evm executor", "{}", e);
                //                        U256::zero()
                //                    }
                //                };
                //                let evm_transaction = build_evm_transaction(&t, nonce);
                //                if cita_vm::exec(
                //                    self.block_provider,
                //                    self.state_provider,
                //                    evm_context,
                //                    evm_config,
                //                    evm_transaction,
                //                ).is_ok() {
                //
                //                }
                Ok(TxExecResult { gas_left: init_gas })
            }
            Action::AmendData => self.call_amend_data(init_gas, t.value, Some(t.data.clone())),
            Action::Call(ref _address) => unimplemented!(),
        };

        Ok(Receipt::default())
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

    fn call_amend_data(
        &mut self,
        init_gas: U256,
        value: U256,
        data: Option<Bytes>,
    ) -> Result<TxExecResult, ExecutionError> {
        let amend_type = value.low_u32();
        let result = TxExecResult { gas_left: init_gas };
        match amend_type {
            AMEND_ABI => {
                if self.transact_set_abi(&(data.to_owned().unwrap())) {
                    Ok(result)
                } else {
                    Err(ExecutionError::Internal("Account doesn't exist".to_owned()))
                }
            }
            AMEND_CODE => {
                if self.transact_set_code(&(data.to_owned().unwrap())) {
                    Ok(result)
                } else {
                    Err(ExecutionError::Internal("Account doesn't exist".to_owned()))
                }
            }
            AMEND_KV_H256 => {
                if self.transact_set_kv_h256(&(data.to_owned().unwrap())) {
                    Ok(result)
                } else {
                    Err(ExecutionError::Internal("Account doesn't exist".to_owned()))
                }
            }

            AMEND_ACCOUNT_BALANCE => {
                if self.transact_set_balance(&(data.to_owned().unwrap())) {
                    Ok(result)
                } else {
                    Err(ExecutionError::Internal(
                        "Account doesn't exist or incomplete trie error".to_owned(),
                    ))
                }
            }

            _ => Ok(result),
        }
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
pub struct ExecParams {
    /// Address of currently executed code.
    pub code_address: Address,
    /// Sender of current part of the transaction.
    pub sender: Address,
    /// Gas paid up front for transaction execution
    pub gas: U256,
    /// Input data.
    pub data: Option<Bytes>,
}

impl Default for ExecParams {
    /// Returns default ActionParams initialized with zeros
    fn default() -> ExecParams {
        ExecParams {
            code_address: Address::new(),
            sender: Address::new(),
            gas: U256::zero(),
            data: None,
        }
    }
}
