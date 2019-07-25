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

// use crate::contracts::native::factory::Contract as NativeContract;
use crate::contracts::native::factory::Factory as NativeFactory;
use crate::engines::Engine;
use crate::error::ExecutionError;
pub use crate::executed::{Executed, ExecutionResult};
use crate::externalities::*;
use crate::libexecutor::economical_model::EconomicalModel;
use crate::libexecutor::sys_config::BlockSysConfig;
use crate::state::backend::Backend as StateBackend;
use crate::state::{State, Substate};
use crate::types::transaction::SignedTransaction;
use cita_types::{Address, U256};
use evm::action_params::ActionParams;
use evm::env_info::EnvInfo;
use evm::{self, Factory, FinalizationResult};
use hashable::Hashable;
use util::*;

/// Roughly estimate what stack size each level of evm depth will use
/// TODO [todr] We probably need some more sophisticated calculations here
///      (limit on my machine 132)
/// Maybe something like here:
/// `https://github.com/ethereum/libethereum/blob/4db169b8504f2b87f7d5a481819cfb959fc65f6c/libethereum/ExtVM.cpp`

thread_local! {
    /// Stack size
    /// Should be modified if it is changed in Rust since it is no way
    /// to know or get it
    pub static LOCAL_STACK_SIZE: ::std::cell::Cell<usize> = ::std::cell::Cell::new(
        ::std::env::var("RUST_MIN_STACK").ok().and_then(
            |s| s.parse().ok()).unwrap_or(2 * 1024 * 1024));
}

/// Returns new address created from address and given nonce.
pub fn contract_address(address: &Address, nonce: &U256) -> Address {
    use rlp::RlpStream;

    let mut stream = RlpStream::new_list(2);
    stream.append(address);
    stream.append(nonce);
    From::from(stream.out().crypt_hash())
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
    chain_version: u32,
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
        chain_version: u32,
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
            chain_version,
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
        chain_version: u32,
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
            chain_version,
        }
    }

    /// Creates `Externalities` from `Executive`.
    #[allow(unknown_lints, clippy::too_many_arguments)] // TODO clippy
    pub fn as_externalities<'any>(
        &'any mut self,
        origin_info: OriginInfo,
        substate: &'any mut Substate,
        output: OutputPolicy<'any, 'any>,
        static_call: bool,
        economical_model: EconomicalModel,
    ) -> Externalities<'any, B>
where {
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
            is_static,
            economical_model,
            self.chain_version,
        )
    }

    /// This function should be used to execute transaction.
    pub fn transact(
        &'a mut self,
        _t: &SignedTransaction,
        _conf: &BlockSysConfig,
    ) -> Result<Executed, ExecutionError> {
        unimplemented!()
    }

    pub fn transact_with_tracer<T, V>(
        &'a mut self,
        _t: &SignedTransaction,
        _conf: &BlockSysConfig,
    ) -> Result<Executed, ExecutionError>
where {
        unimplemented!()
    }

    /// Calls contract function with given contract params.
    /// NOTE. It does not finalize the transaction (doesn't do refunds, nor suicides).
    /// Modifies the substate and the output.
    /// Returns either gas_left or `evm::Error`.
    pub fn call(
        &mut self,
        _params: &ActionParams,
        _substate: &mut Substate,
        _output: BytesRef,
    ) -> evm::Result<FinalizationResult> {
        unimplemented!()
    }

    /// Creates contract with given contract params.
    /// NOTE. It does not finalize the transaction (doesn't do refunds, nor suicides).
    /// Modifies the substate.
    pub fn create(
        &mut self,
        _params: &ActionParams,
        _substate: &mut Substate,
    ) -> evm::Result<FinalizationResult> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    extern crate cita_logger as logger;
    extern crate rustc_hex;
    ////////////////////////////////////////////////////////////////////////////////

    use self::rustc_hex::FromHex;
    use super::*;
    use crate::engines::NullEngine;
    use crate::libexecutor::sys_config::BlockSysConfig;
    use crate::state::Substate;
    use crate::tests::helpers::*;
    use crate::types::transaction::Transaction;
    use cita_crypto::{CreateKey, KeyPair};
    use cita_types::{Address, H256, U256};
    use evm::action_params::{ActionParams, ActionValue};
    use evm::env_info::EnvInfo;
    use evm::Schedule;
    use evm::{Factory, VMType};
    use std::ops::Deref;
    use std::str::FromStr;
    use std::sync::Arc;

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
                0,
            );
            ex.transact(&t, opts, &BlockSysConfig::default())
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
            version: 2,
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
        let conf = BlockSysConfig::default();

        let executed = {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Charge,
                conf.chain_version,
            );
            ex.transact(&t, &conf).unwrap()
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
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let native_factory = NativeFactory::default();
        let engine = NullEngine::default();
        let mut state = get_temp_state();
        state.add_balance(t.sender(), &U256::from(100_042)).unwrap();
        let mut info = EnvInfo::default();
        info.gas_limit = U256::from(100_000);
        let conf = BlockSysConfig::default();

        let result = {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Charge,
                conf.chain_version,
            );
            ex.transact(&t, &conf)
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
            version: 2,
        }
        .fake_sign(keypair.address().clone());

        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let native_factory = NativeFactory::default();
        let engine = NullEngine::default();
        let mut state = get_temp_state();
        let mut info = EnvInfo::default();
        info.gas_limit = U256::from(100_000);
        let conf = BlockSysConfig::default();

        let result = {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
                conf.chain_version,
            );
            ex.transact(&t, &conf)
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
        let conf = BlockSysConfig::default();

        let mut ex = Executive::new(
            &mut state,
            &info,
            &engine,
            &factory,
            &native_factory,
            false,
            EconomicalModel::Quota,
            conf.chain_version,
        );
        let res = ex.create(&params, &mut substate);
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
        let conf = BlockSysConfig::default();

        {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
                conf.chain_version,
            );
            let _ = ex.create(&params, &mut substate);
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
        let conf = BlockSysConfig::default();

        {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
                conf.chain_version,
            );
            let mut out = vec![];
            let _ = ex.call(&params, &mut substate, BytesRef::Fixed(&mut out));
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
        let conf = BlockSysConfig::default();

        {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
                conf.chain_version,
            );
            let mut out = vec![];
            let res = ex.call(&params, &mut substate, BytesRef::Fixed(&mut out));
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
        let conf = BlockSysConfig::default();

        {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
                conf.chain_version,
            );
            let mut out = vec![];
            let res = ex.call(&params, &mut substate, BytesRef::Fixed(&mut out));
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
        let conf = BlockSysConfig::default();

        {
            let mut ex = Executive::new(
                &mut state,
                &info,
                &engine,
                &factory,
                &native_factory,
                false,
                EconomicalModel::Quota,
                conf.chain_version,
            );
            let mut out = vec![];
            let res = ex.call(&params, &mut substate, BytesRef::Fixed(&mut out));

            assert!(res.is_ok());
            match res {
                Ok(gas_used) => println!("gas used: {:?}", gas_used),
                Err(e) => println!("e: {:?}", e),
            }
        };
    }
}
