use super::contract::Contract;
use cita_types::Address;
use std::collections::BTreeMap;

pub struct SysconfigMap {
    // key -> height, value: Sysconfig
    sysconfig: BTreeMap<u64, Sysconfig>,
}

impl Contract for Sysconfig {
    fn execute(
        &self,
        params: &VmExecParams,
        db: RocksDB,
        context: &Context,
    ) -> Result<InterpreterResult, NativeError> {
        let entry = Sysconfig::default();
        let result;
        // 先判断是读操作，还是写操作
        if params.read_only {
            // 读操作，小于查询高度的最近合约状态，根据 data 的前四个字节确定返回值。
            entry = load(old_height);
            method_tools::extract_to_u32(&params.data[..]).and_then(|signature| match signature {
                0x11115 => entry.getPermissionCheck(flag),
            });
        } else {
            // 写操作，查询 db 保存的最近合约状态，解析出要修改的内容，修改合约内容，更新 db。
            entry = load(latest_height);
            method_tools::extract_to_u32(&params.data[..]).and_then(|signature| match signature {
                0x0 => entry.init(params, flag),
                0x11111 => entry.setChainName(params, flag),
                0x11112 => entry.setOperator(params, flag),
                0x11113 => entry.setWebsite(params, flag),
                0x11114 => entry.setBlockInterval(params, flag),
                _ => Err(Error::Internal("out of gas".to_string())),
            });
            map.insert(params_height, entry);
            db.insert(DataCategory::Contracts, b"system-contract".to_vec(), map.to_vec(),);
        }

        let flag = false;
        let entry = Sysconfig::default();

        // load old Sysconfig
        if Some(map) = db.get(DataCategory::Contracts, b"system-contract".to_vec()) {
            let map = rlp::decode(map);
            // query history height
            latest_height = map[map.len()].key();
            if latest_height >= params_height {

            } else {
                // generate new state
                entry = map[map.len()];
                map.insert(params_height, entry);
            }
        }

        let result =
            method_tools::extract_to_u32(&params.data[..]).and_then(|signature| match signature {
                0x0 => entry.init(params, flag),
                0x11111 => entry.setChainName(params, flag),
                0x11112 => entry.setOperator(params, flag),
                0x11113 => entry.setWebsite(params, flag),
                0x11114 => entry.setBlockInterval(params, flag),
                0x11115 => entry.getPermissionCheck(flag),
                _ => Err(Error::Internal("out of gas".to_string())),
            });

        if result.is_ok() & flag = true {
            map.insert(params_height, entry);
            db.insert(
                DataCategory::Contracts,
                b"system-contract".to_vec(),
                map.to_vec(),
            );
        }
    }
}

impl Sysconfig {
    pub fn init(&mut self, params: VmExecParams) -> Result<InterpreterResult, NativeError> {
        // 巴拉巴拉把参数都解析出来，初始化
        Ok(InterpreterResult::Normal(vec![], 100, vec![]))
    }

    pub fn setChainName(&mut self, params: VmExecParams) -> Result<InterpreterResult, NativeError> {
        // let chain_name = params.data[4..36];
        self.chainName = chain_name;
        // db operation
        Ok(InterpreterResult::Normal(vec![], 100, vec![]))
    }

    pub fn getPermissionCheck(
        self,
        params: VmExecParams,
    ) -> Result<InterpreterResult, NativeError> {
        let res = self.getPermissionCheck.to_vec();
        Ok(InterpreterResult::Normal(res, 100, vec![]))
    }

    pub fn setOperator() {}

    pub fn setWebsite() {}

    pub fn setBlockInterval() {}

    pub fn getCreateContractPermissionCheck() {}

    pub fn getQuotaCheck() {}

    pub fn getFeeBackPlatformCheck() {}

    pub fn getChainOwner() {}
}

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

#[derive(Default)]
pub struct Sysconfig {
    delay_block_number: usize,
    check_permission: bool,
    checkSendTxPermission: bool,
    checkCreateContractPermission: bool,
    checkQuota: bool,
    checkFeeBackPlatform: bool,
    chainOwner: Address,
    chainName: String,
    chainId: usize,
    operator: String,
    website: String,
    blockInterval: usize,
    // economicalModel: EconomicalModel,
    tokenInfo: TokenInfo,
    chainIdV1: u64,
    autoExec: bool,
}

enum EconomicalModel {
    Quota,
    Charge,
}

#[derive(Default)]
struct TokenInfo {
    name: String,
    symbol: String,
    avatar: String,
}

pub struct Context {
    pub block_number: BlockNumber,
    pub coin_base: Address,
    pub timestamp: u64,
    pub difficulty: U256,
    pub last_hashes: Arc<LastHashes>,
    pub quota_used: U256,
    pub block_quota_limit: U256,
    pub account_quota_limit: U256,
}
