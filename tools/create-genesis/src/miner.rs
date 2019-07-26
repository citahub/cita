// use crate::common::{secret_2_address, string_2_bytes, string_2_u256};
use crate::genesis::Account;
// use core_executor::engines::NullEngine;
// use core_executor::executive::contract_address;
// use core_executor::libexecutor::sys_config::BlockSysConfig;
// use core_executor::types::transaction::Transaction;
// use evm::cita_types::U256;
// use evm::env_info::EnvInfo;
// use libproto::blockchain::Transaction as ProtoTransaction;

pub struct Miner;

impl Miner {
    pub fn mine(_code: Vec<u8>) -> Account {
        unimplemented!()
        // let mut state = get_temp_state();

        // // Create a transaction
        // let mut proto_tx = ProtoTransaction::new();
        // proto_tx.set_data(code);
        // proto_tx.set_value(string_2_bytes(String::from("0x00")));
        // proto_tx.set_nonce("0x00".to_string());
        // proto_tx.set_quota(string_2_u256(String::from("0x99999999999")).low_u64());

        // let private_key =
        //     String::from("0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6");
        // let tx = Transaction::create(&proto_tx).unwrap();
        // let sender = secret_2_address(&private_key);
        // let signed_transaction = tx.fake_sign(sender);

        // let env_info = EnvInfo::default();
        // let engine = NullEngine::cita();
        // let config = BlockSysConfig::default();

        // // Cal contract address
        // let contract_address = contract_address(&sender, &U256::from(0));
        // // Apply tx and commit to state
        // let _ = state.apply(&env_info, &engine, &signed_transaction, &config);
        // state.commit().unwrap();

        // // Get account content according to contract address
        // let account = state.account(&contract_address).unwrap().unwrap();
        // let code = account.code().unwrap();

        // Account {
        //     nonce: *account.nonce(),
        //     code: String::from("0x") + &hex::encode(code.to_vec()),
        //     storage: account.storage_cache(),
        //     value: *account.balance(),
        // }
    }
}
