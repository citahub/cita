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
use std::sync::Arc;

pub fn test_json_file(p: &str) {
    let f = fs::File::open(p).unwrap();
    let tests = Test::load(f).unwrap();
    for (_name, test) in tests.into_iter() {
        let data_post_homestead = test.post.unwrap().homestead;
        if data_post_homestead.is_none() {
            continue;
        }

        for (_i, postdata) in data_post_homestead.unwrap().into_iter().enumerate() {
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
            let previous_hash = string_2_h256(test.env.previous_hash.clone());
            Arc::make_mut(&mut env_info.last_hashes).push(previous_hash);

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
                state.apply(&env_info, &engine, &signed_transaction, &config);
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
        test_json_path(r"../jsondata/GeneralStateTests/stBadOpcode");
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
        test_json_path(r"../jsondata/GeneralStateTests/stCallDelegateCodesCallCodeHomestead");
        test_json_path(r"../jsondata/GeneralStateTests/stMemExpandingEIP150Calls");
        test_json_path(r"../jsondata/GeneralStateTests/stRefundTest");
    }
}
