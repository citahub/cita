use cita_trie::DB;
use cita_vm::{
    state::{Error as StateError, State, StateObjectInfo},
    BlockDataProvider, Config, Error as VMError,
};
use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::authentication::{check_permission, AuthenticationError};
use crate::core_types::{receipt::Receipt, TypesError};
use crate::libexecutor::sys_config::BlockSysConfig;
use crate::types::transaction::SignedTransaction;

// FIXME: CITAExecutive need rename to Executive after all works ready.
pub struct CitaExecutive<B> {
    pub block_provider: Arc<BlockDataProvider>,
    pub state_provider: Arc<RefCell<State<B>>>,
    pub config: Config,
}

impl<B: DB + 'static> CitaExecutive<B> {
    pub fn new(
        block_provider: Arc<BlockDataProvider>,
        state_provider: State<B>,
        config: Config,
    ) -> Self {
        Self {
            block_provider,
            state_provider: Arc::new(RefCell::new(state_provider)),
            config,
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

        Ok(Receipt::default())
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
