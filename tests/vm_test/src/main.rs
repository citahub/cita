mod json;

use evm::action_params::{ActionParams, ActionValue};
use evm::env_info::EnvInfo;
use evm::factory::{Factory, VMType};
use evm::fake_tests::FakeExt;
use evm::return_data::GasLeft;
use evm::Ext;
use std::fs;
use std::str;
use std::sync::Arc;
use test_helper::{string_2_bytes, string_2_h256, string_2_u256};

fn test_json_file(p: &str) {
    println!("{}", p);
    let f = fs::File::open(p).unwrap();
    let tests = json::Test::load(f).unwrap();
    for (_name, vm) in tests.into_iter() {
        // Step one: Init params, env_info
        let mut params = ActionParams::default();
        params.address = vm.exec.address;
        params.sender = vm.exec.caller;
        params.code = Some(Arc::new(string_2_bytes(vm.exec.code)));
        params.data = Some(string_2_bytes(vm.exec.data));
        params.gas = string_2_u256(vm.exec.gas);
        params.gas_price = string_2_u256(vm.exec.gas_price);
        params.origin = vm.exec.origin;
        params.value = ActionValue::Apparent(string_2_u256(vm.exec.value));

        let mut env_info = EnvInfo::default();
        env_info.difficulty = string_2_u256(vm.env.current_difficulty);
        env_info.number = string_2_u256(vm.env.current_number).low_u64();
        env_info.timestamp = string_2_u256(vm.env.current_timestamp).low_u64();
        env_info.gas_limit = string_2_u256(vm.env.current_gas_limit);
        env_info.author = vm.env.current_coinbase;

        // Step two: Vm exec process
        let factory = Factory::new(VMType::Interpreter, 1024 * 32);
        let mut evm = factory.create(params.gas);
        let mut ext = FakeExt::new();
        ext.info = env_info;

        if let Some(pre) = vm.pre {
            for (_address, account) in pre.into_iter() {
                for (k, v) in account.storage {
                    ext.set_storage(string_2_h256(k), string_2_h256(v))
                        .expect("Init pre state failed.");
                }
            }
        }

        match evm.exec(&params, &mut ext) {
            Ok(GasLeft::Known(gas_left)) => assert_eq!(gas_left, string_2_u256(vm.gas.unwrap())),
            Ok(GasLeft::NeedsReturn { gas_left, data, .. }) => {
                assert_eq!(gas_left, string_2_u256(vm.gas.unwrap()));
                assert_eq!((*data).to_vec(), string_2_bytes(vm.out.unwrap()));
            }
            Err(_) => assert!(vm.gas.is_none() && vm.post.is_none() && vm.logs.is_none()),
        }

        if let Some(post) = vm.post {
            for (_address, account) in post.into_iter() {
                for (k, v) in account.storage {
                    if let Ok(value) = ext.storage_at(&string_2_h256(k)) {
                        assert_eq!(value, string_2_h256(v));
                    }
                }
            }
        }
    }
}

fn test_json_path(p: &str) {
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

fn main() {
    test_json_path(r"./tests/jsondata/VMTests/vmArithmeticTest");
    test_json_path(r"./tests/jsondata/VMTests/vmBitwiseLogicOperation");
    test_json_path(r"./tests/jsondata/VMTests/vmBlockInfoTest");
    test_json_path(r"./tests/jsondata/VMTests/vmEnvironmentalInfo");
    test_json_path(r"./tests/jsondata/VMTests/vmIOandFlowOperations");
    test_json_path(r"./tests/jsondata/VMTests/vmLogTest");
    test_json_path(r"./tests/jsondata/VMTests/vmRandomTest");
    test_json_path(r"./tests/jsondata/VMTests/vmSha3Test");
    test_json_path(r"./tests/jsondata/VMTests/vmPushDupSwapTest");
    test_json_path(r"./tests/jsondata/VMTests/vmSystemOperations");
    test_json_path(r"./tests/jsondata/VMTests/vmTests");
}
