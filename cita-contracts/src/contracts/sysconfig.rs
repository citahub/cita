use super::contract::Contract;
use cita_types::Address;

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
    economicalModel: EconomicalModel,
    tokenInfo: TokenInfo,
    chainIdV1: u64,
    autoExec: bool,
}

enum EconomicalModel {
    Quota,
    Charge,
}

struct TokenInfo {
    name: String,
    symbol: String,
    avatar: String,
}

impl Contract for Sysconfig {
    fn create() {}

    fn execute() {
        //  method_tools::extract_to_u32(&params.data[..]).and_then(|signature| match signature {
        //     0 => self.init(params, data_provider),
        //     // Register function
        //     0x832b_4580 => self.balance_get(params, data_provider),
        //     0xaa91_543e => self.update(params, data_provider),
        //     _ => Err(NativeError::Internal("out of gas".to_string())),
    }

    fn commit() {}
}

// impl Sysconfig {
//     fn init() {}

//     fn setOperator() {}
// }
