use cita_types::{Address, H256, U256};
use common_types::Bytes;

pub struct VmExecParams {
    pub origin: Address,
    pub storage_address: Address,
    /// Address of currently executed code.
    pub code_address: Address,
    pub code_data: Vec<u8>,
    /// Sender of current part of the transaction.
    pub sender: Address,
    /// Receive address. Usually equal to code_address,
    pub to_address: Address,
    /// Gas paid up front for transaction execution
    pub gas: u64,
    /// Gas price.
    pub gas_price: U256,
    /// Transaction value.
    pub value: U256,
    /// nonce
    pub nonce: U256,
    /// Input data.
    pub data: Bytes,
    pub read_only: bool,
    pub extra: H256,
    pub depth: u64,
    pub disable_transfer_value: bool,
}

// #[derive(Clone, Debug)]
// pub struct Log(pub Address, pub Vec<H256>, pub Vec<u8>);

// #[derive(Clone, Debug)]
// pub enum InterpreterResult {
//     // Return data, remain gas, logs.
//     Normal(Vec<u8>, u64, Vec<Log>),
//     // Return data, remain gas
//     Revert(Vec<u8>, u64),
//     // Return data, remain gas, logs, contract address
//     Create(Vec<u8>, u64, Vec<Log>, Address),
// }
