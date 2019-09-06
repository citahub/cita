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
// limitations under the License

use crate::helper::{secret_2_address, string_2_bytes, string_2_h256, string_2_u256};
use crate::json::state::Test;

use core_executor::cita_executive::CitaExecutive;
use core_executor::contracts::native::factory::Factory as NativeFactory;
use core_executor::libexecutor::sys_config::BlockSysConfig;
use core_executor::libexecutor::{block::EVMBlockDataProvider, economical_model::EconomicalModel};
use core_executor::types::{context::Context, transaction::Transaction}; //,Action,SignedTransaction};

use core_executor::types::errors::ExecutionError;

use cita_types::U256;
use cita_vm::state::{State, StateObjectInfo};
use core_executor::tx_gas_schedule::TxGasSchedule;
use libproto::blockchain::Transaction as ProtoTransaction;

use std::fs;
use std::sync::Arc;

pub fn test_json_file(p: &str) {
    let f = fs::File::open(p).unwrap();
    let t = Test::load(f).unwrap();

    for (name, data) in t.into_iter() {
        let data_post_homestead = data.post.unwrap().homestead;
        if data_post_homestead.is_none() {
            continue;
        }

        for (i, postdata) in data_post_homestead.unwrap().iter().enumerate() {
            println!("{}::{}::{}\n", p, name, i);
            let d = Arc::new(cita_trie::MemoryDB::new(false));
            let mut state_provider = State::new(d).unwrap();

            for (address, account) in data.pre.clone().unwrap() {
                let balance = string_2_u256(account.balance);
                let code = string_2_bytes(account.code);
                let nonce = string_2_u256(account.nonce);
                if code.is_empty() {
                    state_provider.new_contract(&address, balance, nonce, vec![]);
                } else {
                    state_provider.new_contract(&address, balance, nonce, code);
                }
                for (k, v) in account.storage {
                    let kk = string_2_h256(k);
                    let vv = string_2_h256(v);
                    state_provider.set_storage(&address, kk, vv).unwrap();
                }
            }
            state_provider.commit().unwrap();

            let state_provider = Arc::new(std::cell::RefCell::new(state_provider));

            let idx_gas = &postdata.indexes[&String::from("gas")];
            let idx_value = &postdata.indexes[&String::from("value")];
            let idx_data = &postdata.indexes[&String::from("data")];

            let str_block_gas = data.env.current_gas_limit.clone();
            let str_prev_hash = data.env.previous_hash.clone();
            let str_gas = data.transaction.gas_limit.clone()[*idx_gas].clone();
            let str_value = data.transaction.value.clone()[*idx_value].clone();
            let str_data = data.transaction.data.clone()[*idx_data].clone();

            let mut evm_context = Context::default();
            evm_context.block_quota_limit = string_2_u256(str_block_gas.clone());
            evm_context.coin_base = data.env.current_coinbase;
            evm_context.block_number = string_2_u256(data.env.current_number.clone()).low_u64();
            evm_context.timestamp = string_2_u256(data.env.current_timestamp.clone()).low_u64();
            evm_context.difficulty = string_2_u256(data.env.current_difficulty.clone());
            evm_context.quota_used = U256::zero();
            evm_context.last_hashes = Arc::new(vec![string_2_h256(str_prev_hash)]);

            let block_data_provider = EVMBlockDataProvider::new(evm_context.clone());
            let native_factory = NativeFactory::default();
            let mut exepinst = CitaExecutive::new(
                Arc::new(block_data_provider),
                state_provider.clone(),
                &native_factory,
                &evm_context,
                EconomicalModel::Charge,
            );

            let mut proto_tx = ProtoTransaction::new();
            proto_tx.set_data(string_2_bytes(str_data));
            proto_tx.set_value(string_2_bytes(str_value));
            proto_tx.set_nonce(data.transaction.nonce.clone());
            proto_tx.set_quota(string_2_u256(str_gas).low_u64());
            if !data.transaction.to.is_empty() {
                proto_tx.set_to(data.transaction.to.clone());
            }

            let mut config = BlockSysConfig::default();
            config.quota_price = string_2_u256(data.transaction.gas_price.clone());
            config.economical_model = EconomicalModel::Charge;
            config.chain_version = 2;
            config.chain_owner = data.env.current_coinbase;
            config.check_options.fee_back_platform = true;

            let tx = Transaction::create(&proto_tx).unwrap();
            let sender = secret_2_address(&data.transaction.secret_key);
            let mut signed_transaction = tx.clone().fake_sign(sender);
            signed_transaction.gas_price = config.quota_price;

            let exec_result = exepinst.exec(&signed_transaction, &config);
            match exec_result {
                Ok(_) => {}
                Err(err) => {
                    let schedule = TxGasSchedule::default();
                    // Bellow has a error, need gas*price before compare with balance
                    let tx_quota_used = match err {
                        ExecutionError::Internal(_) => tx.gas,
                        _ => std::cmp::min(
                            state_provider
                                .borrow_mut()
                                .balance(&sender)
                                .unwrap_or_else(|_| U256::from(0)),
                            U256::from(schedule.tx_gas),
                        ),
                    };

                    let sender_balance = state_provider.borrow_mut().balance(&sender).unwrap();
                    let tx_fee = tx_quota_used * config.quota_price;
                    let real_fee = std::cmp::min(sender_balance, tx_fee);

                    if state_provider
                        .borrow_mut()
                        .sub_balance(&sender, real_fee)
                        .is_ok()
                    {
                        let _ = state_provider
                            .borrow_mut()
                            .add_balance(&config.chain_owner, real_fee);
                    }
                }
            }
            state_provider.borrow_mut().commit().unwrap();
            assert_eq!(
                state_provider.borrow().root,
                string_2_h256(postdata.hash.clone())
            );
        }
    }
}

pub fn test_json_path(p: &str) {
    let info = fs::metadata(p).unwrap();
    if info.is_dir() {
        for entry in fs::read_dir(p).unwrap() {
            let entry = entry.unwrap();
            let p = entry.path();
            test_json_path(p.to_str().unwrap());
        }
    } else {
        test_json_file(p);
    }
}

pub fn skip_json_path(_reason: &str, _name: &str) {}

#[cfg(test)]
mod tests {

    #[cfg(feature = "sha3hash")]
    #[test]
    fn test_json_state() {
        use super::{skip_json_path, test_json_path};
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stArgsZeroOneBalance",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stPreCompiledContracts",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stSStoreTest",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stZeroKnowledge2",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stEIP158Specific",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stEWASMTests",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stEIP150singleCodeGasPrices",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stChangedEIP150",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stStaticCall",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stEIP150Specific",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stZeroCallsRevert",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stCreate2",
        );
        skip_json_path(
            "version above homestead",
            r"../jsondata/GeneralStateTests/stCallCreateCallCodeTest",
        );

        std::thread::Builder::new()
            .stack_size(134_217_728)
            .spawn(move || {
                test_json_path(r"../jsondata/GeneralStateTests/stRandom");
                test_json_path(r"../jsondata/GeneralStateTests/stSystemOperationsTest");
                test_json_path(r"../jsondata/GeneralStateTests/stRecursiveCreate");
                test_json_path(r"../jsondata/GeneralStateTests/stLogTests");
                test_json_path(r"../jsondata/GeneralStateTests/stCodeCopyTest");
                test_json_path(r"../jsondata/GeneralStateTests/stExtCodeHash");
                test_json_path(r"../jsondata/GeneralStateTests/stCallCodes");
                test_json_path(r"../jsondata/GeneralStateTests/stCreateTest");
                test_json_path(r"../jsondata/GeneralStateTests/stZeroKnowledge");
                test_json_path(r"../jsondata/GeneralStateTests/stRandom2");
                test_json_path(r"../jsondata/GeneralStateTests/stTransitionTest");
                test_json_path(r"../jsondata/GeneralStateTests/stZeroCallsTest");
                test_json_path(r"../jsondata/GeneralStateTests/stBugs");
                //test_json_path(r"../jsondata/GeneralStateTests/stBadOpcode");
                test_json_path(r"../jsondata/GeneralStateTests/stWalletTest");
                test_json_path(r"../jsondata/GeneralStateTests/stNonZeroCallsTest");
                test_json_path(r"../jsondata/GeneralStateTests/stCallDelegateCodesHomestead");
                test_json_path(r"../jsondata/GeneralStateTests/stAttackTest");
                test_json_path(r"../jsondata/GeneralStateTests/stStackTests");
                test_json_path(r"../jsondata/GeneralStateTests/stExample");
                test_json_path(r"../jsondata/GeneralStateTests/stSolidityTest");
                test_json_path(r"../jsondata/GeneralStateTests/stQuadraticComplexityTest");
                test_json_path(r"../jsondata/GeneralStateTests/stPreCompiledContracts2");
                test_json_path(r"../jsondata/GeneralStateTests/stInitCodeTest");
                test_json_path(r"../jsondata/GeneralStateTests/stDelegatecallTestHomestead");
                test_json_path(r"../jsondata/GeneralStateTests/stMemoryTest");
                test_json_path(r"../jsondata/GeneralStateTests/stSpecialTest");
                test_json_path(r"../jsondata/GeneralStateTests/stShift");
                test_json_path(r"../jsondata/GeneralStateTests/stHomesteadSpecific");
                test_json_path(r"../jsondata/GeneralStateTests/stCodeSizeLimit");
                test_json_path(r"../jsondata/GeneralStateTests/stReturnDataTest");
                test_json_path(r"../jsondata/GeneralStateTests/stTransactionTest");
                test_json_path(r"../jsondata/GeneralStateTests/stRevertTest");
                test_json_path(r"../jsondata/GeneralStateTests/stMemoryStressTest");
                test_json_path(
                    r"../jsondata/GeneralStateTests/stCallDelegateCodesCallCodeHomestead",
                );
                test_json_path(r"../jsondata/GeneralStateTests/stMemExpandingEIP150Calls");
                test_json_path(r"../jsondata/GeneralStateTests/stRefundTest");
            })
            .unwrap()
            .join()
            .unwrap();
    }
}
