use crate::helper::{
    get_temp_state, secret_2_address, string_2_bytes, string_2_h256, string_2_u256,
};
use crate::json::state::Test;
use core_executor::engines::NullEngine;
use core_executor::libexecutor::economical_model::EconomicalModel;
use core_executor::libexecutor::sys_config::BlockSysConfig;
use core_executor::state::ApplyResult;
use core_executor::types::transaction::Transaction;
use evm::cita_types::U256;
use evm::env_info::EnvInfo;
use libproto::blockchain::Transaction as ProtoTransaction;
use std::fs;

pub fn test_json_file(p: &str) {
    let f = fs::File::open(p).unwrap();
    let tests = Test::load(f).unwrap();
    for (_name, test) in tests.into_iter() {
        let data_post_byzantium = test.post.unwrap().byzantium;
        if data_post_byzantium.is_none() {
            continue;
        }

        for (_i, postdata) in data_post_byzantium.unwrap().into_iter().enumerate() {
            // Init state
            let mut state = get_temp_state();
            for (address, account) in test.pre.clone().unwrap() {
                let balance = string_2_u256(account.balance);
                let code = string_2_bytes(account.code);
                let nonce = string_2_u256(account.nonce);
                if code.is_empty() {
                    state.new_contract(&address, balance, nonce);
                } else {
                    state.new_contract(&address, balance, nonce);
                    let _ = state.init_code(&address, code);
                }

                for (k, v) in account.storage {
                    let kk = string_2_h256(k);
                    let vv = string_2_h256(v);
                    let _ = state.set_storage(&address, kk, vv);
                }
            }
            state.commit().unwrap();

            // Set envionment
            let mut env_info = EnvInfo::default();
            env_info.difficulty = string_2_u256(test.env.current_difficulty.clone());
            env_info.number = string_2_u256(test.env.current_number.clone()).low_u64();
            env_info.timestamp = string_2_u256(test.env.current_timestamp.clone()).low_u64();
            env_info.gas_limit = string_2_u256(test.env.current_gas_limit.clone());
            env_info.author = test.env.current_coinbase;

            let engine = NullEngine::cita();
            let mut config = BlockSysConfig::default();
            config.quota_price = string_2_u256(test.transaction.gas_price.clone());
            config.economical_model = EconomicalModel::Charge;
            config.quota_price = U256::from(1);

            let idx_gas = &postdata.indexes[&String::from("gas")];
            let idx_value = &postdata.indexes[&String::from("value")];
            let idx_data = &postdata.indexes[&String::from("data")];
            let str_gas = test.transaction.gas_limit.clone()[*idx_gas].clone();
            let str_value = test.transaction.value.clone()[*idx_value].clone();
            let str_data = test.transaction.data.clone()[*idx_data].clone();

            let mut proto_tx = ProtoTransaction::new();
            proto_tx.set_data(string_2_bytes(str_data));
            proto_tx.set_value(string_2_bytes(str_value));
            proto_tx.set_nonce(test.transaction.nonce.clone());
            proto_tx.set_quota(string_2_u256(str_gas).low_u64());
            if !test.transaction.to.is_empty() {
                proto_tx.set_to(test.transaction.to.clone());
            }

            let tx = Transaction::create(&proto_tx).unwrap();
            let sender = secret_2_address(&test.transaction.secret_key);
            let signed_transaction = tx.fake_sign(sender);

            // Execute transactions
            let result: ApplyResult =
                state.apply(&env_info, &engine, &signed_transaction, true, &config);
            match result {
                Ok(outcome) => {
                    debug!("lalalal receipt error: {:?}", outcome.receipt.error);
                }
                _ => panic!("apply_transaction: There must be something wrong!"),
            }

            // check root hash
            state.commit().unwrap();
            let root = state.root();
            debug!("state.root {}", root);
            assert_eq!(*root, string_2_h256(postdata.hash));
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
    use super::skip_json_path;

    #[test]
    fn test_json_state() {
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stRefundTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stCodeCopyTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stMemExpandingEIP150Calls",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stCallDelegateCodesCallCodeHomestead",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stRevertTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stArgsZeroOneBalance",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stMemoryStressTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stTransactionTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stReturnDataTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stPreCompiledContracts",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stCodeSizeLimit",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stHomesteadSpecific",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stEIP158Specific",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stZeroKnowledge2",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stCreateTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stSStoreTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stTransitionTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stZeroCallsTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stBadOpcode",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stLogTests",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stNonZeroCallsTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stCallDelegateCodesHomestead",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stBugs",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stShift",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stWalletTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stRandom2",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stEWASMTests",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stStaticCall",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stAttackTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stStackTests",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stPreCompiledContracts2",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stInitCodeTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stZeroKnowledge",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stEIP150singleCodeGasPrices",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stChangedEIP150",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stExample",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stSolidityTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stEIP150Specific",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stQuadraticComplexityTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stSystemOperationsTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stZeroCallsRevert",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stRecursiveCreate",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stRandom",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stDelegatecallTestHomestead",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stMemoryTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stCreate2",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stSpecialTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stCallCodes",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stCallCreateCallCodeTest",
        );
        skip_json_path(
            "run tests integration",
            r"../jsondata/GeneralStateTests/stExtCodeHash",
        );
    }
}
